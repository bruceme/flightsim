use cgmath::{Matrix4, Vector3};

use crate::{
    asset_manager::{AssetManager, Object},
    input_handler::KeyState,
    model::Model,
    window_handler::GlContext,
};

pub struct Entity {
    position: Vector3<f32>,
    model: Model,
}

impl Entity {
    pub fn new(
        gl: &GlContext,
        asset_manager: &AssetManager,
        object_file: &str,
        vert_shader: &'static str,
        frag_shader: &'static str,
        texture_files: &[&str],
        position: Vector3<f32>,
    ) -> Self {
        Self {
            position,
            model: asset_manager.load_obj(gl, object_file, vert_shader, frag_shader, texture_files),
        }
    }

    pub fn new_obj(
        gl: &GlContext,
        asset_manager: &AssetManager,
        object: Object,
        vert_shader: &'static str,
        frag_shader: &'static str,
        texture_files: &[&str],
        position: Vector3<f32>,
    ) -> Self {
        Self {
            position,
            model: asset_manager.load_obj_preloaded(
                gl,
                object,
                vert_shader,
                frag_shader,
                texture_files,
            ),
        }
    }

    pub fn update(&self, _key_state: &KeyState) {}

    pub fn render(&self, gl: &GlContext, time: &f32, cam_per: &[f32; 16]) {
        let matrix = Matrix4::from_translation(self.position);
        self.model.render(gl, matrix, time, cam_per);
    }
}
