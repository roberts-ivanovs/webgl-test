use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::*;
use web_sys::WebGlRenderingContext as GL;
use crate::utils::console_log;

pub fn initialize_webgl_context() -> Result<WebGlRenderingContext, JsValue> {
    console_log("Inicializing webgl");
    let window = window().unwrap();
    let document = window.document().unwrap();
    let canvas = document.get_element_by_id("canvasRust").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>()?;
    let gl: WebGlRenderingContext = canvas.get_context("webgl")?.unwrap().dyn_into()?;

    gl.clear_color(0.0, 0.0, 0.0, 1.0); //RGBA
    gl.clear(GL::COLOR_BUFFER_BIT);

    Ok(gl)
}
