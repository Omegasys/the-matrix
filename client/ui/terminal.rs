pub struct Terminal {
    buffer: Vec<String>,
}

impl Terminal {
    pub fn new() -> Self {
        Self { buffer: Vec::new() }
    }

    pub fn log(&mut self, msg: &str) {
        self.buffer.push(msg.into());
    }

    pub fn draw(&self) {
        println!("--- Terminal ---");
        for line in &self.buffer {
            println!("{}", line);
        }
    }
}
