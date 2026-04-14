#[derive(Debug, Clone)]
pub struct HapticFeedback {
    pub intensity: f32,
    pub duration_ms: u32,
    pub frequency: f32,
}

impl HapticFeedback {
    pub fn new(intensity: f32, duration_ms: u32, frequency: f32) -> Self {
        Self {
            intensity,
            duration_ms,
            frequency,
        }
    }
}
