use crate::{
    asset_manager::{AssetManager},
    entity::Entity,
    input_handler::KeyState,
    mesh_factory::MeshFactory,
    plane::Plane,
    window_handler::GlContext, camera::Camera,
};
use cgmath::{Vector3, Point3};
use glow::HasContext;

pub struct World {
    gl: GlContext,
    objects: Vec<Entity>,
    plane: Plane,
    asset_manager: AssetManager,
    skybox: Entity,
}

impl World {
    pub fn new(gl: &GlContext) -> Self {
        let mut objects = Vec::<Entity>::new();
        let asset_manager = AssetManager::new();
        let skybox = Entity::new(
            &gl,
            &asset_manager,
            "assets/skybox/skybox.obj",
            "assets/skybox/skybox.vert",
            "assets/skybox/skybox.frag",
            &["assets/skybox/skybox.png"],
            Vector3::new(0.0, 0.0, 0.0),
        );

        let surface = Entity::new_obj(
            &gl,
            &asset_manager,
            MeshFactory::generate_surface("assets/surface/surface.png", 10.0, 2.0),
            "assets/surface/surface.vert",
            "assets/surface/surface.frag",
            &["assets/surface/surface.png"],
            Vector3::new(0.0, 0.0, 0.0),
        );
        objects.push(surface);

        let plane = Plane::new(
            asset_manager.load_obj(
                &gl,
                "assets/plane/body.obj",
                "assets/plane/body.vert",
                "assets/plane/body.frag",
                &["assets/plane/plane_mirror_y.png"],
            ),
            asset_manager.load_obj(
                &gl,
                "assets/plane/propeller.obj",
                "assets/plane/propeller.vert",
                "assets/plane/propeller.frag",
                &["assets/plane/plane_mirror_y.png"],
            ),
            Vector3::new(0.0, -2.0, -5.0),
        );

        Self {
            gl: gl.clone(),
            objects,
            plane,
            asset_manager,
            skybox,
        }
    }

    pub fn update(&mut self, key_state: &KeyState) -> () {
        self.objects
            .iter()
            .for_each(|object| object.update(key_state));
        self.plane.update(key_state);
    }

    pub fn render(&mut self, time: &f32, camera: &mut Camera) -> () {
        camera.eye = Point3::new(0.01, 0.0, 0.01);
        camera.update_view();
        self.skybox.render(&self.gl, time, &camera.to_view_matrix());
        unsafe {
            self.gl.clear(glow::DEPTH_BUFFER_BIT);
        }
        self.plane.render(&self.gl, time, camera);

        self.objects
            .iter()
            .for_each(|object| object.render(&self.gl, time, &camera.to_view_matrix()));
        
        
    }
}
