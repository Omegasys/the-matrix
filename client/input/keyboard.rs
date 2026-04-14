use std::collections::HashSet;
use winit::event::{KeyboardInput, VirtualKeyCode, ElementState};

pub struct Keyboard {
    pressed: HashSet<VirtualKeyCode>,
}

impl Keyboard {
    pub fn new() -> Self {
        Self {
            pressed: HashSet::new(),
        }
    }

    pub fn handle_input(&mut self, input: KeyboardInput) {
        if let Some(key) = input.virtual_keycode {
            match input.state {
                ElementState::Pressed => {
                    self.pressed.insert(key);
                }
                ElementState::Released => {
                    self.pressed.remove(&key);
                }
            }
        }
    }

    pub fn is_pressed(&self, key: VirtualKeyCode) -> bool {
        self.pressed.contains(&key)
    }

    pub fn clear(&mut self) {
        self.pressed.clear();
    }
}
