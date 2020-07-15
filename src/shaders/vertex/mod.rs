use std::fs;

pub fn fetch_v_shader() -> &'static str {
    // fs::read_to_string("./vsSource.glsl").unwrap()
    r#"
    attribute vec4 aVertexPosition;

    uniform mat4 uModelViewMatrix;
    uniform mat4 uProjectionMatrix;

    void main() {
        gl_Position = uProjectionMatrix * uModelViewMatrix * aVertexPosition;
    }
    "#
}
