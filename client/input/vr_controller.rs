
#[derive(Debug, Clone)]
pub struct VrControllerState {
    pub position: [f32; 3],
    pub rotation: [f32; 3],
    pub trigger_pressed: bool,
    pub grip_pressed: bool,
}

pub struct VrController {
    pub left: VrControllerState,
    pub right: VrControllerState,
}

impl VrController {
    pub fn new() -> Self {
        Self {
            left: Self::default_state(),
            right: Self::default_state(),
        }
    }

    fn default_state() -> VrControllerState {
        VrControllerState {
            position: [0.0, 0.0, 0.0],
            rotation: [0.0, 0.0, 0.0],
            trigger_pressed: false,
            grip_pressed: false,
        }
    }

    pub fn update_mock(&mut self) {
        // Placeholder for real OpenXR input
        self.left.position[0] += 0.001;
        self.right.position[0] -= 0.001;
    }
}
