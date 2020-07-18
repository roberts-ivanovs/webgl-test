mod gl_setup;
mod programs;
mod shaders;
mod utils;

use std::fmt::{Debug};
use crate::programs::box_2d::Box2D;
use crate::programs::cube::Cube;
use crate::shaders::fragment::F_SHADER;
use crate::shaders::vertex::V_SHADER;
use utils::{console_log, link_program, set_panic_hook};
use wasm_bindgen::prelude::*;
use web_sys::{WebGlProgram, WebGlRenderingContext as GL};


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

#[wasm_bindgen]
pub struct GlClient {
    gl: GL,
    object: Option<RenderObject>, // program_cube: programs::cube::Cube,
    pub is_ready: bool,
}

#[wasm_bindgen]
impl GlClient {
    #[wasm_bindgen(constructor)]
    pub fn new(opt: RenderableOption) -> Self {
        let gl: GL = gl_setup::initialize_webgl_context().unwrap();
        let mut client: GlClient = GlClient {
            gl,
            object: None,
            is_ready: false,
        };
        client.set_renderable(opt);
        client
    }

    #[wasm_bindgen]
    pub fn render(&self) {
        match &self.object {
            Some(renderable) => match &renderable {
                RenderObject::Cube(_) => {
                    // TODO Implement cube rendering
                    console_log("Clearing the canvas");
                    self.clear();
                }
                RenderObject::Box2D(obj) => {
                    console_log("Drawing Box2D");
                    obj.draw_scene(&self.gl);
                }
            },
            None => {
                console_log("doing Nothing");
            }
        }
    }

    #[wasm_bindgen]
    pub fn set_renderable(&mut self, opt: RenderableOption) {
        console_log(&format!("Setting rendarble to {:?}", &opt));

        self.is_ready = false;
        let program: WebGlProgram = link_program(&self.gl, &V_SHADER, &F_SHADER).unwrap();

        match opt {
            RenderableOption::Cube => {
                let object = RenderObject::Cube(Cube::new(1.));
                self.object = Some(object);
            }
            RenderableOption::Box2D => {
                let object = RenderObject::Box2D(Box2D::new(&self.gl, program));
                self.object = Some(object);
            }
        }
        self.is_ready = true;
    }

    fn clear(&self) {
        self.gl.clear_color(0., 0., 0., 1.);
        self.gl.clear_depth(1.);
        self.gl.clear(GL::COLOR_BUFFER_BIT | GL::DEPTH_BUFFER_BIT);
    }
}

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    set_panic_hook();
    Ok(())
}
