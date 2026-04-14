#[derive(Debug, Clone)]
pub struct XrDevice {
    pub name: String,
    pub connected: bool,
}

impl XrDevice {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.into(),
            connected: false,
        }
    }

    pub fn connect(&mut self) {
        println!("[XR] Device connected: {}", self.name);
        self.connected = true;
    }

    pub fn disconnect(&mut self) {
        println!("[XR] Device disconnected: {}", self.name);
        self.connected = false;
    }
}
