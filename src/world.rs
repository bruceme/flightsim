use cgmath::{Vector2, Vector3};
use glow::Context;
use obj::raw::object;
use crate::{entity::{Entity}, asset_manager::{AssetManager, self}, window_handler::GlContext};

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
        let skybox = Entity::new(gl, &asset_manager, "assets/skybox/untitled.obj", "assets/skybox/skybox.vert", "assets/skybox/skybox.frag", &["assets/skybox/skybox512.png"], Vector3::new(0.0, 0.0, 0.0));

        objects.push(skybox);

        Self{
            gl: gl.clone(),
            objects,
            render_order: Vec::new(),
            asset_manager,
        }
    }

    pub fn update(&self) -> (){

    }

    pub fn render(&self, time: &f32, cam_per: &[f32; 16]) -> (){
        self.objects.iter().for_each(|object| object.render(&self.gl, time, cam_per));
    }
}