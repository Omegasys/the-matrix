#[derive(Debug, Clone)]
pub struct Pose {
    pub position: [f32; 3],
    pub rotation: [f32; 3],
}

pub struct TrackingSystem {
    pub head: Pose,
    pub left_hand: Pose,
    pub right_hand: Pose,
}

impl TrackingSystem {
    pub fn new() -> Self {
        Self {
            head: Self::default_pose(),
            left_hand: Self::default_pose(),
            right_hand: Self::default_pose(),
        }
    }

    fn default_pose() -> Pose {
        Pose {
            position: [0.0, 0.0, 0.0],
            rotation: [0.0, 0.0, 0.0],
        }
    }

    pub fn update_mock(&mut self) {
        // Placeholder movement
        self.head.position[1] = 1.6;
    }
}
