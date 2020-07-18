mod plane;
mod point;

use plane::Plane;
use point::Point;

#[derive(Debug)]
pub struct Cube {
    pub sides: [Plane; 6],
}

impl Cube {
    pub fn new(distance: f32) -> Self {
        let front = Plane::new(
            Point::new(0. + distance, 0. + distance, 0. + distance),
            Point::new(0. + distance, 1. + distance, 0. + distance),
            Point::new(1. + distance, 1. + distance, 0. + distance),
            Point::new(1. + distance, 0. + distance, 0. + distance),
        );
        let back = Plane::new(
            Point::new(0. + distance, 0. + distance, -1. + distance),
            Point::new(0. + distance, 1. + distance, -1. + distance),
            Point::new(1. + distance, 1. + distance, -1. + distance),
            Point::new(1. + distance, 0. + distance, -1. + distance),
        );
        let left = Plane::new(
            Point::new(0. + distance, 0. + distance, -1. + distance),
            Point::new(0. + distance, 1. + distance, -1. + distance),
            Point::new(0. + distance, 1. + distance, 0. + distance),
            Point::new(0. + distance, 0. + distance, 0. + distance),
        );
        let right = Plane::new(
            Point::new(1. + distance, 0. + distance, -1. + distance),
            Point::new(1. + distance, 1. + distance, -1. + distance),
            Point::new(1. + distance, 1. + distance, 0. + distance),
            Point::new(1. + distance, 0. + distance, 0. + distance),
        );
        let top = Plane::new(
            Point::new(0. + distance, 1. + distance, 0. + distance), // close left
            Point::new(0. + distance, 1. + distance, -1. + distance),
            Point::new(1. + distance, 1. + distance, 0. + distance),
            Point::new(1. + distance, 1. + distance, -1. + distance),
        );
        let bottom = Plane::new(
            Point::new(0. + distance, 0. + distance, 0. + distance),
            Point::new(0. + distance, 0. + distance, -1. + distance),
            Point::new(1. + distance, 0. + distance, 0. + distance),
            Point::new(1. + distance, 0. + distance, -1. + distance),
        );
        let sides: [Plane; 6] = [front, back, left, right, top, bottom];

        Cube { sides }
    }

    pub fn as_vector(&self) -> Vec<f32> {
        self.sides
            .iter()
            .flat_map(|side| side.points_as_array().clone())
            .collect::<Vec<f32>>()
    }
}
