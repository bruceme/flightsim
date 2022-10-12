use crate::{
    camera::Camera,
    entity::Entity,
    input_handler::KeyState,
    mesh_factory::{self},
    plane::Plane,
    water::Water,
    window_handler::GlContext,
};
use cgmath::{Point3, Vector3};
use glow::HasContext;

pub struct World {
    gl: GlContext,
    objects: Vec<Entity>,
    plane: Plane,
    water: Water,
    skybox: Entity,
}

impl World {
    pub fn new(gl: &GlContext) -> Self {
        let mut objects = Vec::<Entity>::new();
        let skybox = Entity::new(
            gl,
            "assets/skybox/skybox.obj",
            "assets/skybox/skybox.vert",
            "assets/skybox/skybox.frag",
            &["assets/skybox/skybox.png"],
            Vector3::new(0.0, 0.0, 0.0),
        );

        let ground_image = "assets/surface/surface.png";
        let scale = 5.0;
        let height_ext = 1.0;
        let ground_position = Vector3::<f32>::new(-1250.0 * scale, -300.0 * height_ext, -1250.0 * scale);

        let surface = Entity::new_obj(
            gl,
            mesh_factory::generate_surface(ground_image, scale, height_ext),
            "assets/surface/surface.vert",
            "assets/surface/surface.frag",
            &["assets/surface/surface_texture.png"],
            ground_position,
        );
        objects.push(surface);

        let size = imagesize::size(ground_image).unwrap();
        let water = Water::new(gl, (size.width, size.height), scale, height_ext, ground_position);

        let plane = Plane::new(
            Vector3::new(0.0, -2.0, -5.0),
        );

        Self {
            gl: gl.clone(),
            objects,
            plane,
            skybox,
            water,
        }
    }

    pub fn update(&mut self, key_state: &KeyState) {
        self.plane.update(key_state);
    }

    pub fn render(&mut self, time: &f32, camera: &mut Camera) {
        camera.eye = Point3::new(0.01, 0.0, 0.01);
        camera.update_view();
        self.skybox.render(&self.gl, time, camera);
        unsafe {
            self.gl.clear(glow::DEPTH_BUFFER_BIT);
        }
        self.plane.render(&self.gl, camera);
        self.objects
            .iter()
            .for_each(|object| object.render(&self.gl, time, camera));
        self.water.render(&self.gl, time, camera);
    }
}
