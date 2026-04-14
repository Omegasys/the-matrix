use std::collections::HashMap;

pub struct Multiplexer {
    channels: HashMap<u32, Vec<u8>>,
}

impl Multiplexer {
    pub fn new() -> Self {
        Self {
            channels: HashMap::new(),
        }
    }

    pub fn send(&mut self, channel_id: u32, data: Vec<u8>) {
        self.channels.insert(channel_id, data);
    }

    pub fn receive(&mut self, channel_id: u32) -> Option<Vec<u8>> {
        self.channels.remove(&channel_id)
    }
}
