pub struct TraceEvent {
    pub stage: String,
    pub timestamp: u128,
}

pub struct TraceViewer {
    events: Vec<TraceEvent>,
}

impl TraceViewer {
    pub fn new() -> Self {
        Self { events: Vec::new() }
    }

    pub fn record(&mut self, stage: &str, timestamp: u128) {
        self.events.push(TraceEvent {
            stage: stage.into(),
            timestamp,
        });
    }

    pub fn print(&self) {
        println!("--- Trace Viewer ---");
        for e in &self.events {
            println!("[{}] {}", e.timestamp, e.stage);
        }
    }
}
