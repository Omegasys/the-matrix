use std::{thread, time::Duration};
use crate::protocol::src::packet::Packet;

pub struct NetworkSimulator {
    pub latency_ms: u64,
}

impl NetworkSimulator {
    pub fn new(latency_ms: u64) -> Self {
        Self { latency_ms }
    }

    pub fn transmit(&self, packet: Packet) -> Packet {
        println!("[SIM] Transmitting packet with latency {}ms", self.latency_ms);

        thread::sleep(Duration::from_millis(self.latency_ms));

        packet
    }
}
