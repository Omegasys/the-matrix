pub struct Overlay {
    pub name: String,
    pub active: bool,
}

impl Overlay {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.into(),
            active: false,
        }
    }

    pub fn show(&mut self) {
        self.active = true;
    }

    pub fn hide(&mut self) {
        self.active = false;
    }

    pub fn draw(&self) {
        if self.active {
            println!("[Overlay] {}", self.name);
        }
    }
}
