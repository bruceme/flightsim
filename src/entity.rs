use cgmath::Vector2;

use crate::model::Model;

pub struct Entity{
    id: u32,
    position: Vector2<f32>,
    program: i32,
    model: Model,
}

impl Entity{
    pub fn new(position: Vector2<f32>, vert_shader: &str, frag_shader: &str){
        
        
        
    }
}