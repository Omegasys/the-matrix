use crate::vr::haptics::feedback::HapticFeedback;

pub struct HapticSuit {
    pub connected: bool,
}

impl HapticSuit {
    pub fn new() -> Self {
        Self { connected: false }
    }

    pub fn connect(&mut self) {
        println!("[HAPTICS] Suit connected");
        self.connected = true;
    }

    pub fn send_feedback(&self, feedback: HapticFeedback) {
        if self.connected {
            println!(
                "[HAPTICS] intensity={} duration={} freq={}",
                feedback.intensity,
                feedback.duration_ms,
                feedback.frequency
            );
        }
    }
}
