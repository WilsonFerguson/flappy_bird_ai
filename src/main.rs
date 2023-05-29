extern crate piston_window;
use piston_window::*;

mod bird;
use bird::Bird;

mod pipe;
use pipe::Pipe;

mod neural_network;

pub static WIDTH: u32 = 1280;
pub static HEIGHT: u32 = 720;
pub static GROUND_HEIGHT: u32 = (HEIGHT as f64 * 0.9) as u32;

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

fn handle_birds(birds: &mut Vec<Bird>, pipes: &Vec<Pipe>) {
    for i in (0..birds.len()).rev() {
        let mut dead = birds.get_mut(i).unwrap().update();
        if !dead {
            dead = birds.get_mut(i).unwrap().check_collision(pipes);
        }
        if dead {
            handle_bird_death(i, birds);
        }
    }
}

fn handle_bird_death(i: usize, birds: &mut Vec<Bird>) {
    // TODO: implement all of the nn learning stuff
    birds.remove(i);
}

fn main() {
    let mut birds: Vec<Bird> = Vec::new();
    let num_birds = 100;
    let flap_force = 7.0;

    for _ in 0..num_birds {
        birds.push(Bird::new());
    }

    let mut pipes: Vec<Pipe> = Vec::new();
    pipes.push(Pipe::new());
    let pipe_gap = 350.0;
    let pipe_speed = 4.0;

    let mut window: PistonWindow = WindowSettings::new("Flappy Bird AI", [WIDTH, HEIGHT])
        .exit_on_esc(true)
        .build()
        .unwrap();
    while let Some(event) = window.next() {
        window.draw_2d(&event, |context, graphics, _device| {
            clear([0.1; 4], graphics);

            handle_pipes(&mut pipes, pipe_gap, pipe_speed);
            pipes.iter().for_each(|pipe| pipe.draw(&context, graphics));

            handle_birds(&mut birds, &pipes);
            birds.iter().for_each(|bird| bird.draw(&context, graphics));

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
                    birds.iter_mut().for_each(|bird| bird.flap(flap_force));
                }
                _ => {}
            }
        }
    }
}
