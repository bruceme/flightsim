use cgmath::{EuclideanSpace, InnerSpace, Matrix4, Point3, Rad, Vector3, Vector4, Deg};

use crate::{camera::Camera, input_handler::KeyState, model::Model, window_handler::GlContext};

pub struct Plane {
    body: Model,
    propeller: Model,

    scale: f32,
    position: Vector3<f32>,

    forward: Vector3<f32>,
    up: Vector3<f32>,
    right: Vector3<f32>,

    pitch_velocity: f32,
    roll_velocity: f32,

    velocity: f32,

    propeller_rotation: f32,
    camera_offset: Vector3<f32>,
}

impl Plane {
    pub const PITCH_SPEED: f32 = 0.00005;
    pub const ROLL_SPEED: f32 = 0.0001;

    pub fn new(body: Model, propeller: Model, position: Vector3<f32>) -> Self {
        Self {
            body,
            propeller,

            scale: 0.25,
            position,

            forward: Vector3::new(0.0, 0.0, -1.0),
            up: Vector3::new(0.0, 1.0, 0.0),
            right: Vector3::new(1.0, 0.0, 0.0),

            pitch_velocity: 0.0,
            roll_velocity: 0.0,

            velocity: 0.1,
            propeller_rotation: 0.0,
            camera_offset: Vector3::new(0.0, 3.0, -15.0),
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

    pub fn update(&mut self, key_state: &KeyState) {
        //pitch
        if key_state.up {
            self.pitch_velocity -= Self::PITCH_SPEED;
        }

        if key_state.down {
            self.pitch_velocity += Self::PITCH_SPEED;
        }

        if key_state.left {
            self.roll_velocity += Self::ROLL_SPEED;
        }

        if key_state.right {
            self.roll_velocity -= Self::ROLL_SPEED;
        }

        self.pitch_velocity *= 1.0 - 0.01;
        self.roll_velocity *= 1.0 - 0.01;

        self.pitch(self.pitch_velocity);
        self.roll(self.roll_velocity);

        self.velocity -= 0.001;
        if key_state.accelerate {
            self.velocity += 0.005;
        }

        self.velocity = self.velocity.clamp(0.5, 2.0);
        self.position += self.forward * self.velocity;
    }

    pub fn render(&mut self, gl: &GlContext, time: &f32, camera: &mut Camera) {
        camera.perspective.fovy = Deg(80.0 + self.velocity * 20.0).into();
        let translation = Matrix4::from_translation(self.position);

        let right = self.right;
        let forward = self.forward;
        let up = self.up;

        let plane_rot = Matrix4::new(
            right.x, right.y, right.z, 0.0, up.x, up.y, up.z, 0.0, forward.x, forward.y, forward.z,
            0.0, 0.0, 0.0, 0.0, 1.0,
        );

        let scale_matrix = Matrix4::from_scale(self.scale);
        let mut matrix = translation * plane_rot * scale_matrix;
        let camera_position = (matrix
            * Matrix4::from_translation(self.camera_offset)
            * Vector4::<f32>::new(0.0, 0.0, 0.0, 1.0))
        .xyz();
        camera.eye = Point3::from_vec(camera_position);
        camera.direction = self.forward;
        camera.up = self.up;
        camera.update_view();
        self.body.render(gl, matrix, time, &camera.to_view_matrix());

        let offset = Matrix4::from_translation(Vector3::<f32>::new(0.0, -0.1935, 0.0));
        let rev_offset = Matrix4::from_translation(Vector3::<f32>::new(0.0, 0.1935, 0.0));
        self.propeller_rotation += *time * 1000.0;

        let rotation = Matrix4::from_angle_z(Rad(self.propeller_rotation));

        let spin = offset * rotation * rev_offset;
        matrix = matrix * spin;

        self.propeller
            .render(gl, matrix, time, &camera.to_view_matrix());
    }
}
