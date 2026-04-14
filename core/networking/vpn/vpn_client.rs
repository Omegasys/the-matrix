pub struct VPNClient {
    pub endpoint: String,
}

impl VPNClient {
    pub fn new(endpoint: &str) -> Self {
        Self { endpoint: endpoint.to_string() }
    }

    pub fn connect(&self) {
        println!("Connecting to VPN at {}", self.endpoint);
    }

    pub fn send(&self, data: &[u8]) -> Vec<u8> {
        let mut out = b"VPN".to_vec();
        out.extend_from_slice(data);
        out
    }

    pub fn receive(&self, data: &[u8]) -> Vec<u8> {
        data.get(3..).unwrap_or(&[]).to_vec()
    }
}
