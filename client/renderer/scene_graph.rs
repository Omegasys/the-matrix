use crate::client::renderer::mesh::Mesh;
use crate::client::renderer::camera::Camera;
use crate::client::renderer::lighting::Light;

#[derive(Clone)]
pub struct Transform {
    pub position: [f32; 3],
    pub rotation: [f32; 3],
    pub scale: [f32; 3],
}

impl Default for Transform {
    fn default() -> Self {
        Self {
            position: [0.0, 0.0, 0.0],
            rotation: [0.0, 0.0, 0.0],
            scale: [1.0, 1.0, 1.0],
        }
    }
}

pub struct Node {
    pub id: u64,
    pub name: String,
    pub transform: Transform,
    pub mesh: Option<Mesh>,
    pub children: Vec<Node>,
}

impl Node {
    pub fn new(id: u64, name: &str) -> Self {
        Self {
            id,
            name: name.into(),
            transform: Transform::default(),
            mesh: None,
            children: Vec::new(),
        }
    }

    pub fn add_child(&mut self, node: Node) {
        self.children.push(node);
    }
}

pub struct Scene {
    pub root: Node,
    pub camera: Camera,
    pub lights: Vec<Light>,
}

impl Scene {
    pub fn new() -> Self {
        Self {
            root: Node::new(0, "root"),
            camera: Camera::default(),
            lights: Vec::new(),
        }
    }

    pub fn update(&mut self) {
        // future: animations, physics, AI hooks
    }
}
