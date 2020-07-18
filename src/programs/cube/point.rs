#[derive(Debug)]
pub struct Point {
    x: f32,
    y: f32,
    z: f32,
}

impl Point {
    pub fn new(x: f32, y: f32, z: f32) -> Point {
        Point { x, y, z }
    }


    pub fn as_array(&self) -> [f32; 3] {
        [self.x, self.y, self.z]
    }
}
