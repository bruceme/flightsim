use cgmath::{Matrix4, PerspectiveFov, Point3, Vector3};

pub struct Camera {
    pub eye: Point3<f32>,
    pub direction: Vector3<f32>,
    pub up: Vector3<f32>,
    pub perspective: PerspectiveFov<f32>,
    pub view: Matrix4<f32>,
}

impl Camera {
    pub fn update_view(&mut self) {
        self.view = Matrix4::look_to_rh(self.eye, self.direction, self.up);
    }

    pub fn to_view_matrix(&self) -> [f32; 16] {
        unsafe {
            std::mem::transmute_copy::<_, [f32; 16]>(
                &(Matrix4::from(self.perspective.to_perspective()) * self.view),
            )
        }
    }
}
