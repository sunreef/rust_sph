use std::vec::Vec;
use rand;
use time::PreciseTime;
use piston_window::*;
use piston_window::math::*;

use point::Point2d;
use particle::Particle;
use traits::{
    Update,
    Draw
};
use grid::Grid;

pub struct World {
    timer: f32,

    particles: Vec<Particle>,

}

impl World {
    pub fn new() -> Self {
        World {
            timer: 0f32,
            particles: Vec::new(),
        }
    }
}

impl Update for World {
    fn update(&mut self, delta_time: f32) {
        self.timer += delta_time;
        while self.particles.len() < 1000 {
            self.particles.push(Particle::new()
                .position(Point2d::new(rand::random(), rand::random())));
        }
        let grid = Grid::new(&self.particles, 0.1f64);
    }
}

impl Draw for World {
    fn draw<G: Graphics>(&self, context: &Context, graphics: &mut G) {
        println!("number of particles {}", self.particles.len());
        for particle in &self.particles {
            particle.draw(context, graphics);
        }
    }
}
