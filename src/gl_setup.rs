use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::*;
use web_sys::{HtmlCanvasElement, Document, WebGlRenderingContext as GL};
use crate::utils::console_log;


fn wait_until_canvas_is_rendered(document: Document) -> Result<HtmlCanvasElement, Element> {
    let canvas: Option<Element> = document.get_element_by_id("canvasRust");
    match canvas {
        Some(elem) => {
            elem.dyn_into::<web_sys::HtmlCanvasElement>()
        }
        None => {
            return wait_until_canvas_is_rendered(document);
        }
    }
}

pub fn initialize_webgl_context() -> Result<WebGlRenderingContext, JsValue> {
    console_log("Inicializing webgl");
    let window = window().unwrap();
    let document = window.document().unwrap();
    let canvas: HtmlCanvasElement = wait_until_canvas_is_rendered(document)?;
    let gl: WebGlRenderingContext = canvas.get_context("webgl")?.unwrap().dyn_into()?;

    gl.clear_color(0.0, 0.0, 0.0, 1.0); //RGBA
    gl.clear(GL::COLOR_BUFFER_BIT);

    Ok(gl)
}
