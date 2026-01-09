use macroquad::prelude::*;

use crate::player::Bullet;

use crate::utils;
use utils::random_spawn_position;

pub struct Enemy {
    pub position: Vec2,
    pub velocity: Vec2,
    pub texture: Texture2D,
    pub target: Vec2,
    alive: bool,
}

impl Enemy {
    pub fn new(position: Vec2, texture: Texture2D, target: Vec2) -> Self {
        let velocity = vec2(20.0, 0.0);
        Self {
            position,
            velocity,
            texture,
            target,
            alive: true,
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.position += self.velocity * dt;
    }

    pub fn draw(&self) {
        draw_texture_ex(
            &self.texture,
            self.position.x,
            self.position.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(30.0, 40.0)), // keep original texture size
                rotation: 0.0,
                flip_x: false,
                flip_y: false,
                pivot: None,
                source: None,
            },
        );
    }
}
