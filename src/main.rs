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
    game.spawn_ball((room.centre.x, room.centre.y), vec2(0.0, 100.0), 10.0, true);
    game.spawn_wall(
        room.centre.x - 10.0,
        room.centre.y + 10.0,
        30.0,
        15.0,
        90.0,
        Option::None,
    );
    let background = load_texture("bg/bg.png").await.unwrap();
    loop {
        let dt = get_frame_time();
        clear_background(WHITE);
        draw_texture_ex(
            &background,
            0.0,
            0.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(screen_width(), screen_height())),
                ..Default::default()
            },
        );

        room.update(&mut game, dt);

        game.update(dt);

        game.draw();

        next_frame().await;
    }
}
