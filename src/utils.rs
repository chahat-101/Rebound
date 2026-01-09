use macroquad::miniquad::gl::ERROR_INVALID_VERSION_ARB;
use macroquad::prelude::*;
use macroquad::{color::Color, math::Vec2};
use std::{clone, collections::HashMap};
pub const G: Vec2 = vec2(0.0, 500.0);
use crate::{balls::Ball, walls::Wall};

use crate::game::Game;

pub trait HasBounds {
    fn bounds(&self) -> Rect;
}

pub trait GameObject {
    fn draw(&self, color: Color);
    fn update_and_draw(&mut self, accelaration: Vec2, dt: f32);
}

pub struct SpatialGrid {
    pub cell_size: f32,
    pub cells: HashMap<(i32, i32), Vec<Entity>>,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Entity {
    Wall(usize),
    Ball(usize),
}

impl SpatialGrid {
    pub fn new(cell_size: f32) -> Self {
        Self {
            cell_size: cell_size,
            cells: HashMap::new(),
        }
    }

    pub fn insert_bounds(&mut self, bounds: Rect, entity: Entity) {
        let min = self.get_cell_key(vec2(bounds.x, bounds.y));
        let max = self.get_cell_key(vec2(bounds.x + bounds.w, bounds.y + bounds.h));

        for x in min.0..=max.0 {
            for y in min.1..=max.1 {
                self.cells.entry((x, y)).or_default().push(entity);
            }
        }
    }

    pub fn get_cell_key(&self, position: Vec2) -> (i32, i32) {
        let cell_size = self.cell_size;
        (
            (position.x / self.cell_size).floor() as i32,
            (position.y / self.cell_size).floor() as i32,
        )
    }

    pub fn remove_bounds(&mut self, bounds: Rect, entity: Entity) {
        let min = self.get_cell_key(vec2(bounds.x, bounds.y));
        let max = self.get_cell_key(vec2(bounds.x + bounds.w, bounds.y + bounds.h));

        for x in min.0..=max.0 {
            for y in min.1..=max.1 {
                if let Some(cell) = self.cells.get_mut(&(x, y)) {
                    cell.retain(|e| *e != entity);
                }
            }
        }
    }
    pub fn update_bounds(&mut self, old_bounds: Rect, new_bounds: Rect, entity: Entity) {
        let old_min = self.get_cell_key(vec2(old_bounds.x, old_bounds.y));
        let old_max = self.get_cell_key(vec2(
            old_bounds.x + old_bounds.w,
            old_bounds.y + old_bounds.h,
        ));

        let new_min = self.get_cell_key(vec2(new_bounds.x, new_bounds.y));
        let new_max = self.get_cell_key(vec2(
            new_bounds.x + new_bounds.w,
            new_bounds.y + new_bounds.h,
        ));

        if old_min == new_min && old_max == new_max {
            return;
        }

        for x in old_min.0..=old_max.0 {
            for y in old_min.1..=old_max.1 {
                if let Some(cell) = self.cells.get_mut(&(x, y)) {
                    cell.retain(|e| e != &entity);
                }
            }
        }

        self.insert_bounds(new_bounds, entity);
    }

    pub fn clear(&mut self) {
        self.cells.clear();
    }

    pub fn query_result(&self, position: Vec2, radius: f32) -> Vec<Entity> {
        // this result all the objects in the specified radius
        let mut result = Vec::new();

        let cells_to_check = (radius / self.cell_size).ceil() as i32;
        let centre = self.get_cell_key(position);

        for dx in -cells_to_check..=cells_to_check {
            for dy in -cells_to_check..=cells_to_check {
                let key = (centre.0 + dx, centre.1 + dy);
                if let Some(cell) = self.cells.get(&key) {
                    result.extend(cell.iter().copied());
                }
            }
        }

        result.sort_unstable();
        result.dedup();

        result
    }
}

pub fn ball_rect_collision(ball: &mut Ball, wall: &Wall) {
    let angle = wall.angle;
    let rect_center = wall.rect.point() + wall.rect.size() / 2.0;

    let translated_ball_center = ball.position - rect_center;
    let rotated_ball_center = vec2(
        translated_ball_center.x * angle.cos() + translated_ball_center.y * angle.sin(),
        -translated_ball_center.x * angle.sin() + translated_ball_center.y * angle.cos(),
    );

    let half_extents = wall.rect.size() / 2.0;
    let closest_point_local = rotated_ball_center.clamp(-half_extents, half_extents);

    let normal_local;
    if (rotated_ball_center.x).abs() > half_extents.x
        || (rotated_ball_center.y).abs() > half_extents.y
    {
        normal_local = (rotated_ball_center - closest_point_local).normalize();
    } else {
        let dx = closest_point_local.x - rotated_ball_center.x;
        let dy = closest_point_local.y - rotated_ball_center.y;

        if dx.abs() > dy.abs() {
            normal_local = vec2(dx.signum(), 0.0);
        } else {
            normal_local = vec2(0.0, dy.signum());
        }
    }

    let distance_sq = rotated_ball_center.distance_squared(closest_point_local);

    if distance_sq < ball.radius * ball.radius {
        let distance = distance_sq.sqrt();
        let penetration_depth = ball.radius - distance;
        let normal_world = vec2(
            normal_local.x * angle.cos() - normal_local.y * angle.sin(),
            normal_local.x * angle.sin() + normal_local.y * angle.cos(),
        );

        ball.position += normal_world * penetration_depth;

        let restitution = 1.0;
        let closing_vel = ball.velocity;
        let vel_along_normal = closing_vel.dot(normal_world);

        if vel_along_normal < 0.0 {
            let j = -(1.0 + restitution) * vel_along_normal;
            let impulse = j * normal_world;
            ball.velocity += impulse;
        }
    }
}

pub fn random_spawn_position(player_pos: Vec2) -> Vec2 {
    let padding = 30.0;
    let min_distance = 100.0;

    loop {
        let edge = rand::gen_range(0, 4);

        let pos = match edge {
            // Left
            0 => vec2(-padding, rand::gen_range(0., screen_height())),
            // Right
            1 => vec2(
                screen_width() + padding,
                rand::gen_range(0., screen_height()),
            ),
            // Top
            2 => vec2(rand::gen_range(0., screen_width()), -padding),
            // Bottom
            _ => vec2(
                rand::gen_range(0., screen_width()),
                screen_height() + padding,
            ),
        };

        // Safety check
        if pos.distance(player_pos) > min_distance {
            return pos;
        }
    }
}
