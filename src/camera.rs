use cgmath::{Point3, Vector3};

pub struct Camera {
    pub eye: Point3<f32>,
    pub direction: Vector3<f32>,
    pub up: Vector3<f32>,
}
