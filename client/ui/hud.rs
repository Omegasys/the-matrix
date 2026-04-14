pub struct Hud {
    pub visible: bool,
}

impl Hud {
    pub fn new() -> Self {
        Self { visible: true }
    }

    pub fn draw(&self) {
        if self.visible {
            println!("[HUD] Rendering HUD elements");
        }
    }
}
