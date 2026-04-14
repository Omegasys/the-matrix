pub trait Plugin {
    fn initialize(&mut self);
    fn update(&mut self);
}

pub struct PluginSystem {
    plugins: Vec<Box<dyn Plugin>>,
}

impl PluginSystem {
    pub fn new() -> Self {
        Self { plugins: Vec::new() }
    }

    pub fn register(&mut self, plugin: Box<dyn Plugin>) {
        self.plugins.push(plugin);
    }

    pub fn initialize(&mut self) {
        for plugin in self.plugins.iter_mut() {
            plugin.initialize();
        }
    }

    pub fn update(&mut self) {
        for plugin in self.plugins.iter_mut() {
            plugin.update();
        }
    }
}
