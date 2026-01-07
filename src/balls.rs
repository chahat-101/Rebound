use crate::utils::{G, HasBounds};

use macroquad::prelude::*;
pub struct Ball {
    pub position: Vec2,
    pub velocity: Vec2,
    pub radius: f32,
    pub gravity: bool,
}

impl Ball {
    pub fn update(&mut self, dt: f32) {
        if self.gravity {
            self.velocity += G * dt;
        }
        self.position += self.velocity * dt;
    }
    pub fn draw(&self) {
        draw_circle(self.position.x, self.position.y, self.radius, WHITE);
    }

    pub fn new(position: Vec2, velocity: Vec2, radius: f32, gravity: bool) -> Self {
        Self {
            position,
            velocity,
            radius,
            gravity,
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
