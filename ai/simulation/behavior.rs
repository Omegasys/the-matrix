use crate::ai::api::embodiment::Embodiment;

pub trait Behavior {
    fn update(&mut self, entity: &mut Embodiment);
}

pub struct IdleBehavior;

impl Behavior for IdleBehavior {
    fn update(&mut self, _entity: &mut Embodiment) {
        // Do nothing
    }
}
