use cgmath::{Vector3};
use crate::{entity::{Entity}, asset_manager::{AssetManager, self}, window_handler::GlContext, input_handler::KeyState};

pub struct World{
    gl: GlContext,
    objects: Vec::<Entity>,
    render_order: Vec::<i32>,
    asset_manager: AssetManager,
}

impl World{
    pub fn new(gl: &GlContext) -> Self{
        let mut objects = Vec::<Entity>::new();
        let asset_manager =AssetManager::new();
        let skybox = Entity::new(gl, &asset_manager, "assets/skybox/skybox.obj", "assets/skybox/skybox.vert", "assets/skybox/skybox.frag", &["assets/skybox/skybox.png"], Vector3::new(0.0, 0.0, 0.0));

        objects.push(skybox);

        Self{
            gl: gl.clone(),
            objects,
            render_order: Vec::new(),
            asset_manager,
        }
    }

    pub fn update(&self, key_state: &KeyState) -> (){
        self.objects.iter().for_each(|object| object.update(key_state));
    }

    pub fn render(&self, time: &f32, cam_per: &[f32; 16]) -> (){
        self.objects.iter().for_each(|object| object.render(&self.gl, time, cam_per));
    }
}