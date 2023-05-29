extern crate piston_window;
use piston_window::*;

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
            [0.3, 0.8, 0.4, 0.3],
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

    pub fn update(&mut self) {
        self.velocity += 0.35;
        self.y += self.velocity;
        self.limit_velocity();
    }

    pub fn flap(&mut self, force: f64) {
        self.velocity -= force;
        self.limit_velocity();
    }
}
