use cgmath::Vector2;
use crate::entity::{Entity};

pub struct World{
    size: Vector2<f32>,
    objects: Vec::<Entity>,
    render_order: Vec::<i32>,
}

impl World{
    pub fn update() -> (){

    }

    pub fn render() -> (){
        
    }
}