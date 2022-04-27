use std::rc::Rc;
use cgmath::Matrix4;
use glow::{Context, NativeProgram, HasContext, NativeBuffer, NativeVertexArray, Texture, NO_ERROR};

use crate::{window_handler::GlContext, helper::AsRawBytes};

pub struct Model{
    pub vertices: Vec<[f32; 3]>,
    pub texies: Vec<[f32; 2]>,
    pub normals: Vec<[f32; 3]>,
    pub indices: Vec<u32>,

    vao: NativeVertexArray,

    program: NativeProgram,
    textures: Vec<Texture>,
}

impl Model{

    pub fn new(gl: &GlContext, vertices: Vec<[f32;3]>, texies: Vec<[f32; 2]>, normals: Vec<[f32; 3]>, indices: Vec<u32>, program: NativeProgram, textures: Vec::<Texture>) -> Self{  
        
        let vao = unsafe {
            let vao = gl.create_vertex_array().unwrap();
            gl.bind_vertex_array(Some(vao));

            let vertices_buf = gl.create_buffer().unwrap();
            gl.bind_buffer(glow::ARRAY_BUFFER, Some(vertices_buf));
            gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, vertices.as_raw_bytes(), glow::STATIC_DRAW);
            gl.vertex_attrib_pointer_f32(gl.get_attrib_location(program, "vertices").unwrap(), 3, glow::FLOAT, false, 0, 0);
            gl.enable_vertex_attrib_array(gl.get_attrib_location(program, "vertices").unwrap());


            if let Some(texture_location) = gl.get_attrib_location(program, "texies"){
                let texies_buf = gl.create_buffer().unwrap();
                gl.bind_buffer(glow::ARRAY_BUFFER, Some(texies_buf));
                gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, texies.as_raw_bytes(), glow::STATIC_DRAW);
                gl.vertex_attrib_pointer_f32(texture_location, 2, glow::FLOAT, false, 0, 0);
                gl.enable_vertex_attrib_array(texture_location);
            };
            
            
            if let Some(normals_location) = gl.get_attrib_location(program, "normals"){
                let normals_buf = gl.create_buffer().unwrap();
                gl.bind_buffer(glow::ARRAY_BUFFER, Some(normals_buf));
                gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, normals.as_raw_bytes(), glow::STATIC_DRAW);
                gl.vertex_attrib_pointer_f32(normals_location, 3, glow::FLOAT, false, 0, 0);
                gl.enable_vertex_attrib_array(normals_location);
            }
            

            let indices_buf = gl.create_buffer().unwrap();
            gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, Some(indices_buf));
            gl.buffer_data_u8_slice(glow::ELEMENT_ARRAY_BUFFER, indices.as_raw_bytes(), glow::STATIC_DRAW);

            vao
        };
        
        Self{
            vertices,
            texies,
            normals,
            indices,
            vao,
            program,
            textures
        }
    }


    pub fn render(&self, gl: &Context, matrix: Matrix4<f32>, time: &f32, cam_per: &[f32; 16]) {
        let m: [f32; 16] = *matrix.as_ref();
        unsafe{
            gl.bind_vertex_array(Some(self.vao));
            gl.use_program(Some(self.program));
            for (i, texture) in self.textures.iter().enumerate(){
                gl.active_texture(glow::TEXTURE0 + i as u32);
                gl.bind_texture(glow::TEXTURE0 + i as u32, Some(*texture));
                gl.uniform_1_i32(gl.get_uniform_location(self.program, &format!("texture{}", i)).as_ref(), i as i32);
            }
            gl.uniform_1_f32(gl.get_uniform_location(self.program, "time").as_ref(), *time);
            gl.uniform_matrix_4_f32_slice(gl.get_uniform_location(self.program, "view").as_ref(), false,  cam_per);
            gl.draw_elements(glow::TRIANGLES, self.indices.len() as i32, glow::UNSIGNED_INT, 0);   
        }
    }
}

impl Drop for Model {
    fn drop(&mut self) {

    }
}

fn generic_slice_as_u8_slice<T: Copy>(slice: &[T]) -> &[u8] {
    unsafe {
        let len = slice.len() * std::mem::size_of::<T>();
        let ptr = std::mem::transmute::<_, *mut u8>(slice.as_ptr());
        std::slice::from_raw_parts_mut(ptr, len)
    }
}