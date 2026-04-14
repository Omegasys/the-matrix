use crate::protocol::src::packet::{Packet, PacketType};

pub struct Decoder;

impl Decoder {
    pub fn decode(data: &[u8]) -> Result<Packet, String> {
        if data.len() < 1 + 1 + 2 + 4 + 16 + 16 {
            return Err("Packet too small".into());
        }

        let version = data[0];
        let packet_type = match data[1] {
            1 => PacketType::Scene,
            2 => PacketType::Mesh,
            3 => PacketType::Texture,
            4 => PacketType::Control,
            _ => return Err("Invalid packet type".into()),
        };

        let flags = u16::from_be_bytes([data[2], data[3]]);
        let length = u32::from_be_bytes([data[4], data[5], data[6], data[7]]);

        let source = u128::from_be_bytes(data[8..24].try_into().unwrap());
        let destination = u128::from_be_bytes(data[24..40].try_into().unwrap());

        let payload = data[40..].to_vec();

        Ok(Packet {
            version,
            packet_type,
            flags,
            length,
            source,
            destination,
            payload,
        })
    }
}
