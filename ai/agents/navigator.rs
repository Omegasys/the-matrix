use crate::ai::api::embodiment::Embodiment;

pub struct Navigator;

impl Navigator {
    pub fn move_towards(entity: &mut Embodiment, target: [f32; 3], speed: f32) {
        let mut direction = [0.0; 3];

        for i in 0..3 {
            direction[i] = target[i] - entity.transform.position[i];
        }

        let magnitude = (direction[0].powi(2)
            + direction[1].powi(2)
            + direction[2].powi(2)).sqrt();

        if magnitude > 0.0 {
            for i in 0..3 {
                direction[i] /= magnitude;
                entity.transform.position[i] += direction[i] * speed;
            }
        }
    }
}
