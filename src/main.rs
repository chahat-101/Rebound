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
    let mut game = Game::new(10.0);
    let thickness = 10.0;
    game.spawn_wall(0.0, 0.0, screen_width(), thickness, 0.0, None); // top
    game.spawn_wall(
        0.0,
        screen_height() - thickness,
        screen_width(),
        thickness,
        0.0,
        None,
    ); // bottom
    game.spawn_wall(0.0, 0.0, thickness, screen_height(), 0.0, None); // left
    game.spawn_wall(
        screen_width() - thickness,
        0.0,
        thickness,
        screen_height(),
        0.0,
        None,
    );

    for i in 0..1 {
        let x = 100.0 + i as f32 * 50.0;
        let y = 100.0;
        game.spawn_ball((x, y), vec2(0.0, 100.0), 20.0);
    }
    let acc = vec2(0.0, 130.0);
    loop {
        clear_background(BLACK);

        let dt = get_frame_time();
        for ball in game.balls.iter_mut() {
            ball.velocity += acc * dt;
        }
        game.update(dt);
        game.draw();

        next_frame().await;
    }
}
