use crate::protocol::src::packet::{Packet, PacketType};

pub struct Encoder;

impl Encoder {
    pub fn encode(packet: &Packet) -> Vec<u8> {
        let mut buffer = Vec::new();

        buffer.push(packet.version);
        buffer.push(packet.packet_type as u8);

        buffer.extend(&packet.flags.to_be_bytes());
        buffer.extend(&packet.length.to_be_bytes());
        buffer.extend(&packet.source.to_be_bytes());
        buffer.extend(&packet.destination.to_be_bytes());
        buffer.extend(&packet.payload);

        buffer
    }
}
