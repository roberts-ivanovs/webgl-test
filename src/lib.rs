mod client;
mod gl_setup;
mod programs;
mod shaders;
mod utils;

use crate::programs::box_2d::Box2D;
use crate::programs::cube::Cube;
use core::f32::consts::PI;
use std::fmt::Debug;
use utils::set_panic_hook;
use wasm_bindgen::prelude::*;

pub enum RenderObject {
    Cube(Cube),
    Box2D(Box2D),
}

#[wasm_bindgen]
#[derive(Debug)]
pub enum RenderableOption {
    Cube,
    Box2D,
}

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    set_panic_hook();
    Ok(())
}

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
}

#[wasm_bindgen]
#[derive(Debug, Copy, Clone)]
pub struct Transform {
    trans_x: f32,
    trans_y: f32,
    trans_z: f32,
}


#[wasm_bindgen]
impl Transform {
    #[wasm_bindgen(constructor)]
    pub fn new(trans_x: f32, trans_y: f32, trans_z: f32) -> Self {
        Self {
            trans_x,
            trans_y,
            trans_z,
        }
    }

    #[wasm_bindgen]
    pub fn set_trans_x(&mut self, new_x: f32) {
        self.trans_x = new_x
    }

    #[wasm_bindgen]
    pub fn set_trans_y(&mut self, new_y: f32) {
        self.trans_y = new_y
    }

    #[wasm_bindgen]
    pub fn set_trans_z(&mut self, new_z: f32) {
        self.trans_z = new_z
    }

    #[wasm_bindgen]
    pub fn get_trans_x(&self) -> f32 {
        self.trans_x
    }

    #[wasm_bindgen]
    pub fn get_trans_y(&self) -> f32 {
        self.trans_y
    }

    #[wasm_bindgen]
    pub fn get_trans_z(&self) -> f32 {
        self.trans_z
    }
}
