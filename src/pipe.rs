extern crate piston_window;
use piston_window::*;

use rand::prelude::*;

use crate::{HEIGHT, WIDTH};

pub struct Pipe {
    pub x: f64,
    width: f64,
    top_gap: f64,
    bottom_gap: f64,
}

impl Pipe {
    pub fn new() -> Self {
        let mut rng = thread_rng();
        let gap_size = rng.gen_range(150..225);
        let top_gap = rng.gen_range(100..HEIGHT - gap_size - 100) as f64;
        let bottom_gap = top_gap + gap_size as f64;
        Pipe {
            x: WIDTH as f64,
            width: rng.gen_range(50..100) as f64,
            top_gap,
            bottom_gap,
        }
    }

    pub fn update(&mut self, speed: f64) -> bool {
        self.x -= speed;

        return self.x < -self.width;
    }

    pub fn draw(&self, context: &Context, graphics: &mut G2d) {
        rectangle(
            [0.12, 0.6, 0.25, 1.0],
            [self.x, 0.0, self.width, self.top_gap],
            context.transform,
            graphics,
        );
        rectangle(
            [0.12, 0.6, 0.25, 1.0],
            [
                self.x,
                self.bottom_gap,
                self.width,
                HEIGHT as f64 - self.bottom_gap,
            ],
            context.transform,
            graphics,
        );
    }
}
