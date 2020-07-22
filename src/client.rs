use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::*;
use crate::canvas::CanvasData;
use crate::transform::Transform;
use crate::programs::box_2d::Box2D;
use crate::shaders::fragment::F_SHADER;
use crate::shaders::vertex::V_SHADER;
use crate::RenderableOption;
use crate::{
    gl_setup,
    utils::{console_log, link_program}, RenderObjectTrait,
};
use web_sys::{WebGlProgram, WebGlRenderingContext as GL, HtmlCanvasElement};

#[wasm_bindgen]
pub struct GlClient {
    gl: GL,
    object: Option<Box<dyn RenderObjectTrait>>,
    pub is_ready: bool,
    master_canvas: HtmlCanvasElement
}

#[wasm_bindgen]
impl GlClient {
    #[wasm_bindgen(constructor)]
    pub fn new(opt: RenderableOption, canvas: &CanvasData, transform: &Transform) -> Self {
        let canvas_el: HtmlCanvasElement = gl_setup::get_canvas(&canvas.get_canvas());
        let gl: GL = gl_setup::initialize_webgl_context(&canvas_el).unwrap();
        let mut client: GlClient = GlClient {
            gl,
            object: None,
            is_ready: false,
            master_canvas: canvas_el,
        };
        client.attach_mouse_down_handler(&client.master_canvas);
        client.attach_mouse_up_handler(&client.master_canvas);
        client.attach_mouse_move_handler(&client.master_canvas);
        client.set_renderable(opt, canvas, transform);
        client
    }

    #[wasm_bindgen]
    pub fn render(&self) {
        match &self.object {
            Some(obj) => {
                obj.draw_scene(&self.gl);
            }
            None => {
                console_log("Clearing the canvas");
                self.clear();
            }
        }
    }

    fn attach_mouse_down_handler(&self, canvas: &HtmlCanvasElement) -> Result<(), JsValue> {
        // let handler = move |event: web_sys::MouseEvent| {
        //     super::app_state::update_mouse_down(event.client_x() as f32, event.client_y() as f32, true);
        // };

        // let handler = Closure::wrap(Box::new(handler) as Box<dyn FnMut(_)>);
        // canvas.add_event_listener_with_callback("mousedown", handler.as_ref().unchecked_ref())?;
        // handler.forget();

        Ok(())
    }

    fn attach_mouse_up_handler(&self, canvas: &HtmlCanvasElement) -> Result<(), JsValue> {
        // let handler = move |event: web_sys::MouseEvent| {
        //     super::app_state::update_mouse_down(event.client_x() as f32, event.client_y() as f32, false);
        // };

        // let handler = Closure::wrap(Box::new(handler) as Box<dyn FnMut(_)>);
        // canvas.add_event_listener_with_callback("mouseup", handler.as_ref().unchecked_ref())?;
        // handler.forget();

        Ok(())
    }

    fn attach_mouse_move_handler(&self, canvas: &HtmlCanvasElement) -> Result<(), JsValue> {
        // let handler = move |event: web_sys::MouseEvent| {
        //     super::app_state::update_mouse_position(event.client_x() as f32, event.client_y() as f32);
        // };

        // let handler = Closure::wrap(Box::new(handler) as Box<dyn FnMut(_)>);
        // canvas.add_event_listener_with_callback("mousemove", handler.as_ref().unchecked_ref())?;
        // handler.forget();

        Ok(())
    }

    #[wasm_bindgen]
    pub fn set_renderable(&mut self, opt: RenderableOption, canvas: &CanvasData, transform: &Transform) {
        console_log(&format!("Setting rendarble to {:?}", &opt));

        self.is_ready = false;
        let program: WebGlProgram = link_program(&self.gl, &V_SHADER, &F_SHADER).unwrap();

        match opt {
            RenderableOption::Cube => {
                // let object = RenderObject::Cube(Cube::new(1.));
                // TODO implement cube rendering
                self.object = None;
            }
            RenderableOption::Box2D => {
                let object: Box<Box2D> = Box::new(RenderObjectTrait::new(&self.gl, program, canvas.clone(), transform.clone()));
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
            Some(obj) => {
                Some(obj.transform().clone())
            }
            None => {None}
        }
    }

    #[wasm_bindgen]
    pub fn set_transform(&mut self, new_transform: &Transform) {
        match &mut self.object {
            Some(obj) => {
                obj.set_transform(new_transform.clone());
            }
            None => {
                console_log("doing Nothing");
            }
        }
    }
}
