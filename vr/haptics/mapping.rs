use crate::vr::haptics::feedback::HapticFeedback;

pub struct HapticMapper;

impl HapticMapper {
    pub fn collision(strength: f32) -> HapticFeedback {
        HapticFeedback::new(strength, 100, 120.0)
    }

    pub fn interaction() -> HapticFeedback {
        HapticFeedback::new(0.5, 50, 200.0)
    }

    pub fn ambient() -> HapticFeedback {
        HapticFeedback::new(0.1, 500, 60.0)
    }
}
