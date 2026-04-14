use std::time::{Duration, Instant};

pub struct Scheduler {
    last_tick: Instant,
    tick_rate: Duration,
}

impl Scheduler {
    pub fn new() -> Self {
        Self {
            last_tick: Instant::now(),
            tick_rate: Duration::from_millis(16), // ~60 FPS
        }
    }

    pub fn tick(&mut self) {
        let now = Instant::now();
        if now.duration_since(self.last_tick) >= self.tick_rate {
            self.last_tick = now;
            // Execute scheduled tasks here
        }
    }
}
