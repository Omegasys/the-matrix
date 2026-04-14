use crate::protocol::src::packet::Packet;

pub fn inspect(packet: &Packet) {
    println!("--- Packet Inspector ---");
    println!("Version: {}", packet.version);
    println!("Type: {:?}", packet.packet_type);
    println!("Flags: {}", packet.flags);
    println!("Length: {}", packet.length);
    println!("Source: {}", packet.source);
    println!("Destination: {}", packet.destination);
    println!("Payload size: {}", packet.payload.len());
}
