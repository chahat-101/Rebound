use macroquad::prelude::*;

mod balls;
mod utils;
mod walls; // or whatever you named the module with collision code

use balls::Ball;
use utils::SpatialGrid;
use walls::Wall;

mod game;
use game::Game;

use crate::utils::ball_rect_collision;

#[macroquad::main("a")]
async fn main() {
    let x = screen_width();
    let mut ball = Ball {
        position: vec2(x / 2.0, 0.0),
        velocity: vec2(0.0, 100.0),
        radius: 10.0,
    };
    let mut wall = Wall {
        rect: Rect {
            x: 100.0,
            y: 300.0,
            w: 350.0,
            h: 150.0,
        },
        angle: 0.0,
        velocity: Some(vec2(10.0, 200.0)),
    };

    loop {
        let dt = get_frame_time();
        clear_background(BLACK);
        ball.draw();
        ball.update(dt);
        ball_rect_collision(&mut ball, &wall);
        wall.update(vec2(-10.0, -50.0), dt);
        wall.draw();
        next_frame().await;
    }
}
