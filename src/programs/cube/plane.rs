// mod point;
// use self::point::Point;
use super::point::Point;

#[derive(Debug)]
pub struct Plane {
    pub points: [Point; 4],
}

impl Plane {
    pub fn new(bl: Point, tl: Point, tr: Point, br: Point) -> Self {
        Plane {
            points: [bl, tl, tr, br],
        }
    }
    pub fn points_as_array(&self) -> Vec<f32> {

        let mut returnable: Vec<f32> = vec![];

        self.points.iter().for_each(|point| {
            point.as_array().iter().for_each(|coord| {
                returnable.push(coord.clone());
            })
        });
        returnable
    }
}
