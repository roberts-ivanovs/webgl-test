use core::f32::consts::PI;
use wasm_bindgen::prelude::*;


#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct CanvasData {
    aspect: f32,
    field_of_view: f32,
    canvas_id: String,
}

#[wasm_bindgen]
impl CanvasData {
    #[wasm_bindgen(constructor)]
    pub fn new(width: f32, height: f32, degrees: f32, canvas_id: String) -> Self {
        Self {
            aspect: CanvasData::calculate_aspect(width, height),
            field_of_view: CanvasData::calculate_fov(degrees),
            canvas_id,
        }
    }

    pub fn calculate_aspect(width: f32, height: f32) -> f32 {
        width / height as f32
    }

    pub fn calculate_fov(degrees: f32) -> f32 {
        (degrees * PI / 180.) as f32
    }

    #[wasm_bindgen]
    pub fn set_fov(&mut self, degrees: f32) {
        self.field_of_view = CanvasData::calculate_fov(degrees);
    }

    #[wasm_bindgen]
    pub fn set_aspect(&mut self, width: f32, height: f32) {
        self.aspect = CanvasData::calculate_aspect(width, height);
    }

    pub fn get_canvas(&self) -> String {
        self.canvas_id.to_string()
    }

    pub fn get_aspect(&self) -> f32 {
        self.aspect
    }

    pub fn get_fov(&self) -> f32 {
        self.field_of_view
    }
}
