use macroquad::miniquad::gl::ERROR_INVALID_VERSION_ARB;
use macroquad::prelude::*;
use macroquad::{color::Color, math::Vec2};
use std::{clone, collections::HashMap};

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
    let ball_centre = vec![ball.position.x, ball.position.y];
    let ball_radius = ball.radius;

    let closest_x = ball_centre[0].clamp(wall.rect.x, wall.rect.x + wall.rect.w);
    let closest_y = ball_centre[1].clamp(wall.rect.y, wall.rect.y + wall.rect.h);

    let dx = ball_centre[0] - closest_x;
    let dy = ball_centre[1] - closest_y;

    let dist_squared = dx * dx + dy * dy;

    let radius_squared = ball_radius * ball_radius;

    if dist_squared < radius_squared {
        let dist = dist_squared.sqrt();

        // Calculate penetration depth
        let penetration_depth = ball_radius - dist;

        // Calculate normalized collision normal
        let normal_x = if dist == 0.0 { 1.0 } else { dx / dist }; // Avoid division by zero
        let normal_y = if dist == 0.0 { 0.0 } else { dy / dist }; // Avoid division by zero

        // Positional correction: move ball out of the wall
        ball.position.x += normal_x * penetration_depth;
        ball.position.y += normal_y * penetration_depth;

        // Velocity reflection
        let vel_dot_n = ball.velocity.x * normal_x + ball.velocity.y * normal_y;

        if vel_dot_n < 0.0 {
            ball.velocity.x -= (2.0) * vel_dot_n * normal_x;
            ball.velocity.y -= (2.0) * vel_dot_n * normal_y;
        }
    }
}
