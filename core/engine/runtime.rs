use crate::engine::scheduler::Scheduler;
use crate::engine::resource_manager::ResourceManager;
use crate::engine::plugin_system::PluginSystem;

pub struct Runtime {
    scheduler: Scheduler,
    resources: ResourceManager,
    plugins: PluginSystem,
    running: bool,
}

impl Runtime {
    pub fn new() -> Self {
        Self {
            scheduler: Scheduler::new(),
            resources: ResourceManager::new(),
            plugins: PluginSystem::new(),
            running: false,
        }
    }

    pub fn start(&mut self) {
        self.running = true;
        self.plugins.initialize();

        while self.running {
            self.scheduler.tick();
            self.plugins.update();
        }
    }

    pub fn stop(&mut self) {
        self.running = false;
    }
}
