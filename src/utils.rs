use std::{clone, collections::HashMap};

use macroquad::{color::Color, math::Vec2};

use crate::{balls::Ball, walls::Wall};

pub trait GameObject {
    fn draw(&self, color: Color);
    fn update_and_draw(&mut self, accelaration: Vec2, dt: f32);
}

pub struct SpatialGrid {
    pub cell_size: f32,
    pub cells: HashMap<(i32, i32), Vec<Entity>>,
}

pub struct Game {
    walls: Vec<Wall>,
    balls: Vec<Ball>,
    grid: SpatialGrid,
}
#[derive(Clone, Copy, PartialEq)]
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
    pub fn get_cell_key(&self, position: Vec2) -> (i32, i32) {
        let cell_size = self.cell_size;
        (
            (position.x / cell_size) as i32,
            (position.y / cell_size) as i32,
        )
    }

    pub fn insert_entity(&mut self, position: Vec2, entity: Entity) {
        let key = self.get_cell_key(position);
    }

    pub fn remove(&self, position: Vec2, entity: Entity) {}

    pub fn update(&mut self, old_position: Vec2, new_position: Vec2, entity: Entity) {
        let old_key = self.get_cell_key(old_position);
        let new_key = self.get_cell_key(new_position);

        if new_key != old_key {
            if let Some(cell) = self.cells.get_mut(&old_key) {
                self.remove(old_position, entity);
                self.insert_entity(new_position, entity);
            }
        }
    }

    pub fn clear(&mut self){
        self.cells.clear();
    }

    pub fn query_result(&self,position: Vec2,radius:f32) -> Vec<Entity> { // this result all the objects in the specified radius
        let mut result = Vec::new();
        let cells_to_check = (radius/self.cell_size).ceil();
        let centre = self.get_cell_key(position);
        for dx in -cells_to_check..=cells_to_check{
            for dy in -cells_to_check..=cells_to_check{
                let key = (centre[0]+dx,centre[1]+dy);
                if let Some(cell) = self.cells.get(&key){
                    result.extend(cell);
                }
            }
        }
}

impl Game {
    pub fn new(cell_size: f32) -> Self {
        Self {
            walls: Vec::new(),
            balls: Vec::new(),
            grid: SpatialGrid::new(cell_size),
        }
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
