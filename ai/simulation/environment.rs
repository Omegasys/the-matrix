use crate::ai::api::embodiment::Embodiment;
use crate::ai::simulation::behavior::Behavior;

pub struct Entity {
    pub embodiment: Embodiment,
    pub behavior: Box<dyn Behavior>,
}

pub struct Environment {
    pub entities: Vec<Entity>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            entities: Vec::new(),
        }
    }

    pub fn add_entity(&mut self, entity: Entity) {
        self.entities.push(entity);
    }

    pub fn update(&mut self) {
        for entity in &mut self.entities {
            entity.behavior.update(&mut entity.embodiment);
        }
    }
}
