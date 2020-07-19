use core::f32::consts::PI;
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
    aspect: f32,
    field_of_view: f32,
    trans_x: f32,
    trans_y: f32,
    trans_z: f32,
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
            field_of_view: (45. * PI / 180.) as f32,
            aspect: 4. / 3. as f32,
            trans_x: 0.,
            trans_y: 0.,
            trans_z: -6.,
        }
    }

    fn init_buffers(gl: &GL) -> WebGlBuffer {
        let position_buffer = gl.create_buffer().unwrap();
        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&position_buffer));
        let positions = [-1., 1., 1., 1., -1., -1., 1., -1.];

        unsafe {
            let vert_array = js_sys::Float32Array::view(&positions);

            gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &vert_array, GL::STATIC_DRAW);
        }
        position_buffer
    }

    pub fn draw_scene(&self, gl: &GL) {
        gl.clear_color(0., 0., 0., 1.);
        gl.clear_depth(1.);
        gl.enable(GL::DEPTH_TEST);
        gl.depth_func(GL::LEQUAL);
        gl.clear(GL::COLOR_BUFFER_BIT | GL::DEPTH_BUFFER_BIT);

        let z_near: f32 = 0.1;
        let z_far: f32 = 100.0;

        let projection_matrix = glm::perspective(self.aspect, self.field_of_view, z_near, z_far);
        let mut empty_matrix = glm::mat4x4(
            0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
        );
        empty_matrix.fill_with_identity();
        let translation_vector = glm::vec3(self.trans_x, self.trans_y, self.trans_z);
        let model_view_matrix = glm::translate(&empty_matrix, &translation_vector);

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
            offset,
        );
        gl.enable_vertex_attrib_array(self.attribute_locations.vertex_position as u32);

        gl.use_program(Some(&self.program));
        let transpose = false;
        gl.uniform_matrix4fv_with_f32_array(
            Some(&self.uniform_locations.projection_matrix),
            transpose,
            projection_matrix.as_slice(),
        );
        gl.uniform_matrix4fv_with_f32_array(
            Some(&self.uniform_locations.model_view_matrix),
            transpose,
            model_view_matrix.as_slice(),
        );

        let offset = 0;
        let vertex_count = 4;
        gl.draw_arrays(GL::TRIANGLE_STRIP, offset, vertex_count);
    }

    pub fn set_flied_of_view(&mut self, degrees: f32) {
        self.field_of_view = (degrees * PI / 180.) as f32;
    }

    pub fn set_aspect(&mut self, width: f32, height: f32) {
        self.aspect = width / height as f32;
    }

    pub fn set_trans_x(&mut self, new_x: f32) {
        self.trans_x = new_x
    }

    pub fn set_trans_y(&mut self, new_y: f32) {
        self.trans_y = new_y
    }

    pub fn set_trans_z(&mut self, new_z: f32) {
        self.trans_z = new_z
    }

    pub fn get_trans_x(&self) -> f32 {
        self.trans_x
    }

    pub fn get_trans_y(&self) -> f32 {
        self.trans_y
    }

    pub fn get_trans_z(&self) -> f32 {
        self.trans_z
    }


}
