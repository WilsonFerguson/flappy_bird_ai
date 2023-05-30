extern crate piston_window;
use std::time::Instant;

use piston_window::*;

mod bird;
use bird::Bird;

mod pipe;
use pipe::Pipe;
use rand::Rng;

mod neural_network;

pub static WIDTH: u32 = 1280;
pub static HEIGHT: u32 = 720;
pub static GROUND_HEIGHT: u32 = (HEIGHT as f64 * 0.9) as u32;

pub static NEURAL_NETWORK_STRUCTURE: &[usize] = &[2, 5, 3, 2];
pub static FLAP_FORCE: f64 = 6.0;

fn handle_pipes(pipes: &mut Vec<Pipe>, pipe_gap: f64, pipe_speed: f64) {
    if let Some(pipe) = pipes.last() {
        if pipe.x < WIDTH as f64 - pipe_gap {
            pipes.push(Pipe::new());
        }
    }
    for i in (0..pipes.len()).rev() {
        let should_remove = pipes.get_mut(i).unwrap().update(pipe_speed);
        if should_remove {
            pipes.remove(i);
        }
    }
}

fn handle_birds(birds: &mut Vec<Bird>, finished_birds: &mut Vec<Bird>, pipes: &Vec<Pipe>) {
    // Closest pipe to x = 100 while still being in front of the bird
    let nearest_pipe = pipes
        .iter()
        .filter(|pipe| pipe.x + pipe.width > 100.0 - 20.0) // 20 is the bird radius
        .min_by(|a, b| a.x.partial_cmp(&b.x).unwrap())
        .unwrap();

    // Call auto_fly on each bird passing in the nearest pipe
    birds
        .iter_mut()
        .for_each(|bird| bird.auto_fly(nearest_pipe));

    for i in (0..birds.len()).rev() {
        let mut dead = birds.get_mut(i).unwrap().update();
        if !dead {
            dead = birds.get_mut(i).unwrap().check_collision(pipes);
        }
        if dead {
            handle_bird_death(i, birds, finished_birds);
        }
    }
}

fn handle_bird_death(i: usize, birds: &mut Vec<Bird>, finished_birds: &mut Vec<Bird>) {
    finished_birds.push(birds.remove(i));
}

fn select_parent(parents: &Vec<Bird>) -> Bird {
    let mut r = rand::thread_rng().gen_range(0.0..1.0);
    for parent in parents {
        r -= parent.fitness;
        if r <= 0.0 {
            return parent.make_copy();
        }
    }

    return parents[0].make_copy();
}

fn next_generation(birds: &mut Vec<Bird>, finished_birds: &mut Vec<Bird>, pipes: &mut Vec<Pipe>) {
    // TODO: add abilitiy to switch between different selection methods (and add crossover support,
    // NN already has support btw)

    // Scale fitnesses between 0 and 1
    let fitness_sum = finished_birds
        .iter()
        .fold(0.0, |acc, bird| acc + bird.fitness);
    finished_birds
        .iter_mut()
        .for_each(|bird| bird.fitness /= fitness_sum);

    // Sort the birds by fitness (descending)
    finished_birds.sort_by(|a, b| b.fitness.partial_cmp(&a.fitness).unwrap());

    println!("Best Fitness: {}", finished_birds[0].fitness * fitness_sum);

    // Random mutation method:
    for _ in 0..finished_birds.len() {
        let mut child_bird = select_parent(&finished_birds);
        child_bird.mutate(0.1);
        birds.push(child_bird);
    }

    pipes.clear();
    pipes.push(Pipe::new());

    finished_birds.clear();
}

fn main() {
    let mut birds: Vec<Bird> = Vec::new();
    let num_birds = 30;
    for _ in 0..num_birds {
        birds.push(Bird::new());
    }

    let mut finished_birds: Vec<Bird> = Vec::new();
    let mut generation: usize = 1;

    let mut pipes: Vec<Pipe> = Vec::new();
    pipes.push(Pipe::new());
    let pipe_gap = 350.0;
    let pipe_speed = 4.0;

    let start_time = Instant::now();

    let mut window: PistonWindow = WindowSettings::new("Flappy Bird AI", [WIDTH, HEIGHT])
        .exit_on_esc(true)
        .build()
        .unwrap();
    while let Some(event) = window.next() {
        window.draw_2d(&event, |context, graphics, _device| {
            clear([0.1; 4], graphics);

            handle_pipes(&mut pipes, pipe_gap, pipe_speed);
            pipes.iter().for_each(|pipe| pipe.draw(&context, graphics));

            handle_birds(&mut birds, &mut finished_birds, &pipes);
            birds.iter().for_each(|bird| bird.draw(&context, graphics));

            if birds.len() == 0 {
                next_generation(&mut birds, &mut finished_birds, &mut pipes);
                generation += 1;
                println!("New Generation. Now on generation #{}.", generation);
            }

            // Draw the ground
            rectangle(
                [0.53, 0.29, 0.12, 1.0],
                [
                    0.0,
                    GROUND_HEIGHT as f64,
                    WIDTH as f64,
                    HEIGHT as f64 - GROUND_HEIGHT as f64,
                ],
                context.transform,
                graphics,
            );
        });

        // Keyboard input
        if let Some(Button::Keyboard(key)) = event.press_args() {
            match key {
                Key::Space => {
                    birds.iter_mut().for_each(|bird| bird.flap());
                }
                _ => {}
            }
        }
    }

    let end_time = Instant::now();
    let duration = end_time.duration_since(start_time);
    println!(
        "Time elapsed: {}.{:03} seconds.",
        duration.as_secs(),
        duration.subsec_millis()
    );
}
