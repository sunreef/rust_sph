use std::vec::Vec;
use piston_window::*;
use piston_window::math::*;

use particle::Particle;
use traits::{
    Update,
    Draw
};

pub struct World {
    particles: Vec<Particle>,
}

impl World {
    pub fn new() -> Self {
        World {
            particles: Vec::new(),
        }
    }
}

impl Update for World {
    fn update(&mut self, delta_time: f32) {
        if self.particles.len() < 500 {
            self.particles.push(Particle::new());
        }
    }
}

impl Draw for World {
    fn draw<G: Graphics>(&self, context: &Context, graphics: &mut G) {
        for particle in &self.particles {
            particle.draw(context, graphics);
        }
    }
}