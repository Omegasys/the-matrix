use std::collections::HashMap;
use crate::networking::ip_layer::addressing::IPv8Address;

pub struct RoutingTable {
    routes: HashMap<IPv8Address, String>, // maps address → next hop
}

impl RoutingTable {
    pub fn new() -> Self {
        Self { routes: HashMap::new() }
    }

    pub fn add_route(&mut self, dest: IPv8Address, next_hop: String) {
        self.routes.insert(dest, next_hop);
    }

    pub fn get_next_hop(&self, dest: &IPv8Address) -> Option<&String> {
        self.routes.get(dest)
    }
}
