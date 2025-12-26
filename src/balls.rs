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
}
