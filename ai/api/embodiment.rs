use crate::client::renderer::scene_graph::Transform;

#[derive(Clone)]
pub struct Embodiment {
    pub id: u64,
    pub name: String,
    pub transform: Transform,
    pub visible: bool,
}

impl Embodiment {
    pub fn new(id: u64, name: &str) -> Self {
        Self {
            id,
            name: name.into(),
            transform: Transform::default(),
            visible: true,
        }
    }

    pub fn move_to(&mut self, position: [f32; 3]) {
        self.transform.position = position;
    }

    pub fn translate(&mut self, delta: [f32; 3]) {
        for i in 0..3 {
            self.transform.position[i] += delta[i];
        }
    }
}
