// mod gl_setup;
// mod programs;
// mod shaders;
// mod utils;
// mod client;

use crate::canvas::CanvasData;
use crate::transform::Transform;
use crate::programs::box_2d::Box2D;
use crate::programs::cube::Cube;
use crate::shaders::fragment::F_SHADER;
use crate::shaders::vertex::V_SHADER;
use crate::RenderObject;
use crate::RenderableOption;
use crate::{
    gl_setup,
    utils::{console_log, link_program},
};
use wasm_bindgen::prelude::*;
use web_sys::{WebGlProgram, WebGlRenderingContext as GL};

#[wasm_bindgen]
pub struct GlClient {
    gl: GL,
    object: Option<RenderObject>, // program_cube: programs::cube::Cube,
    pub is_ready: bool,
}

#[wasm_bindgen]
impl GlClient {
    #[wasm_bindgen(constructor)]
    pub fn new(opt: RenderableOption, canvas: &CanvasData, transform: &Transform) -> Self {
        let gl: GL = gl_setup::initialize_webgl_context(&canvas.get_canvas()).unwrap();
        let mut client: GlClient = GlClient {
            gl,
            object: None,
            is_ready: false,
        };
        client.set_renderable(opt, canvas, transform);
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
    pub fn set_renderable(&mut self, opt: RenderableOption, canvas: &CanvasData, transform: &Transform) {
        console_log(&format!("Setting rendarble to {:?}", &opt));

        self.is_ready = false;
        let program: WebGlProgram = link_program(&self.gl, &V_SHADER, &F_SHADER).unwrap();

        match opt {
            RenderableOption::Cube => {
                let object = RenderObject::Cube(Cube::new(1.));
                self.object = Some(object);
            }
            RenderableOption::Box2D => {
                let object = RenderObject::Box2D(Box2D::new(&self.gl, program, canvas.clone(), transform.clone()));
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

    #[wasm_bindgen]
    pub fn get_transform(&self) -> Option<Transform> {
        match &self.object {
            Some(renderable) => match &renderable {
                RenderObject::Cube(_) => {
                    // TODO implement cube transform
                    None
                }
                RenderObject::Box2D(obj) => {
                    Some(obj.transform)
                }
            },
            None => {
                None
            }
        }
    }

    #[wasm_bindgen]
    pub fn set_transform(&mut self, new_transform: &Transform) {
        match &mut self.object {
            Some(renderable) => match renderable {
                RenderObject::Cube(_) => {
                    // TODO Implement cube transformation
                    console_log("Transforming Cube");
                }
                RenderObject::Box2D(obj) => {
                    console_log(&format!("Transforming Box2D {:?}", new_transform));
                    obj.transform = new_transform.clone();
                }
            },
            None => {
                console_log("doing Nothing");
            }
        }
    }
}
