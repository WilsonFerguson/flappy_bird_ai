extern crate piston_window;
use piston_window::*;

mod bird;
use bird::Bird;

mod pipe;
use pipe::Pipe;

pub static WIDTH: u32 = 1280;
pub static HEIGHT: u32 = 720;

fn main() {
    let mut birds: Vec<Bird> = Vec::new();
    let num_birds = 100;

    for _ in 0..num_birds {
        birds.push(Bird::new());
    }

    let mut pipes: Vec<Pipe> = Vec::new();
    pipes.push(Pipe::new());
    let pipe_gap = 350.0;
    let pipe_speed = 4.0;

    let mut window: PistonWindow = WindowSettings::new("Test App", [WIDTH, HEIGHT])
        .exit_on_esc(true)
        .build()
        .unwrap();
    while let Some(event) = window.next() {
        window.draw_2d(&event, |context, graphics, _device| {
            clear([0.1; 4], graphics);

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
            pipes.iter().for_each(|pipe| pipe.draw(&context, graphics));

            birds.iter_mut().for_each(|bird| bird.update());
            birds.iter().for_each(|bird| bird.draw(&context, graphics));
        });
    }
}
