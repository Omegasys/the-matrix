#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct IPv8Address {
    pub node_id: String,
    pub space_id: String, // 3D space / world identifier
}

impl IPv8Address {
    pub fn new(node_id: &str, space_id: &str) -> Self {
        Self {
            node_id: node_id.to_string(),
            space_id: space_id.to_string(),
        }
    }
}
