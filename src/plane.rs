use cgmath::{Matrix4, Vector2, Vector3, SquareMatrix, Rad};

use crate::{input_handler::KeyState, model::Model, window_handler::GlContext};

pub struct Plane {
    body: Model,
    propeller: Model,
    position: Vector3<f32>,

    propeller_rotation: f32,
}

impl Plane {
    pub fn new(body: Model, propeller: Model, position: Vector3<f32>) -> Self {
        Self {
            body,
            propeller,
            position,
            propeller_rotation: 0.0,
        }
    }

    pub fn update(&self, key_state: &KeyState) -> () {

    }

    pub fn render(&mut self, gl: &GlContext, time: &f32, cam_per: &[f32; 16]) -> () {
        let mut matrix = Matrix4::identity();
        let translation = Matrix4::from_translation(self.position);

        matrix = translation * matrix;
        self.body.render(gl, matrix, time, cam_per);

        //0.1935

        let offset = Matrix4::from_translation(Vector3::<f32>::new(0.0, -0.1935, 0.0));
        let rev_offset = Matrix4::from_translation(Vector3::<f32>::new(0.0, 0.1935, 0.0));
        self.propeller_rotation += *time * 1000.0;

        let rotation = Matrix4::from_angle_z(Rad(self.propeller_rotation));

        let rotation_matrix = offset * rotation * rev_offset;
        matrix = translation * rotation_matrix;

        self.propeller.render(gl, matrix, time, cam_per);
    }
}
