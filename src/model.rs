use cgmath::Matrix4;
use glow::{Context, HasContext, NativeProgram, NativeVertexArray, Texture};

use crate::camera::Camera;
use crate::mesh::Mesh;
use crate::{helper::AsRawBytes, window_handler::GlContext};

pub struct Model {
    pub mesh: Mesh,
    vao: NativeVertexArray,
    program: NativeProgram,
    textures: Vec<Texture>,
}

impl Model {
    pub fn new(gl: &GlContext, mesh: Mesh, program: NativeProgram, textures: Vec<Texture>) -> Self {
        let vao = unsafe {
            let vao = gl.create_vertex_array().unwrap();
            gl.bind_vertex_array(Some(vao));

            let vertices_buf = gl.create_buffer().unwrap();
            gl.bind_buffer(glow::ARRAY_BUFFER, Some(vertices_buf));
            gl.buffer_data_u8_slice(
                glow::ARRAY_BUFFER,
                mesh.vertices.as_raw_bytes(),
                glow::STATIC_DRAW,
            );
            gl.vertex_attrib_pointer_f32(
                gl.get_attrib_location(program, "position").unwrap(),
                3,
                glow::FLOAT,
                false,
                0,
                0,
            );
            gl.enable_vertex_attrib_array(gl.get_attrib_location(program, "position").unwrap());

            if let Some(texture_location) = gl.get_attrib_location(program, "tex") {
                let texies_buf = gl.create_buffer().unwrap();
                gl.bind_buffer(glow::ARRAY_BUFFER, Some(texies_buf));
                gl.buffer_data_u8_slice(
                    glow::ARRAY_BUFFER,
                    mesh.texies.as_raw_bytes(),
                    glow::STATIC_DRAW,
                );
                gl.vertex_attrib_pointer_f32(texture_location, 2, glow::FLOAT, false, 0, 0);
                gl.enable_vertex_attrib_array(texture_location);
            };

            if let Some(normals_location) = gl.get_attrib_location(program, "normal") {
                let normals_buf = gl.create_buffer().unwrap();
                gl.bind_buffer(glow::ARRAY_BUFFER, Some(normals_buf));
                gl.buffer_data_u8_slice(
                    glow::ARRAY_BUFFER,
                    mesh.normals.as_raw_bytes(),
                    glow::STATIC_DRAW,
                );
                gl.vertex_attrib_pointer_f32(normals_location, 3, glow::FLOAT, false, 0, 0);
                gl.enable_vertex_attrib_array(normals_location);
            }

            let indices_buf = gl.create_buffer().unwrap();
            gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, Some(indices_buf));
            gl.buffer_data_u8_slice(
                glow::ELEMENT_ARRAY_BUFFER,
                mesh.indices.as_raw_bytes(),
                glow::STATIC_DRAW,
            );

            vao
        };

        Self {
            mesh,
            vao,
            program,
            textures,
        }
    }

    pub fn render(&self, gl: &Context, matrix: Matrix4<f32>, time: &f32, camera: &mut Camera) {
        let transformation: [f32; 16] = *matrix.as_ref();
        unsafe {
            gl.bind_vertex_array(Some(self.vao));
            gl.use_program(Some(self.program));
            for (i, texture) in self.textures.iter().enumerate() {
                gl.active_texture(glow::TEXTURE0 + i as u32);
                gl.enable(glow::TEXTURE_2D);
                gl.bind_texture(glow::TEXTURE_2D, Some(*texture));
                gl.uniform_1_i32(
                    gl.get_uniform_location(self.program, &format!("texture{}", i))
                        .as_ref(),
                    i as i32,
                );
            }
            gl.uniform_1_f32(
                gl.get_uniform_location(self.program, "time").as_ref(),
                *time,
            );
            gl.uniform_matrix_4_f32_slice(
                gl.get_uniform_location(self.program, "transformation")
                    .as_ref(),
                false,
                &transformation,
            );
            gl.uniform_matrix_4_f32_slice(
                gl.get_uniform_location(self.program, "projection_view").as_ref(),
                false,
                &camera.to_projection_view_matrix(),
            );
            gl.uniform_matrix_4_f32_slice(
                gl.get_uniform_location(self.program, "view").as_ref(),
                false,
                &camera.to_view_matrix(),
            );
            gl.draw_elements(
                glow::TRIANGLES,
                self.mesh.indices.len() as i32,
                glow::UNSIGNED_INT,
                0,
            );

            for (i, _) in self.textures.iter().enumerate() {
                gl.active_texture(glow::TEXTURE0 + i as u32);
                gl.bind_texture(glow::TEXTURE_2D, None);
            }
        }
    }
}

impl Drop for Model {
    fn drop(&mut self) {}
}
