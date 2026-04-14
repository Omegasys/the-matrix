#[derive(Debug, Clone)]
pub struct HapticEvent {
    pub intensity: f32,
    pub duration_ms: u32,
}

pub struct Haptics;

impl Haptics {
    pub fn send(event: HapticEvent) {
        // Placeholder for real hardware
        println!(
            "[HAPTIC] intensity={} duration={}ms",
            event.intensity, event.duration_ms
        );
    }
}
