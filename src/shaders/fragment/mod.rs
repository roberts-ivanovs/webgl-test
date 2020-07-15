use std::fs;


pub fn fetch_f_shader() -> &'static str {
    // fs::read_to_string("./fsSource.glsl").unwrap()

    r#"
    void main() {
        gl_FragColor = vec4(1.0, 1.0, 1.0, 1.0);
    }
    "#
}
