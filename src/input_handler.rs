use glutin::event::{ElementState, KeyboardInput, VirtualKeyCode};

pub struct InputHandler {
    key_state: KeyState,
}

#[derive(Debug)]
pub struct KeyState {
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
    pub accelerate: bool,
    pub escape: bool,
    pub turbo: bool,
}

impl InputHandler {
    pub fn new() -> Self {
        Self {
            key_state: KeyState {
                up: false,
                down: false,
                left: false,
                right: false,
                accelerate: false,
                escape: false,
                turbo: false,
            },
        }
    }

    pub fn key_pressed(&mut self, input: &KeyboardInput) {
        if let Some(vk) = input.virtual_keycode {
            match vk {
                VirtualKeyCode::W => {
                    self.key_state.up = input.state == ElementState::Pressed;
                }
                VirtualKeyCode::S => {
                    self.key_state.down = input.state == ElementState::Pressed;
                }
                VirtualKeyCode::A => {
                    self.key_state.left = input.state == ElementState::Pressed;
                }
                VirtualKeyCode::D => {
                    self.key_state.right = input.state == ElementState::Pressed;
                }
                VirtualKeyCode::Space | VirtualKeyCode::LControl => {
                    self.key_state.accelerate = input.state == ElementState::Pressed;
                }
                VirtualKeyCode::Escape => {
                    if input.state == ElementState::Released {
                        self.key_state.escape = !self.key_state.escape;
                    }
                }

                VirtualKeyCode::T => {
                    self.key_state.turbo = input.state == ElementState::Pressed;
                }
                _ => (),
            }
        }
    }

    pub fn get_key_state(&self) -> &KeyState {
        &self.key_state
    }
}
