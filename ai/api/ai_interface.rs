use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
pub struct AIMessage {
    pub content: String,
}

pub trait AIInterface: Send + Sync {
    fn send(&self, msg: AIMessage);
    fn receive(&self) -> Option<AIMessage>;
}

/// Simple local AI bridge (mock for now)
pub struct LocalAI {
    inbox: Arc<Mutex<Vec<AIMessage>>>,
}

impl LocalAI {
    pub fn new() -> Self {
        Self {
            inbox: Arc::new(Mutex::new(Vec::new())),
        }
    }
}

impl AIInterface for LocalAI {
    fn send(&self, msg: AIMessage) {
        self.inbox.lock().unwrap().push(msg);
    }

    fn receive(&self) -> Option<AIMessage> {
        self.inbox.lock().unwrap().pop()
    }
}
