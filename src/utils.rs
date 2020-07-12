use web_sys::WebGlProgram;
use web_sys::WebGlShader;
use web_sys::{console, WebGlRenderingContext};


pub fn console_log(to_log: &str) {
    let array = js_sys::Array::new();
    array.push(&to_log.into());
    console::log(&array);
}

pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

pub fn compile_shader(
    context: &WebGlRenderingContext,
    shader_type: u32,
    source: &str,
) -> Result<WebGlShader, String> {
    let shader = context
        .create_shader(shader_type)
        .ok_or_else(|| String::from("Unable to create shader object"))?;
    context.shader_source(&shader, source);
    context.compile_shader(&shader);

    if context
        .get_shader_parameter(&shader, WebGlRenderingContext::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(shader)
    } else {
        Err(context
            .get_shader_info_log(&shader)
            .unwrap_or_else(|| String::from("Unknown error creating shader")))
    }
}

pub fn link_program(
    gl: &WebGlRenderingContext,
    vert_source: &str,
    frag_source: &str,
) -> Result<WebGlProgram, String> {
    let program = gl
        .create_program()
        .ok_or_else(|| String::from("Error creating program"))?;
    let vert_shader = compile_shader(
        &gl, WebGlRenderingContext::VERTEX_SHADER, vert_source
    ).unwrap();

    let frag_shader = compile_shader(
        &gl, WebGlRenderingContext::FRAGMENT_SHADER, frag_source,
    ).unwrap();

    gl.attach_shader(&program, &vert_shader);
    gl.attach_shader(&program, &frag_shader);
    gl.link_program(&program);

    if gl.get_program_parameter(&program, WebGlRenderingContext::LINK_STATUS).as_bool().unwrap_or(false) {
        Ok(program)
    } else {
        Err(gl.get_program_info_log(&program).unwrap_or_else(|| String::from("Unknown error creating program object!")))
    }
}
