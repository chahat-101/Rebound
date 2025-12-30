use crate::balls::Ball;
use crate::utils::{Entity, SpatialGrid};
use crate::walls::Wall;
use macroquad::prelude::*;

pub struct Game {
    pub walls: Vec<Wall>,
    pub balls: Vec<Ball>,
    pub grid: SpatialGrid,
}

impl Game {
    pub fn new(cell_size: f32) -> Self {
        Self {
            walls: Vec::new(),
            balls: Vec::new(),
            grid: SpatialGrid::new(cell_size),
        }
    }

    pub fn update(&mut self, dt: f32) {
        for (i, ball) in self.balls.iter_mut().enumerate() {
            let old_pos = ball.position;
            ball.update(dt);
            let new_pos = ball.position;

            self.grid.update(old_pos, new_pos, Entity::Ball(i));
        }
    }

    pub fn draw(&self) {
        for ball in self.balls.iter() {
            ball.draw();
        }

        for wall in self.walls.iter() {
            wall.draw();
        }
    }

    pub fn spawn_wall(
        &mut self,
        x: f32,
        y: f32,
        w: f32,
        h: f32,
        angle: f32,
        velocity: Option<Vec2>,
    ) {
        let wall = Wall::new(x, y, w, h, angle, velocity);

        self.walls.push(wall);
        let index = self.walls.len();

        self.grid.insert_entity(position, entity);
    }
}
