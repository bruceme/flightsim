use std::{collections::HashMap, fs::File, io::BufReader, path::Path};
use glow::{Context, HasContext, NativeProgram};
use obj::{TexturedVertex, load_obj};

use crate::{model::Model, window_handler::GlContext};



pub struct AssetManager{
    loaded_shaders: HashMap<String, NativeProgram>,
}

impl AssetManager{
    pub fn new() -> Self{
        Self{
            loaded_shaders: HashMap::new(),
        }
    }

    fn load_shaders(&self, gl: GlContext, vert_name: &'static str, frag_name: &'static str) -> NativeProgram{
        if let Some(index) = self.loaded_shaders.get(vert_name) {
            return *index;
        }

        let program = unsafe {
            gl.create_program().expect("Cannot create program")
        };
        
        let vertex_shader_source = std::fs::read_to_string(vert_name).expect(&format!("Could not open file: {vert_name}"));
        let fragment_shader_source = std::fs::read_to_string(frag_name).expect(&format!("Could not open file: {frag_name}"));
        let (vertex_shader, fragment_shader) = unsafe{
            (gl.create_shader(glow::VERTEX_SHADER).expect("Cannot create shader"), gl.create_shader(glow::FRAGMENT_SHADER).expect("Cannot create shader"))
        };

        unsafe{
            gl.shader_source(vertex_shader, &vertex_shader_source);
            gl.compile_shader(vertex_shader);
            if !gl.get_shader_compile_status(vertex_shader) {
                panic!("{}", gl.get_shader_info_log(vertex_shader));
            }
            gl.attach_shader(program, vertex_shader);

            gl.shader_source(fragment_shader, &fragment_shader_source);
            gl.compile_shader(fragment_shader);
            if !gl.get_shader_compile_status(fragment_shader) {
                panic!("{}", gl.get_shader_info_log(fragment_shader));
            }
            gl.attach_shader(program, fragment_shader);

            gl.link_program(program);
            if !gl.get_program_link_status(program) {
                panic!("{}", gl.get_program_info_log(program));
            }

            gl.detach_shader(program, vertex_shader);
            gl.delete_shader(vertex_shader);
            gl.detach_shader(program, fragment_shader);
            gl.delete_shader(fragment_shader);
        };
        program
    }

    pub fn load_textures<P: AsRef<Path>>(&self, textures: &[P]){

    }

    pub fn load_obj(&self, gl: GlContext, file: impl AsRef<Path>, vert_shader: &'static str, frag_shader: &'static str) -> Model{
        let file_data = BufReader::new(File::open(file).expect("Could not open file: {file}"));
        let model = load_obj::<TexturedVertex, _, u32>(file_data).expect("Could not parse file to Model: {file}");

        let mut vert: Vec<[f32; 3]> = Vec::new();
        let mut tex: Vec<[f32; 2]> = Vec::new();
        let mut norm: Vec<[f32; 3]> = Vec::new();

        for vertex in model.vertices.into_iter(){
            vert.push(vertex.position);
            tex.push(vertex.texture[0..1].try_into().unwrap());
            norm.push(vertex.normal);
        }

        Model::new(gl.clone(), vert, tex, norm, model.indices, self.load_shaders(gl.clone(), vert_shader, frag_shader))
    }

    
}