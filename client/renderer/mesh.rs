#[derive(Clone)]
pub struct Vertex {
    pub position: [f32; 3],
    pub normal: [f32; 3],
    pub uv: [f32; 2],
}

#[derive(Clone)]
pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
}

impl Mesh {
    pub fn new(vertices: Vec<Vertex>, indices: Vec<u32>) -> Self {
        Self { vertices, indices }
    }

    pub fn cube() -> Self {
        // Minimal placeholder cube (expand later)
        let vertices = vec![
            Vertex { position: [-1.0, -1.0,  1.0], normal: [0.0, 0.0, 1.0], uv: [0.0, 0.0] },
            Vertex { position: [ 1.0, -1.0,  1.0], normal: [0.0, 0.0, 1.0], uv: [1.0, 0.0] },
            Vertex { position: [ 1.0,  1.0,  1.0], normal: [0.0, 0.0, 1.0], uv: [1.0, 1.0] },
            Vertex { position: [-1.0,  1.0,  1.0], normal: [0.0, 0.0, 1.0], uv: [0.0, 1.0] },
        ];

        let indices = vec![0, 1, 2, 2, 3, 0];

        Self { vertices, indices }
    }
}
