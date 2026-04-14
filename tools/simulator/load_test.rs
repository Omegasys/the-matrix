use std::time::Instant;
use crate::protocol::src::packet::{Packet, PacketType};

pub fn run_load_test(count: usize) {
    let start = Instant::now();

    for i in 0..count {
        let packet = Packet::new(
            PacketType::Scene,
            i as u128,
            999,
            vec![0; 1024],
        );

        // Simulate processing
        let _ = packet;
    }

    let duration = start.elapsed();
    println!(
        "[LOAD TEST] Processed {} packets in {:?}",
        count, duration
    );
}
