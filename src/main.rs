const FIRE_RATE:f32 = 0.15;
use macroquad::prelude::*;
mod balls;
mod player;

mod game;
use game::Game;

mod utils;
mod walls;
#[macroquad::main("new game")]

async fn main() {
    let player_texture = load_texture("bg/player.png").await.unwrap();
    let bullet_texture = load_texture("bg/bullet.png").await.unwrap();
    let mut game = Game::new(10.0, vec2(100.0, 100.0), player_texture, bullet_texture);
    let screen_w = screen_width();
    let screen_h = screen_height();
    game.spawn_wall(0.0, 0.0, screen_w, 10.0, 0.0, None);
    game.spawn_wall(0.0, screen_h - 10.0, screen_w, 10.0, 0.0, None);
    game.spawn_wall(0.0, 0.0, 10.0, screen_h, 0.0, None); // Left wall
    game.spawn_wall(screen_w - 10.0, 0.0, 10.0, screen_h, 0.0, None);


    loop {
        clear_background(BLACK);
        
        game.player.fire_cooldown -= get_frame_time(); 
        let cooldown_ready = game.player.fire_cooldown <= 0.0;
        let dt = get_frame_time();
        if is_key_down(KeyCode::W) || is_key_down(KeyCode::Up) || is_key_down(KeyCode::Space) {
            game.player.velocity.y -= 30.0;
        }
        if is_key_down(KeyCode::S) || is_key_down(KeyCode::Down) {
            game.player.velocity.y += 30.0;
        }
        if is_key_down(KeyCode::A) || is_key_down(KeyCode::Left) {
            game.player.velocity.x -= 30.0;
        }
        if is_key_down(KeyCode::D) || is_key_down(KeyCode::Right) {
            game.player.velocity.x += 30.0;
        }
        if is_key_down(KeyCode::Enter) && cooldown_ready {
            game.player.fire_bullet(game.bullet_texture.clone());
            game.player.fire_cooldown = FIRE_RATE;
        }
        game.update(dt);
        game.draw();
        next_frame().await;
    }
}
