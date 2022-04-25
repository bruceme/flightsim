use cgmath::Vector2;
use glow::Texture;

use crate::{model::Model, window_handler::GlContext, asset_manager::{self, AssetManager}};

pub struct Entity{
    position: Vector2<f32>,
    model: Model,
    gl: GlContext,
}

impl Entity{
    pub fn new(gl: &GlContext, asset_manager: &AssetManager, object_file: &str, vert_shader: &'static str, frag_shader: &'static str, texture_files: &[&str], position: Vector2<f32>)-> Self{
        Self{
            position,
            model: asset_manager.load_obj(gl, object_file, vert_shader, frag_shader, texture_files),
            gl: gl
        }
    }

    pub fn render(self){
        
    }
}