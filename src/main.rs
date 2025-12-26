use macroquad::prelude::*;

mod balls;
mod utils;
mod walls; // or whatever you named the module with collision code

use balls::Ball;
use utils::SpatialGrid;
use walls::Wall;

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
        velocity: Option::None,
    };

    loop {
        let dt = get_frame_time();
        clear_background(BLACK);
        if !ball_rect_collision(&ball, &wall, dt) {
            draw_circle(ball.position.x, ball.position.y, ball.radius, ORANGE);
            ball.update(dt);
            draw_rectangle(wall.rect.x, wall.rect.y, wall.rect.w, wall.rect.h, WHITE);
        }
        println!("{},{}", ball.position.x, ball.position.y);
        next_frame().await;
    }
}
