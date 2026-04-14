use std::convert::TryInto;

#[derive(Debug, Clone, Copy)]
pub enum PacketType {
    Scene = 1,
    Mesh = 2,
    Texture = 3,
    Control = 4,
}

#[derive(Debug, Clone)]
pub struct Packet {
    pub version: u8,
    pub packet_type: PacketType,
    pub flags: u16,
    pub length: u32,
    pub source: u128,
    pub destination: u128,
    pub payload: Vec<u8>,
}

impl Packet {
    pub fn new(
        packet_type: PacketType,
        source: u128,
        destination: u128,
        payload: Vec<u8>,
    ) -> Self {
        Self {
            version: 1,
            packet_type,
            flags: 0,
            length: payload.len() as u32,
            source,
            destination,
            payload,
        }
    }

    pub fn set_flag(&mut self, bit: u16) {
        self.flags |= bit;
    }

    pub fn has_flag(&self, bit: u16) -> bool {
        self.flags & bit != 0
    }
}
