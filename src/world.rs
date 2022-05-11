use crate::{
    asset_manager::{self, AssetManager},
    entity::Entity,
    input_handler::KeyState,
    window_handler::GlContext, mesh_factory::MeshFactory,
};
use cgmath::Vector3;
use glow::HasContext;

pub struct World {
    gl: GlContext,
    objects: Vec<Entity>,
    asset_manager: AssetManager,
    skybox: Entity,
}

impl World {
    pub fn new(gl: &GlContext) -> Self {
        let mut objects = Vec::<Entity>::new();
        let asset_manager =AssetManager::new();
        let skybox = Entity::new(&gl, &asset_manager, "assets/skybox/skybox.obj", "assets/skybox/skybox.vert", "assets/skybox/skybox.frag", &["assets/skybox/skybox.png"], Vector3::new(0.0, 0.0, 0.0));

        let surface = Entity::new_obj(&gl, &asset_manager, MeshFactory::generate_surface("assets/skybox/skybox.png", 0.1, 0.1), "assets/surface/surface.vert", "assets/surface/surface.frag", &["assets/surface/surface.png"], Vector3::new(0.0, -20.0, 0.0));
        objects.push(surface);

        Self {
            gl: gl.clone(),
            objects,
            asset_manager,
            skybox
        }
    }

    pub fn update(&self, key_state: &KeyState) -> () {
        self.objects
            .iter()
            .for_each(|object| object.update(key_state));
    }

    pub fn render(&self, time: &f32, cam_per: &[f32; 16]) -> (){
        self.skybox.render(&self.gl, time, cam_per);
        unsafe{
            self.gl.clear(glow::DEPTH_BUFFER_BIT);
        }

        self.objects.iter().for_each(|object| object.render(&self.gl, time, cam_per));
    }
}
