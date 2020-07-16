use nalgebra_glm::U1;
use crate::utils::console_log;
use core::f64::consts::PI;
use js_sys::{ArrayBuffer, Float32Array};
use nalgebra_glm as glm;

use web_sys::WebGlBuffer;
use web_sys::WebGlProgram;
use web_sys::WebGlRenderingContext as GL;
use web_sys::WebGlUniformLocation;

pub struct AttributeLocations {
    pub vertex_position: i32,
}

pub struct UniformLocations {
    projection_matrix: WebGlUniformLocation,
    model_view_matrix: WebGlUniformLocation,
}

pub struct Box2D {
    buffer: WebGlBuffer,
    program: WebGlProgram,
    attribute_locations: AttributeLocations,
    uniform_locations: UniformLocations,
}

impl Box2D {
    pub fn new(gl: &GL, program: WebGlProgram) -> Self {
        // program.
        let attribute_locations = AttributeLocations {
            vertex_position: gl.get_attrib_location(&program, "aVertexPosition"),
        };
        let uniform_locations = UniformLocations {
            projection_matrix: gl
                .get_uniform_location(&program, "uProjectionMatrix")
                .unwrap(),
            model_view_matrix: gl
                .get_uniform_location(&program, "uModelViewMatrix")
                .unwrap(),
        };

        let buffer = Box2D::init_buffers(&gl);

        Self {
            buffer,
            attribute_locations,
            uniform_locations,
            program,
        }
    }

    fn init_buffers(gl: &GL) -> WebGlBuffer {
        let position_buffer = gl.create_buffer().unwrap();
        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&position_buffer));
        let positions = [-1., 1., 1., 1., -1., -1., 1., -1.];

        unsafe {
        let vert_array = js_sys::Float32Array::view(&positions);

            gl.buffer_data_with_array_buffer_view(
                GL::ARRAY_BUFFER,
                &vert_array,
                GL::STATIC_DRAW,
            );
        }
        position_buffer
    }

    pub fn draw_scene(&self, gl: &GL) {
        gl.clear_color(0., 0., 0., 1.);
        gl.clear_depth(1.);
        gl.enable(GL::DEPTH_TEST);
        gl.depth_func(GL::LEQUAL);
        gl.clear(GL::COLOR_BUFFER_BIT | GL::DEPTH_BUFFER_BIT);

        let field_of_view: f32 = (45. * PI / 180.) as f32;
        let aspect: f32 = 4. / 3.;
        // let aspect = gl.canvas().unwrap().client_width / gl.canvas().unwrap().clientHeight;
        let z_near: f32 = 0.1;
        let z_far: f32 = 100.0;

        let projection_matrix = glm::perspective(aspect, field_of_view, z_near, z_far);
        let mut empty_matrix = glm::mat4x4(
            0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
        );
        empty_matrix.fill_with_identity();
        let translation_vector = glm::vec3(0.0, 0.0, -6.0);
        let model_view_matrix= glm::translate(&empty_matrix, &translation_vector);
        console_log(format!("translation_vector: {:?}", translation_vector).as_ref());
        console_log(format!("model_view_matrix: {:?}", model_view_matrix).as_ref());
        console_log(format!("projection_matrix: {:?}", projection_matrix).as_ref());

        let number_components = 2;
        let buffer_type = GL::FLOAT;
        let normalize = false;
        let stride = 0;
        let offset = 0;

        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&self.buffer));
        gl.vertex_attrib_pointer_with_i32(
            self.attribute_locations.vertex_position as u32,
            number_components,
            buffer_type,
            normalize,
            stride,
            offset
        );
        gl.enable_vertex_attrib_array(self.attribute_locations.vertex_position as u32);

        gl.use_program(Some(&self.program));
        let transpose = false;
        gl.uniform_matrix4fv_with_f32_array(Some(&self.uniform_locations.projection_matrix), transpose, projection_matrix.as_slice());
        gl.uniform_matrix4fv_with_f32_array(Some(&self.uniform_locations.model_view_matrix), transpose, model_view_matrix.as_slice());

        let offset = 0;
        let vertex_count = 4;
        gl.draw_arrays(GL::TRIANGLE_STRIP, offset, vertex_count);
    }
}
