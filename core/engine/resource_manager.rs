use std::collections::HashMap;

pub struct ResourceManager {
    resources: HashMap<String, Vec<u8>>,
}

impl ResourceManager {
    pub fn new() -> Self {
        Self {
            resources: HashMap::new(),
        }
    }

    pub fn load(&mut self, name: &str, data: Vec<u8>) {
        self.resources.insert(name.to_string(), data);
    }

    pub fn get(&self, name: &str) -> Option<&Vec<u8>> {
        self.resources.get(name)
    }
}
