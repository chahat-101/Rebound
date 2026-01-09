const MAX_VEL_X: f32 = 300.0;
const MAX_VEL_Y: f32 = 400.0;
use crate::utils::G;
use macroquad::prelude::*;

pub const BULLET_LENGTH: f32 = 50.0;

pub struct Bullet {
    pub centre: Vec2,
    pub velocity: Vec2,
    pub alive: bool,
    texture: Texture2D,
}

pub struct Player {
    pub position: Vec2,
    pub velocity: Vec2,
    pub texture: Texture2D,
    pub bullets: Vec<Bullet>,
    pub fire_cooldown: f32,
}

impl Player {
    pub fn new(position: Vec2, texture: Texture2D) -> Self {
        Self {
            position,
            velocity: Vec2::ZERO,
            texture,
            bullets: Vec::new(),
            fire_cooldown: 0.0,
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.velocity += G * dt;
        self.position += self.velocity * dt;

        self.velocity.x = self.velocity.x.clamp(-MAX_VEL_X, MAX_VEL_X);
        self.velocity.y = self.velocity.y.clamp(-MAX_VEL_Y, MAX_VEL_Y);

        self.bullets.retain_mut(|bullet| {
            bullet.update(dt);
            bullet.alive
        });
    }

    pub fn draw(&self) {
        draw_texture_ex(
            &self.texture,
            self.position.x,
            self.position.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(50.0, 50.0)),
                pivot: Some(vec2(
                    self.texture.width() / 2.0,
                    self.texture.height() / 2.0,
                )),
                flip_x: true,
                ..Default::default()
            },
        );

        for bullet in self.bullets.iter() {
            bullet.draw();
        }
    }

    pub fn fire_bullet(&mut self, bullet_texture: Texture2D) {
        let bullet = Bullet::new(self.position + BULLET_LENGTH / 2.0, bullet_texture);

        self.bullets.push(bullet);
    }
}

impl Bullet {
    pub fn new(centre: Vec2, texture: Texture2D) -> Self {
        Self {
            centre,
            velocity: vec2(200.0, 0.0),
            alive: true,
            texture,
        }
    }

    pub fn draw(&self) {
        draw_texture_ex(
            &self.texture,
            self.centre.x,
            self.centre.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(BULLET_LENGTH, 70.0)),
                ..Default::default()
            },
        );
    }

    pub fn update(&mut self, dt: f32) {
        self.centre.x += self.velocity.x * dt;

        if self.centre.x + BULLET_LENGTH / 2.0 < 0.0
            || self.centre.x > screen_width()
            || self.centre.y < 0.0
            || self.centre.y > screen_height()
        {
            self.alive = false;
        }
    }
}
