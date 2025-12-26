use std::collections::HashMap;

use macroquad::{color::Color, math::Vec2};

use crate::{balls::Ball, walls::Wall};

pub trait GameObject {
    fn draw(&self, color: Color);
    fn update_and_draw(&mut self, accelaration: Vec2, dt: f32);
}

pub struct SpatialGrid {
    pub cell_size: f32,
    pub cells: HashMap<(i32, i32), Vec<usize>>,
}

pub struct Game {
    walls: Vec<Wall>,
    balls: Vec<Ball>,
    grid: SpatialGrid,
}

impl SpatialGrid {
    pub fn new(cell_size: f32) -> Self {
        Self {
            cell_size: cell_size,
            cells: HashMap::new(),
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

pub fn get_cell(x: f32, y: f32, spatial_grid: &SpatialGrid) -> (i32, i32) {
    let cell_size = spatial_grid.cell_size;

    ((x / cell_size) as i32, (y / cell_size) as i32)
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
