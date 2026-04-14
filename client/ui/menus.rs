pub struct Menu {
    pub title: String,
    pub items: Vec<String>,
}

impl Menu {
    pub fn new(title: &str) -> Self {
        Self {
            title: title.into(),
            items: Vec::new(),
        }
    }

    pub fn add_item(&mut self, item: &str) {
        self.items.push(item.into());
    }

    pub fn draw(&self) {
        println!("== {} ==", self.title);
        for item in &self.items {
            println!(" - {}", item);
        }
    }
}
