use macroquad::prelude::*;

use crate::{
    game::Game,
    utils::{Entity, HasBounds},
    walls::Wall,
};

pub struct RotatingRoom {
    pub centre: Vec2,
    pub radius: f32,
    pub angular_speed: f32,
    pub angle: f32,
    pub n: usize,
    pub wall_indices: Vec<usize>,
}

impl RotatingRoom {
    pub fn new(
        game: &mut Game,
        centre: Vec2,
        radius: f32,
        n: usize,
        wall_length: f32,
        wall_thickness: f32,
        angular_speed: f32,
    ) -> Self {
        let mut wall_indices = Vec::with_capacity(n);

        for i in 0..n {
            let base_angle = i as f32 * std::f32::consts::TAU / n as f32;

            let pos = centre + vec2(base_angle.cos(), base_angle.sin()) * radius;

            let wall = Wall::new(
                pos.x - wall_length / 2.0,
                pos.y - wall_thickness / 2.0,
                wall_length,
                wall_thickness,
                base_angle + std::f32::consts::FRAC_PI_2,
                None,
            );

            game.walls.push(wall);
            let idx = game.walls.len() - 1;

            let bounds = game.walls[idx].bounds();

            wall_indices.push(idx);
        }

        Self {
            centre,
            radius,
            angular_speed,
            angle: 0.0,
            n,
            wall_indices,
        }
    }
    pub fn update(&mut self, game: &mut Game, dt: f32) {
        self.angle += self.angular_speed * dt;

        self.angle = self.angle.rem_euclid(std::f32::consts::TAU);

        for (i, &wall_idx) in self.wall_indices.iter().enumerate() {
            if wall_idx >= game.walls.len() {
                continue;
            }

            let base_angle = i as f32 * std::f32::consts::TAU / self.n as f32;
            let angle = base_angle + self.angle;

            let wall = &mut game.walls[wall_idx];

            let old_bounds = wall.bounds();

            let pos = self.centre + vec2(angle.cos(), angle.sin()) * self.radius;

            wall.rect.x = pos.x - wall.rect.w / 2.0;
            wall.rect.y = pos.y - wall.rect.h / 2.0;
            wall.angle = angle + std::f32::consts::FRAC_PI_2;

            let new_bounds = wall.bounds();

            game.grid
                .update_bounds(old_bounds, new_bounds, Entity::Wall(wall_idx));
        }
    }
}
