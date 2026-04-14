use crate::vr::spatial::tracking::TrackingSystem;

pub struct Calibration;

impl Calibration {
    pub fn calibrate_origin(tracking: &mut TrackingSystem) {
        let origin = tracking.head.position;

        for p in [
            &mut tracking.head,
            &mut tracking.left_hand,
            &mut tracking.right_hand,
        ] {
            for i in 0..3 {
                p.position[i] -= origin[i];
            }
        }

        println!("[VR] Calibration complete");
    }
}
