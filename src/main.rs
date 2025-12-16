use macroquad::prelude::*;

struct Wall {
    rect: Rect,
}

#[macroquad::main("Walls")]
async fn main() {
    println!("hello world");
    let wall = Wall {
        rect: Rect::new(100.0, 200.0, 300.0, 100.0),
    };

    loop {
        clear_background(BLACK);
        draw_rectangle(wall.rect.x, wall.rect.y, wall.rect.w, wall.rect.h, WHITE);
        next_frame().await;
    }
}
