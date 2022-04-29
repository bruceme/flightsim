use glutin::event::{KeyboardInput, VirtualKeyCode, ElementState};

pub struct InputHandler{
    key_state: KeyState
}

pub struct KeyState{
    up: bool,
    down: bool,
    left: bool,
    right: bool,
    accelerate: bool,
}

impl InputHandler{
    pub fn new() -> Self{
        Self{
            key_state: KeyState{
                up: false,
                down: false,
                left: false,
                right: false,
                accelerate: false,
            }
        }
    }

    pub fn key_pressed(&mut self, input: &KeyboardInput){
        if let Some(vk) = input.virtual_keycode{
            match vk {
                VirtualKeyCode::W =>{
                    self.key_state.up = input.state == ElementState::Pressed;
                }
                VirtualKeyCode::A =>{
                    self.key_state.down = input.state == ElementState::Pressed;
                }
                VirtualKeyCode::S =>{
                    self.key_state.left = input.state == ElementState::Pressed;
                }
                VirtualKeyCode::D=>{
                    self.key_state.right = input.state == ElementState::Pressed;
                },
                VirtualKeyCode::Space | VirtualKeyCode::LControl =>{
                    self.key_state.accelerate = input.state == ElementState::Pressed;
                }
                _=> ()
            }
        }
    }

    pub fn get_key_state(&self) -> &KeyState{
        &self.key_state
    }
}
