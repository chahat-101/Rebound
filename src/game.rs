use crate::balls::Ball;
use crate::player::{Player,BULLET_LENGTH};
use crate::utils::{Entity, HasBounds, SpatialGrid, ball_rect_collision};
use crate::walls::Wall;
use macroquad::prelude::*;



pub struct Game {
    pub walls: Vec<Wall>,
    pub balls: Vec<Ball>,
    pub player: Player,
    pub grid: SpatialGrid,
    pub bullet_texture: Texture2D
}

impl Game {
    pub fn new(cell_size: f32, position: Vec2, texture:Texture2D,bullet_texture: Texture2D) -> Self {
        Self {
            walls: Vec::new(),
            balls: Vec::new(),
            grid: SpatialGrid::new(cell_size),
            player: Player::new(position, texture),
            bullet_texture,
        }
    }

    pub fn update(&mut self, dt: f32) {
        // move balls and update spatial grid
        for i in 0..self.balls.len() {
            let ball = &mut self.balls[i];
            let old_bounds = ball.bounds();
            ball.update(dt);
            let new_bounds = ball.bounds();

            self.grid
                .update_bounds(old_bounds, new_bounds, Entity::Ball(i));
        }

        // check collisions
        for i in 0..self.balls.len() {
            let ball_pos = self.balls[i].position;
            let ball_radius = self.balls[i].radius;

            let near_entities = self.grid.query_result(ball_pos, ball_radius);

            for entity in near_entities {
                if let Entity::Wall(wall_idx) = entity {
                    if wall_idx < self.walls.len() {
                        let wall = &self.walls[wall_idx];
                        let ball = &mut self.balls[i];
                        ball_rect_collision(ball, wall);
                    }
                }
            }
        }
        
        let old_player_pos = self.player.position;
        self.player.update(dt);
        let player_rect = Rect::new(self.player.position.x - 25.0, self.player.position.y - 25.0, 50.0, 50.0);

        for wall in self.walls.iter(){
            if player_rect.intersect(wall.rect).is_some(){
                self.player.position = old_player_pos;
            }
            for bullet in self.player.bullets.iter_mut(){
                let bullet_rect = Rect::new(bullet.centre.x, bullet.centre.y, BULLET_LENGTH, 5.0);
                    if bullet_rect.intersect(wall.rect).is_some(){
                        bullet.alive = false;
                    }
            }
        }
        
    
        
    }

    pub fn draw(&self) {
        for ball in self.balls.iter() {
            ball.draw();
        }

        for wall in self.walls.iter() {
            wall.draw();
        }
        self.player.draw();
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
        let wall_index = self.walls.len() - 1;
        let bounds = wall.bounds();

        self.grid.insert_bounds(bounds, Entity::Wall(wall_index));
    }

    pub fn spawn_ball(&mut self, centre: (f32, f32), velocity: Vec2, radius: f32, gravity: bool) {
        let ball = Ball::new(vec2(centre.0, centre.1), velocity, radius, gravity);

        let bounds = ball.bounds();
        self.balls.push(ball);
        let index = self.balls.len() - 1;

        self.grid.insert_bounds(bounds, Entity::Ball(index));
    }
}
