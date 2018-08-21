use std::vec::Vec;
use std::f64;

use point::Point2d;
use particle::Particle;

pub struct Grid {
    cell_size: f64,
    cells: Vec<Vec<Vec<usize>>>,
}

impl Grid {
    pub fn new(particles: &Vec<Particle>, cell_size: f64) -> Grid {
        let mut bounding_box = (f64::MAX, f64::MAX, f64::MIN, f64::MIN);

        for particle in particles {
            bounding_box.0 = f64::min(bounding_box.0, particle.position[0]);
            bounding_box.1 = f64::min(bounding_box.1, particle.position[1]);
            bounding_box.2 = f64::max(bounding_box.2, particle.position[0]);
            bounding_box.3 = f64::max(bounding_box.3, particle.position[1]);
        }
        let grid_dimensions = (
            ((bounding_box.2 - bounding_box.0) / cell_size).floor() as usize + 1usize,
            ((bounding_box.3 - bounding_box.1) / cell_size).floor() as usize + 1usize
        );

        let mut cells = Vec::new();
        for x in 0..grid_dimensions.0 {
            cells.push(Vec::new());
            for y in 0..grid_dimensions.1 {
                cells[x].push(Vec::<usize>::new());
            }
        }

        Grid {
            cell_size: cell_size,
            cells: Vec::new(),
        }

    }
}

