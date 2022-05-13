use std::convert::identity;

use cgmath::{Matrix4, Vector2, Vector3, SquareMatrix, Rad, Deg, Vector4, InnerSpace};

use crate::{input_handler::KeyState, model::Model, window_handler::GlContext, camera::Camera};

pub struct Plane {
    body: Model,
    propeller: Model,
    
    position: Vector3<f32>,

    pitch: f32,
    pitch_speed: f32,
    
    roll: f32,
    roll_speed: f32,

    max_speed: f32,
    
    forward: Vector3::<f32>,
    up: Vector3::<f32>,
    left: Vector3::<f32>,

    speed: f32,
    acceleration: f32,

    propeller_rotation: f32,

    camera_offset: Vector3<f32>
}

impl Plane {
    pub fn new(body: Model, propeller: Model, position: Vector3<f32>) -> Self {
        Self {
            body,
            propeller,
            position,
            
            pitch: 0.0,
            pitch_speed: 1.00,
            
            roll: 0.0,
            roll_speed: 1.00,

            max_speed: 1.5,
            
            forward: Vector3::new(0.0, 0.0, 0.0),
            up: Vector3::new(0.0, 0.0, 0.0),
            left: Vector3::new(0.0, 0.0, 0.0),
            
            speed: 0.1,
            acceleration: 0.05,

            propeller_rotation: 0.0,
            camera_offset: Vector3::new(0.0, 1.0, -2.0),
        }
    }

    pub fn update(&mut self, key_state: &KeyState) -> () {

        // pub struct KeyState {
        //     pub up: bool,
        //     pub down: bool,
        //     pub left: bool,
        //     pub right: bool,
        //     pub accelerate: bool,
        //     pub escape: bool,
        // }

        //pitch
        if key_state.up {
            self.pitch = self.pitch + self.pitch_speed;
        }

        if key_state.down {
            self.pitch = self.pitch - self.pitch_speed;
        }

        if key_state.left {
            self.roll = self.roll - self.roll_speed;
        }

        if key_state.right {
            self.roll = self.roll + self.roll_speed;
        }
        self.speed -= 0.001;
        if key_state.accelerate{
            self.speed += 0.005;
        }

        self.speed = self.speed.clamp(0.0, self.max_speed);

        let pitch =  Matrix4::from_angle_x(Deg(self.pitch));
        let roll =  Matrix4::from_angle_z(Deg(self.roll));

        self.forward = (roll * pitch * Vector4::new(0.0, 0.0, 1.0, 0.0)).xyz().normalize();
        self.up = (roll * pitch * Vector4::new(0.0, 1.0, 0.0, 0.0)).xyz().normalize();
        self.left = (roll * pitch * Vector4::new(1.0, 0.0, 0.0, 0.0)).xyz().normalize();

        println!("{:?}, {:?}, {:?}", self.left, self.up, self.forward);

        self.position += self.forward * self.speed;
    }

    pub fn render(&mut self, gl: &GlContext, time: &f32, cam_per: &[f32; 16], camera: &mut Camera) -> () {
        let mut matrix = Matrix4::identity();
        let translation = Matrix4::from_translation(self.position);


        
        let pitch =  Matrix4::from_axis_angle(self.left, Deg(self.pitch));
        let roll =  Matrix4::from_axis_angle(self.forward, Deg(self.roll));

        let plane_rot = roll * pitch;

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
