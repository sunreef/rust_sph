use piston_window::*;
use piston_window::math::*;

pub trait Update {
    fn update(&mut self, delta_time: f32);
}

pub trait Draw {
    fn draw<G: Graphics>(&self, context: &Context, graphics: &mut G);
}