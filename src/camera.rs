use std::ops::Mul;

use cgmath::{Point3, Vector3, Matrix4};

pub struct Camera {
    pub eye: Point3<f32>,
    pub direction: Vector3<f32>,
    pub up: Vector3<f32>,
    pub perspective: Matrix4<f32>,
    pub view: Matrix4<f32>,
}

impl Camera{
    pub fn update_view(&mut self) {
        self.view = Matrix4::look_to_rh(self.eye, self.direction, self.up);
    }

    pub fn to_view_matrix(&self) -> [f32; 16]{
        *self.perspective.mul(self.view).as_ref()
    }
}
