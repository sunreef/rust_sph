#![feature(unboxed_closures, fn_traits)]
extern crate piston_window;
extern crate rand;
extern crate time;

mod traits;
mod point;
mod kernel;
mod particle;
mod world;
mod grid;



use time::PreciseTime;
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
        let time_start = PreciseTime::now();
        world.update(0.001);
        let time_after_update = PreciseTime::now();
        window.draw_2d(&event, |c, g| {
            clear([0.0,0.0,0.0,0.0], g);
            world.draw(&c, g);
        });
        let time_after_draw = PreciseTime::now();
        println!("Timing: update {}, draw {}.", time_start.to(time_after_update), time_after_update.to(time_after_draw));
    }
}
