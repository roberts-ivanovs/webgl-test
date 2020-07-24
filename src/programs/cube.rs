pub mod plane;
pub mod point;

use plane::Plane3D;
use point::Point3D;

#[derive(Debug)]
pub struct Cube {
    pub sides: [Plane3D; 6],
}

impl Cube {
    pub fn new(distance: f32) -> Self {
        let front = Plane3D::new(
            Point3D::new(0. + distance, 0. + distance, 0. + distance),
            Point3D::new(0. + distance, 1. + distance, 0. + distance),
            Point3D::new(1. + distance, 1. + distance, 0. + distance),
            Point3D::new(1. + distance, 0. + distance, 0. + distance),
        );
        let back = Plane3D::new(
            Point3D::new(0. + distance, 0. + distance, -1. + distance),
            Point3D::new(0. + distance, 1. + distance, -1. + distance),
            Point3D::new(1. + distance, 1. + distance, -1. + distance),
            Point3D::new(1. + distance, 0. + distance, -1. + distance),
        );
        let left = Plane3D::new(
            Point3D::new(0. + distance, 0. + distance, -1. + distance),
            Point3D::new(0. + distance, 1. + distance, -1. + distance),
            Point3D::new(0. + distance, 1. + distance, 0. + distance),
            Point3D::new(0. + distance, 0. + distance, 0. + distance),
        );
        let right = Plane3D::new(
            Point3D::new(1. + distance, 0. + distance, -1. + distance),
            Point3D::new(1. + distance, 1. + distance, -1. + distance),
            Point3D::new(1. + distance, 1. + distance, 0. + distance),
            Point3D::new(1. + distance, 0. + distance, 0. + distance),
        );
        let top = Plane3D::new(
            Point3D::new(0. + distance, 1. + distance, 0. + distance), // close left
            Point3D::new(0. + distance, 1. + distance, -1. + distance),
            Point3D::new(1. + distance, 1. + distance, 0. + distance),
            Point3D::new(1. + distance, 1. + distance, -1. + distance),
        );
        let bottom = Plane3D::new(
            Point3D::new(0. + distance, 0. + distance, 0. + distance),
            Point3D::new(0. + distance, 0. + distance, -1. + distance),
            Point3D::new(1. + distance, 0. + distance, 0. + distance),
            Point3D::new(1. + distance, 0. + distance, -1. + distance),
        );
        let sides: [Plane3D; 6] = [front, back, left, right, top, bottom];

        Cube { sides }
    }

    pub fn as_vector(&self) -> Vec<f32> {
        self.sides
            .iter()
            .flat_map(|side| side.points_as_array().clone())
            .collect::<Vec<f32>>()
    }
}
