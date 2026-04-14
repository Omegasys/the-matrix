use winit::event::{MouseButton, ElementState};

pub struct Mouse {
    pub position: (f64, f64),
    pub delta: (f64, f64),
    pub buttons: Vec<MouseButton>,
}

impl Mouse {
    pub fn new() -> Self {
        Self {
            position: (0.0, 0.0),
            delta: (0.0, 0.0),
            buttons: Vec::new(),
        }
    }

    pub fn update_position(&mut self, x: f64, y: f64) {
        self.delta = (x - self.position.0, y - self.position.1);
        self.position = (x, y);
    }

    pub fn handle_button(&mut self, button: MouseButton, state: ElementState) {
        match state {
            ElementState::Pressed => {
                if !self.buttons.contains(&button) {
                    self.buttons.push(button);
                }
            }
            ElementState::Released => {
                self.buttons.retain(|b| b != &button);
            }
        }
    }

    pub fn is_pressed(&self, button: MouseButton) -> bool {
        self.buttons.contains(&button)
    }

    pub fn reset_delta(&mut self) {
        self.delta = (0.0, 0.0);
    }
}
