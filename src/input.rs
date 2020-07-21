use crate::canvas::CanvasData;
use core::f32::consts::PI;

pub struct UserInput<'a> {
    mouse_down: bool,
    mouse_x: f32,
    mouse_y: f32,
    rotation_x_axis: f32,
    rotation_y_axis: f32,
    cd: &'a CanvasData,
}

impl<'a> UserInput<'a> {
    pub fn new(cd: &'a CanvasData) -> Self {
        Self {
            mouse_down: false,
            mouse_x: 0.,
            mouse_y: 0.,
            rotation_x_axis: 0.,
            rotation_y_axis: 0.,
            cd,
        }
    }

    pub fn update_mouse_down(&mut self, x: f32, y: f32, is_down: bool) {
        self.mouse_x = x;
        self.mouse_y = y;
        self.mouse_down = is_down;
    }

    pub fn update_mouse_position(&mut self, x: f32, y: f32) {
        let inverted_y = self.cd.height - y;
        let x_delta = x - self.mouse_x;
        let y_delta = inverted_y - self.mouse_y;
        let rotation_x_delta = if self.mouse_down {
            PI * y_delta / self.cd.height
        } else {
            0.
        };
        let rotation_y_delta = if self.mouse_down {
            PI * x_delta / self.cd.width
        } else {
            0.
        };

        self.mouse_x = x;
        self.mouse_y = y;
        self.rotation_x_axis = self.rotation_x_axis + rotation_x_delta;
        self.rotation_y_axis = self.rotation_y_axis + rotation_y_delta;
    }
}
