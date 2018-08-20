#![feature(unboxed_closures, fn_traits)]
extern crate piston_window;
extern crate rand;

mod traits;
mod point;
mod kernel;
mod particle;
mod world;



use world::World;
use traits::{Update, Draw};

use piston_window::*;

fn main() {
    let mut window: PistonWindow = WindowSettings::new(
        "test_window",
        [500, 500]
    )
        .exit_on_esc(true)
        .samples(4)
        .build()
        .unwrap();

    let mut world : World = World::new();

    while let Some(event) = window.next() {
        world.update(0.001);
        window.draw_2d(&event, |c, g| {
            clear([0.0,0.0,0.0,0.0], g);
            world.draw(&c, g);
        });
    }
}
