pub struct Camera {
    pub position: [f32; 3],
    pub rotation: [f32; 3],
    pub fov: f32,
    pub near: f32,
    pub far: f32,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            position: [0.0, 0.0, 5.0],
            rotation: [0.0, 0.0, 0.0],
            fov: 70.0,
            near: 0.1,
            far: 1000.0,
        }
    }
}

impl Camera {
    pub fn move_forward(&mut self, amount: f32) {
        self.position[2] -= amount;
    }

    pub fn move_right(&mut self, amount: f32) {
        self.position[0] += amount;
    }
}
