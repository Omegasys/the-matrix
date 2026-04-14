use crate::ai::api::embodiment::Embodiment;
use rand::Rng;

pub struct AutonomousAgent;

impl AutonomousAgent {
    pub fn wander(entity: &mut Embodiment) {
        let mut rng = rand::thread_rng();

        let dx = rng.gen_range(-0.05..0.05);
        let dz = rng.gen_range(-0.05..0.05);

        entity.translate([dx, 0.0, dz]);
    }
}
