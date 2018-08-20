use point::Point2f;
use traits::{Update, Draw};

use piston_window::*;
use piston_window::math::*;

use rand::*;

pub struct Particle {
    position: Point2f,
}

impl Particle {
    pub fn new() -> Self {
        Particle {
            position: Point2f::default(),
        }
    }
}

impl Update for Particle {
    fn update(&mut self, delta_time: f32) {
    }
}

impl Draw for Particle {
    fn draw<G: Graphics>(&self, context: &Context, graphics: &mut G) {
        let x = random::<f64>();
        let y = random::<f64>();
        Ellipse::new([1.0,0.0,0.0,1.0]).draw([495.0 * x, 495.0 * y, 5.0,5.0], &context.draw_state, context.transform, graphics);
    }
}



