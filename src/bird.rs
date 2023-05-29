extern crate piston_window;
use piston_window::*;

use crate::{pipe::Pipe, GROUND_HEIGHT};

pub struct Bird {
    x: f64,
    y: f64,
    velocity: f64,
    radius: f64,
}

impl Bird {
    pub fn new() -> Self {
        Bird {
            x: 100.0,
            y: 200.0,
            velocity: 0.0,
            radius: 20.0,
        }
    }

    pub fn draw(&self, context: &Context, graphics: &mut G2d) {
        ellipse(
            [0.92, 0.25, 0.2, 1.0],
            [
                self.x - self.radius,
                self.y - self.radius,
                self.radius * 2.0,
                self.radius * 2.0,
            ],
            context.transform,
            graphics,
        )
    }

    fn limit_velocity(&mut self) {
        if self.velocity > 10.0 {
            self.velocity = 10.0;
        }
        if self.velocity < -10.0 {
            self.velocity = -10.0;
        }
    }

    pub fn update(&mut self) -> bool {
        self.velocity += 0.35;
        self.y += self.velocity;

        self.limit_velocity();

        if self.y > GROUND_HEIGHT as f64 {
            self.y = GROUND_HEIGHT as f64;
            return true;
        }

        return false;
    }

    pub fn check_collision(&self, pipes: &Vec<Pipe>) -> bool {
        for pipe in pipes {
            if self.x + self.radius < pipe.x || self.x - self.radius > pipe.x + pipe.width {
                continue;
            }

            if self.y - self.radius < pipe.top_gap || self.y + self.radius > pipe.bottom_gap {
                return true;
            }
        }

        return false;
    }
    pub fn flap(&mut self, force: f64) {
        self.velocity -= force;
        self.limit_velocity();
    }
}
