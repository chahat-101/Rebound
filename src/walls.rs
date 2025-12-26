use macroquad::prelude::*;

pub struct Wall {
    pub rect: Rect,
    pub angle: f32,
    pub velocity: Option<Vec2>,
}

impl Wall {
    pub fn update_and_draw(&mut self, acceleration: Vec2, dt: f32) {
        if let Some(mut v) = self.velocity {
            let old_angle = v.y.atan2(v.x);
            v += acceleration * dt;
            self.rect.x += v.x * dt;
            self.rect.y += v.y * dt;

            let new_angle = v.y.atan2(v.x);
            self.angle += new_angle - old_angle;
            self.angle = self.angle.rem_euclid(std::f32::consts::TAU);

            self.velocity = Some(v);
        }
    }

    pub fn draw(&self) {
        draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, WHITE);
    }
}
