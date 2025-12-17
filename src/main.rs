use macroquad::prelude::*;

struct Wall {
    rect: Rect,
}

#[macroquad::main("Walls")]
async fn main() {
    let room_x = 100.0;
    let room_y = 100.0;
    let room_size = 100.0;
    let thickness = 17.0;

    let top = Wall {
        rect: Rect::new(room_x, room_y, room_size + thickness, thickness),
    };

    let bottom = Wall {
        rect: Rect::new(room_x, room_y + room_size, room_size + thickness, thickness),
    };

    let left = Wall {
        rect: Rect::new(room_x, room_y, thickness, room_size + thickness),
    };

    let right = Wall {
        rect: Rect::new(room_x + room_size, room_y, thickness, room_size + thickness),
    };

    loop {
        clear_background(BLACK);

        draw_circle(150.0, 150.0, 15.0, WHITE);

        draw_rectangle(top.rect.x, top.rect.y, top.rect.w, top.rect.h, RED);
        draw_rectangle(
            bottom.rect.x,
            bottom.rect.y,
            bottom.rect.w,
            bottom.rect.h,
            WHITE,
        );
        draw_rectangle(left.rect.x, left.rect.y, left.rect.w, left.rect.h, BROWN);
        draw_rectangle(
            right.rect.x,
            right.rect.y,
            right.rect.w,
            right.rect.h,
            GREEN,
        );

        next_frame().await;
    }
}
