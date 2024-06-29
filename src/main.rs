use cvector::cvector::CVector;
use macroquad::prelude::*;
use vrd::random::Random;

#[macroquad::main("Purple Rain")]
async fn main() {
    let mut rng = Random::new();

    let mut rain_drops: Vec<RainDrop> = Vec::new();
    for _ in 0..2500 {
        let hor_dis = rng.float();
        let ver_dis = rng.float();
        let speed_m = rng.int(7, 10) as f32 / 10.0;
        rain_drops.push(RainDrop::new(
            screen_width() * hor_dis,
            screen_height() * ver_dis,
            1.0 * speed_m,
        ));
    }

    loop {
        clear_background(GRAY);

        rain_drops.iter_mut().for_each(|drop| {
            drop.update();
            drop.draw_drop();
            drop.respawn();
        });

        next_frame().await
    }
}

struct RainDrop {
    length: f32,
    pos: CVector,
    vel: CVector,
    rand_gen: Random,
}

impl RainDrop {
    fn new(pos_x: f32, pos_y: f32, vel: f32) -> Self {
        Self {
            length: 10.0,
            pos: CVector::new(pos_x, pos_y),
            vel: CVector::new(0.0, vel),
            rand_gen: Random::new(),
        }
    }

    fn draw_drop(&self) {
        draw_line(
            self.pos.x() as f32,
            self.pos.y() as f32,
            self.pos.x() as f32,
            self.pos.y() as f32 + self.length,
            3.0,
            Color {
                r: 112.0 / 255.0,
                g: 39.0 / 255.0,
                b: 195.0 / 255.0,
                a: 255.0 / 255.0,
            },
        );
    }

    fn update(&mut self) {
        self.pos.add(&self.vel);
    }

    fn respawn(&mut self) {
        if self.pos.y() as f32 > screen_height() {
            let hor_dis = self.rand_gen.float();
            let ver_dis = self.rand_gen.int(3, 10) as f32 / 10.0;
            let speed_m = self.rand_gen.int(7, 10) as f32 / 10.0;
            self.pos.reset(screen_width() * hor_dis, -100.0 * ver_dis);
            self.vel.reset(0.0, 1.0 * speed_m);
        }
    }
}
