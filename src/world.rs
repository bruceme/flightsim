use cgmath::Vector2;
use glow::Context;
use crate::{entity::{Entity}, asset_manager::AssetManager};

pub struct World{
    objects: Vec::<Entity>,
    render_order: Vec::<i32>,
    asset_manager: AssetManager,
}

impl World{
    pub fn new() -> Self{
        Self{
            objects: Vec::new(),
            render_order: Vec::new(),
            asset_manager: AssetManager::new(),
        }
    }

    pub fn update(gl: Context) -> (){

    }

    pub fn render(gl: Context) -> (){
        
    }
}