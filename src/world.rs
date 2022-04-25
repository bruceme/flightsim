use cgmath::Vector2;
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
        let skybox = Entity::new(gl, &asset_manager, "assets/skybox/skybox.obj", "assets/skybox/skybox.vert", "assets/skybox/skybox.frag", &["assets/skybox/skybox.tga"], Vector2::new(0.0, 0.0));

        objects.push(skybox);

        Self{
            gl: gl,
            objects,
            render_order: Vec::new(),
            asset_manager,
        }
    }

    pub fn update(&self) -> (){

    }

    pub fn render(&self) -> (){
        
    }
}