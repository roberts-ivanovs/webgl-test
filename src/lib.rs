#[macro_use]
extern crate lazy_static;

mod gl_setup;
mod programs;
mod shaders;
mod utils;

use crate::programs::box_2d::Box2D;
use crate::shaders::fragment::fetch_f_shader;
use crate::shaders::vertex::fetch_v_shader;
use programs::cube::Cube;
use utils::{link_program, set_panic_hook};
use wasm_bindgen::prelude::*;
use web_sys::{WebGlProgram, WebGlRenderingContext};

lazy_static! {
    static ref F_SHADER: &'static str = r#"
    void main() {
      gl_FragColor = vec4(1.0, 1.0, 1.0, 1.0);
    }
    "#;
    static ref V_SHADER: &'static str = r#"
    attribute vec4 aVertexPosition;
    uniform mat4 uModelViewMatrix;
    uniform mat4 uProjectionMatrix;
    void main() {
      gl_Position = uProjectionMatrix * uModelViewMatrix * aVertexPosition;
    }
    "#;
}

pub enum RenderObject {
    Cube(Option<Cube>),
    Box2D(Option<Box2D>),
}

// #[wasm_bindgen]
pub struct GlClient {
    pub gl: WebGlRenderingContext,
    pub object: RenderObject, // program_cube: programs::cube::Cube,
}

impl GlClient {
    pub fn new(creatable: RenderObject) -> Self {
        let gl: WebGlRenderingContext = gl_setup::initialize_webgl_context().unwrap();
        let program: WebGlProgram =
            link_program(&gl, &V_SHADER, &F_SHADER).unwrap();
        match creatable {
            RenderObject::Cube(_) => {
                let object = RenderObject::Cube(Some(Cube::new(1.)));
                Self {
                    object: object,
                    gl: gl,
                }
            }
            RenderObject::Box2D(_) => {
                let object = RenderObject::Box2D(Some(Box2D::new(&gl, program)));
                Self { object, gl }
            }
        }
    }

    pub fn render(&self) {
        match &self.object {
            RenderObject::Cube(cube) => match cube {
                Some(_cube) => {}
                None => {}
            },
            RenderObject::Box2D(box2d) => match &box2d {
                Some(box2d) => {
                    let buffers = box2d.init_buffers(&self.gl);
                    box2d.draw_scene(&self.gl, buffers);
                }
                None => {}
            },
        }
        // <Cube as WebGlRender<Cube>>::render(&self.gl)
    }
}

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    set_panic_hook();
    let client = GlClient::new(RenderObject::Box2D(None));
    client.render();
    // client.gl.clearColor(0.0, 0.0, 0.0, 1.0);
    // client.gl.
    Ok(())
}
