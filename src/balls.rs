use std::backtrace;

use crate::utils::HasBounds;
use macroquad::prelude::*;

pub struct Ball {
    pub position: Vec2,
    pub velocity: Vec2,
    pub radius: f32,
}

impl Ball {
    pub fn update(&mut self, dt: f32) {
        self.position += self.velocity * dt;
    }
    pub fn draw(&self) {
        draw_circle(self.position.x, self.position.y, self.radius, WHITE);
    }

    pub fn new(position: Vec2, velocity: Vec2, radius: f32) -> Self {
        Self {
            position: position,
            velocity: velocity,
            radius: radius,
        }
    }
}

impl HasBounds for Ball {
    fn bounds(&self) -> Rect {
        Rect::new(
            self.position.x - self.radius,
            self.position.y - self.radius,
            self.radius * 2.0,
            self.radius * 2.0,
        )
    }
}
