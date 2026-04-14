use crate::networking::vpn::vpn_client::VPNClient;

pub struct MultiHopVPN {
    hops: Vec<VPNClient>,
}

impl MultiHopVPN {
    pub fn new() -> Self {
        Self { hops: Vec::new() }
    }

    pub fn add_hop(mut self, vpn: VPNClient) -> Self {
        self.hops.push(vpn);
        self
    }

    pub fn connect_all(&self) {
        for hop in &self.hops {
            hop.connect();
        }
    }

    pub fn route(&self, mut data: Vec<u8>) -> Vec<u8> {
        for hop in &self.hops {
            data = hop.send(&data);
        }
        data
    }

    pub fn unwrap(&self, mut data: Vec<u8>) -> Vec<u8> {
        for hop in self.hops.iter().rev() {
            data = hop.receive(&data);
        }
        data
    }
}
