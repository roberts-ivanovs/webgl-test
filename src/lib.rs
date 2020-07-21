mod client;
mod gl_setup;
mod programs;
mod shaders;
mod utils;
mod canvas;
mod transform;

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
