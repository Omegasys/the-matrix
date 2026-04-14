use crate::networking::ip_layer::packet::Packet;
use crate::networking::ip_layer::routing::RoutingTable;
use crate::networking::ip_layer::addressing::IPv8Address;

pub struct IPv8 {
    routing: RoutingTable,
}

impl IPv8 {
    pub fn new() -> Self {
        Self {
            routing: RoutingTable::new(),
        }
    }

    pub fn send(&self, packet: Packet, dest: &IPv8Address) {
        if let Some(next_hop) = self.routing.get_next_hop(dest) {
            println!("Routing packet to next hop: {}", next_hop);
            // send via transport layer (to be integrated)
        } else {
            println!("No route found for destination");
        }
    }

    pub fn receive(&self, packet: Packet) {
        println!("Received packet: {:?}", packet.header);
        // process payload
    }
}
