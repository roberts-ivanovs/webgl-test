mod client;
mod gl_setup;
mod programs;
mod shaders;
mod utils;
mod canvas;

use crate::programs::box_2d::Box2D;
use crate::programs::cube::Cube;
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
