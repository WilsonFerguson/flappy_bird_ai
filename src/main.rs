extern crate piston_window;
use piston_window::*;

mod bird;
use bird::Bird;

fn main() {
    let mut birds: Vec<Bird> = Vec::new();
    let num_birds = 100;

    for _ in 0..num_birds {
        birds.push(Bird::new());
    }

    let mut window: PistonWindow = WindowSettings::new("Test App", [1280, 720])
        .exit_on_esc(true)
        .build()
        .unwrap();
    while let Some(event) = window.next() {
        window.draw_2d(&event, |context, graphics, _device| {
            clear([0.1; 4], graphics);

            birds.iter().for_each(|bird| bird.draw(&context, graphics));
            birds.iter_mut().for_each(|bird| bird.update());
        });
    }
}
