mod programs;
mod utils;
mod gl_setup;
mod shaders;

use js_sys;
use utils::{compile_shader, link_program, set_panic_hook};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{console, WebGlProgram, WebGlRenderingContext, WebGlShader};
use programs::{cube::Cube};

// #[wasm_bindgen]
pub struct GlClient {
    pub gl: WebGlRenderingContext,
    // program_cube: programs::cube::Cube,
}

impl GlClient {
    pub fn new() -> Self {
        let gl: WebGlRenderingContext = gl_setup::initialize_webgl_context().unwrap();

        Self {
            // program_cube: <Cube as WebGlRender<Cube>>::new(&gl),
            gl,
        }
    }

    pub fn render(&self) {
        // <Cube as WebGlRender<Cube>>::render(&self.gl)
    }

}

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    set_panic_hook();
    let client = GlClient::new();
    // client.gl.clearColor(0.0, 0.0, 0.0, 1.0);
    // client.gl.
    Ok(())
}
