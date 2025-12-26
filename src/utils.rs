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

pub fn ball_rect_collision(ball: &Ball, wall: &Wall, dt: f32) -> bool {
    let ball_centre = vec![ball.position.x, ball.position.y];

    let wall_centre = vec![
        wall.rect.x + wall.rect.w / 2.0,
        wall.rect.y + wall.rect.h / 2.0,
    ];

    let wall_x_range = vec![
        wall_centre[0] - wall.rect.w / 2.0,
        wall_centre[0] + wall.rect.w / 2.0,
    ];

    let wall_y_range = vec![
        wall_centre[1] - wall.rect.h / 2.0,
        wall_centre[1] + wall.rect.h / 2.0,
    ];

    let ball_velocity = ball.velocity;

    let ball_slope = ball_velocity.x / ball_velocity.y;
    let ball_intrcpt = get_y_intercept(ball_slope, ball.position.x, ball.position.y);

    let wall_slope = wall.angle.tan();
    let wall_intrcpt = get_y_intercept(wall_slope, wall.rect.x, wall.rect.y);

    let wall_unit_vector = normal_unit_vector(wall_slope, -1.0);

    if wall_slope != ball_slope {
        if dot_product(vec![ball_velocity.x, ball_velocity.y], wall_unit_vector) < 0.0 {
            let intersection_point =
                get_intersection_point(ball_slope, ball_intrcpt, wall_slope, wall_intrcpt);

            let current_distance = get_distace(
                ball.position.x,
                ball.position.y,
                intersection_point[0],
                intersection_point[1],
            );

            let frame_dist = get_distace(0.0, 0.0, (ball_velocity * dt).x, (ball_velocity * dt).y);
            if frame_dist > current_distance {
                return true;
            }
        }
    }
    false
}

fn get_y_intercept(slope: f32, x: f32, y: f32) -> f32 {
    y - slope * x
}

fn normal_unit_vector(a: f32, b: f32) -> Vec<f32> {
    let denominator = (a * a + b * b).powf(0.5);

    vec![a / denominator, b / denominator]
}

fn dot_product(a: Vec<f32>, b: Vec<f32>) -> f32 {
    let dot: f32 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();

    dot
}

fn get_intersection_point(slope1: f32, intercept1: f32, slope2: f32, intercept2: f32) -> Vec<f32> {
    let x = (intercept2 - intercept1) / (slope1 - slope2);
    let y = slope1 * x + intercept1;

    vec![x, y]
}

fn get_distace(x1: f32, y1: f32, x2: f32, y2: f32) -> f32 {
    let x_diff = x2 - x1;
    let y_dist = y2 - y1;

    let dist = x_diff.powi(2) + y_dist.powi(2);

    dist.sqrt()
}
