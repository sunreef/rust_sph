use point::Point2d;
use traits::{Update, Draw};

use piston_window::*;
use piston_window::math::*;

use rand::*;

pub struct Particle {
    pub position: Point2d,
}

impl Particle {
    pub fn new() -> Self {
        Particle {
            position: Point2d::default(),
        }
    }

    pub fn position(mut self, new_pos: Point2d) -> Self {
        self.position = new_pos;
        self
    }
}

impl Update for Particle {
    fn update(&mut self, delta_time: f32) {
    }
}

impl Draw for Particle {
    fn draw<G: Graphics>(&self, context: &Context, graphics: &mut G) {
        Ellipse::new([1.0,0.0,0.0,1.0]).draw([495.0 * self.position.x(), 495.0 * self.position.y(), 5.0,5.0], &context.draw_state, context.transform, graphics);
    }
}



