use std::convert::identity;

use cgmath::{Matrix4, Vector2, Vector3, SquareMatrix, Rad, Deg, Vector4, InnerSpace, Matrix3};

use crate::{input_handler::KeyState, model::Model, window_handler::GlContext, camera::Camera};

pub struct Plane {
    body: Model,
    propeller: Model,
    
    position: Vector3<f32>,

    max_speed: f32,
    
    forward: Vector3::<f32>,
    up: Vector3::<f32>,
    right: Vector3::<f32>,

    speed: f32,
    acceleration: f32,

    propeller_rotation: f32,

    camera_offset: Vector3<f32>
}

impl Plane {
    pub const PITCH_SPEED: f32 = 0.01;
    pub const ROLL_SPEED: f32 = 0.01;

    pub fn new(body: Model, propeller: Model, position: Vector3<f32>) -> Self {
        Self {
            body,
            propeller,
            position,

            max_speed: 1.5,
            
            forward: Vector3::new(0.0, 0.0, -1.0),
            up: Vector3::new(0.0, 1.0, 0.0),
            right: Vector3::new(1.0, 0.0, 0.0),
            
            speed: 0.1,
            acceleration: 0.05,

            propeller_rotation: 0.0,
            camera_offset: Vector3::new(0.0, 1.0, -2.0),
        }
    }

    fn pitch(&mut self, angle: f32) {
        self.forward = (self.forward * angle.cos() + self.up * angle.sin()).normalize();
        self.up = self.right.cross(self.forward);

    }

    fn roll(&mut self, angle: f32) {
        self.right = (self.right * angle.cos() + self.up * angle.sin()).normalize();
        self.up = self.right.cross(self.forward);
    }

    pub fn update(&mut self, key_state: &KeyState) -> () {

        //pitch
        if key_state.up {
            self.pitch(Self::PITCH_SPEED);
        }

        if key_state.down {
            self.pitch(-Self::PITCH_SPEED);
        }

        if key_state.left {
            self.roll(Self::ROLL_SPEED);
        }

        if key_state.right {
            self.roll(-Self::ROLL_SPEED);
        }
        self.speed -= 0.001;
        if key_state.accelerate{
            self.speed += 0.005;
        }

        self.speed = self.speed.clamp(0.0, self.max_speed);

        self.position += self.forward * self.speed;
    }

    pub fn render(&mut self, gl: &GlContext, time: &f32, cam_per: &[f32; 16], camera: &mut Camera) -> () {
        let mut matrix = Matrix4::identity();
        let translation = Matrix4::from_translation(self.position);

        let right = self.right;
        let forward = self.forward;
        let up = self.up;

        let plane_rot = Matrix4::new(
            right.x, right.y, right.z, 0.0,
            up.x, up.y, up.z, 0.0,
            forward.x, forward.y, forward.z,0.0,
            0.0, 0.0, 0.0, 1.0,
        );
        
        matrix = translation * plane_rot * matrix;
        self.body.render(gl, matrix, time, cam_per);

        let offset = Matrix4::from_translation(Vector3::<f32>::new(0.0, -0.1935, 0.0));
        let rev_offset = Matrix4::from_translation(Vector3::<f32>::new(0.0, 0.1935, 0.0));
        self.propeller_rotation += *time * 1000.0;

        let rotation = Matrix4::from_angle_z(Rad(self.propeller_rotation));

        //camera.eye = Vector4::new(0.0, 0.0, 0.0, 1.0).xyz().into();
        let spin = offset * rotation * rev_offset;
        matrix = translation * plane_rot * spin;

        self.propeller.render(gl, matrix, time, cam_per);
    }
}
