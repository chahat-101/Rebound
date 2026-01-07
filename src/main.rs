use macroquad::prelude::*;

mod balls;
mod utils;
mod walls; // or whatever you named the module with collision code

use balls::Ball;
use utils::SpatialGrid;
use walls::Wall;

mod game;
use game::Game;

mod rotatingroom;

use crate::utils::ball_rect_collision;

use crate::rotatingroom::RotatingRoom;

#[macroquad::main("a")]
async fn main() {
    let mut game = Game::new(10.0);
    let mut room = RotatingRoom::new(
        &mut game,
        vec2(400.0, 300.0),
        200.0,
        8,
        10.0,
        1.0, // radians/sec
    );
    let background = load_texture("bg/bg.png").await.unwrap();
    loop {
        let dt = get_frame_time();
        clear_background(WHITE);

        draw_texture(&background, 0.0, 0.0, WHITE);

        room.update(&mut game, dt);

        game.update(dt);

        game.draw();

        next_frame().await;
    }
}
