use cgmath::{Matrix4, Vector2, Vector3};

use crate::{input_handler::KeyState, model::Model, window_handler::GlContext};

pub struct Plane {
    body: Model,
    propeller: Model,
    position: Vector3<f32>,
}

impl Plane {
    pub fn new(body: Model, propeller: Model, position: Vector3<f32>) -> Self {
        Self {
            body,
            propeller,
            position,
        }
    }

    pub fn update(&self, key_state: &KeyState) -> () {}

    pub fn render(&self, gl: &GlContext, time: &f32, cam_per: &[f32; 16]) -> () {
        let matrix = Matrix4::from_translation(self.position);
        self.body.render(gl, matrix, time, cam_per);
        self.propeller.render(gl, matrix, time, cam_per);
    }
}
