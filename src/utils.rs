use std::collections::HashMap;

use macroquad::{color::Color, math::Vec2, telemetry::frame};

use crate::{balls::Ball, walls::Wall};

pub trait GameObject {
    fn draw(&self, color: Color);
    fn update_and_draw(&mut self, accelaration: Vec2, dt: f32);
}

pub struct SpatialGrid {
    cell_size: f32,
    cells: HashMap<(i32, i32), Vec<usize>>,
}

impl SpatialGrid {
    fn new(cell_size: f32) -> Self {
        Self {
            cell_size,
            cells: HashMap::new(),
        }
    }

    fn world_to_cell(&self, x: f32, y: f32) -> (i32, i32) {
        ((x / self.cell_size) as i32, (y / self.cell_size) as i32)
    }

    fn insert_wall(&mut self, wall_index: usize, wall: &Wall) {
        let min_cell = self.world_to_cell(wall.rect.x, wall.rect.y);
        let max_cell = self.world_to_cell(wall.rect.x + wall.rect.w, wall.rect.y + wall.rect.h);

        for cx in min_cell.0..=min_cell.0 {
            for cy in min_cell.1..=min_cell.1 {
                self.cells
                    .entry((cx, cy))
                    .or_insert_with(Vec::new)
                    .push(wall_index);
            }
        }
    }

    fn get_cell(&self, x: f32, y: f32) -> (i32, i32) {
        ((x / self.cell_size) as i32, (y / self.cell_size) as i32)
    }

    fn query(&self, ball: &Ball) -> Vec<usize> {
        let cell = self.get_cell(ball.position.x, ball.position.y);
        self.cells.get(&cell).cloned().unwrap_or_default()
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
