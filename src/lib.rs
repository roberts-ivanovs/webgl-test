mod client;
mod gl_setup;
mod programs;
mod shaders;
mod utils;
mod canvas;
mod transform;
mod input;

use crate::input::UserInput;
use std::fmt::Debug;
use utils::set_panic_hook;
use wasm_bindgen::prelude::*;

use web_sys::WebGlProgram;
use web_sys::WebGlRenderingContext as GL;
use crate::transform::Transform;
use crate::canvas::CanvasData;



pub trait RenderObjectTrait {

    fn new(gl: &GL, program: WebGlProgram, canvas: CanvasData, transform: Transform) -> Self where Self: Sized;
    fn canvas(&mut self) ->  &mut CanvasData;
    fn set_canvas(&mut self, canvas: CanvasData);
    fn transform(&mut self) -> &mut Transform;
    fn set_transform(&mut self, transform: Transform);
    fn input(&mut self) -> &mut UserInput;
    fn set_input(&mut self, input: UserInput);
    fn draw_scene(&self, gl: &GL);
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
