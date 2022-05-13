use glow::{HasContext, NativeProgram, Texture};
use obj::{load_obj, TexturedVertex};
use std::{fs::File, io::BufReader, path::Path};

use crate::{model::Model, window_handler::GlContext};

mod load_image;

pub type Object = (Vec<[f32; 3]>, Vec<[f32; 2]>, Vec<[f32; 3]>, Vec<u32>);

pub struct AssetManager {}

impl AssetManager {
    pub fn new() -> Self {
        Self {}
    }

    fn load_shaders(
        &self,
        gl: GlContext,
        vert_name: &'static str,
        frag_name: &'static str,
    ) -> NativeProgram {
        let program = unsafe { gl.create_program().expect("Cannot create program") };

        let vertex_shader_source = std::fs::read_to_string(vert_name)
            .unwrap_or_else(|_| panic!("Could not open file: {vert_name}"));
        let fragment_shader_source = std::fs::read_to_string(frag_name)
            .unwrap_or_else(|_| panic!("Could not open file: {frag_name}"));
        let (vertex_shader, fragment_shader) = unsafe {
            (
                gl.create_shader(glow::VERTEX_SHADER)
                    .expect("Cannot create shader"),
                gl.create_shader(glow::FRAGMENT_SHADER)
                    .expect("Cannot create shader"),
            )
        };

        unsafe {
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

    pub fn load_textures<P: AsRef<Path>>(&self, gl: &GlContext, textures: &[P]) -> Vec<Texture> {
        let mut loaded_textures = Vec::<Texture>::new();
        for texture_name in textures.iter() {
            let image =
                load_image::load_texture(texture_name).expect("Texture could not load properly");
            loaded_textures.push(unsafe {
                let texture = gl.create_texture().unwrap();
                gl.bind_texture(glow::TEXTURE_2D, Some(texture));
                gl.tex_image_2d(
                    glow::TEXTURE_2D,
                    0,
                    glow::RGBA8 as i32,
                    image.width() as i32,
                    image.height() as i32,
                    0,
                    glow::RGBA,
                    glow::UNSIGNED_BYTE,
                    Some(image.as_raw()),
                );
                gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_WRAP_S, glow::REPEAT as i32);
                gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_WRAP_T, glow::REPEAT as i32);
                gl.tex_parameter_i32(
                    glow::TEXTURE_2D,
                    glow::TEXTURE_MIN_FILTER,
                    glow::LINEAR as i32,
                );
                gl.tex_parameter_i32(
                    glow::TEXTURE_2D,
                    glow::TEXTURE_MAG_FILTER,
                    glow::LINEAR as i32,
                );
                texture
            });
        }
        loaded_textures
    }

    pub fn load_obj(
        &self,
        gl: &GlContext,
        file: impl AsRef<Path>,
        vert_shader: &'static str,
        frag_shader: &'static str,
        texture_files: &[&str],
    ) -> Model {
        let file_data = BufReader::new(File::open(file).expect("Could not open file"));
        let model =
            load_obj::<TexturedVertex, _, u32>(file_data).expect("Could not parse file to Model");

        let mut vert: Vec<[f32; 3]> = Vec::new();
        let mut tex: Vec<[f32; 2]> = Vec::new();
        let mut norm: Vec<[f32; 3]> = Vec::new();

        for vertex in model.vertices.into_iter() {
            vert.push(vertex.position);
            tex.push([vertex.texture[0], vertex.texture[1]]);
            norm.push(vertex.normal);
        }

        let textures = self.load_textures(gl, texture_files);

        Model::new(
            &gl.clone(),
            vert,
            tex,
            norm,
            model.indices,
            self.load_shaders(gl.clone(), vert_shader, frag_shader),
            textures,
        )
    }

    pub fn load_obj_preloaded(
        &self,
        gl: &GlContext,
        object: Object,
        vert_shader: &'static str,
        frag_shader: &'static str,
        texture_files: &[&str],
    ) -> Model {
        let (vert, tex, norm, ind) = object;
        let textures = self.load_textures(gl, texture_files);
        Model::new(
            &gl.clone(),
            vert,
            tex,
            norm,
            ind,
            self.load_shaders(gl.clone(), vert_shader, frag_shader),
            textures,
        )
    }
}
