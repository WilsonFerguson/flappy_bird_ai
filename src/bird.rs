extern crate piston_window;
use piston_window::*;

use crate::{
    neural_network::NeuralNetwork, pipe::Pipe, FLAP_FORCE, GROUND_HEIGHT, HEIGHT,
    NEURAL_NETWORK_STRUCTURE,
};

pub struct Bird {
    x: f64,
    y: f64,
    velocity: f64,
    radius: f64,
    pub fitness: f64,
    brain: NeuralNetwork,
}

impl Bird {
    pub fn new() -> Self {
        Bird {
            x: 100.0,
            y: 200.0,
            velocity: 0.0,
            radius: 20.0,
            fitness: 0.0,
            brain: NeuralNetwork::new(NEURAL_NETWORK_STRUCTURE),
        }
    }

    pub fn draw(&self, context: &Context, graphics: &mut G2d) {
        ellipse(
            [0.92, 0.25, 0.2, 0.3],
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

        // Hit ceiling or ground
        if self.y - self.radius < 0.0 {
            self.y = self.radius;
            return true;
        }
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

    pub fn flap(&mut self) {
        self.velocity -= FLAP_FORCE;
        self.limit_velocity();
    }

    pub fn auto_fly(&mut self, pipe: &Pipe) {
        let gap_center = (pipe.top_gap + pipe.bottom_gap) / 2.0;

        let inputs: Vec<f64> = vec![self.velocity, self.y - gap_center]; // maybe
                                                                         // make 3rd arg "gap_center - self.y"?
        let outputs = self.brain.feed_forward(&inputs);
        if outputs[0] > outputs[1] {
            self.flap();
        }

        // Increase fitness based on how close it is to the center of the gap
        let mut y_diff = (self.y - gap_center).abs();
        y_diff = y_diff / (HEIGHT as f64 / 2.0);
        self.fitness += 1.0 - y_diff;
    }

    pub fn mutate(&mut self, mutation_rate: f64) {
        self.brain.mutate(mutation_rate);
    }

    pub fn make_copy(&self) -> Bird {
        Bird {
            x: 100.0,
            y: 200.0,
            velocity: 0.0,
            radius: 20.0,
            fitness: 0.0,
            brain: self.brain.clone(),
        }
    }
}
