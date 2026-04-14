use crate::ai::api::ai_interface::{AIInterface, AIMessage};

pub struct Assistant<T: AIInterface> {
    pub ai: T,
}

impl<T: AIInterface> Assistant<T> {
    pub fn new(ai: T) -> Self {
        Self { ai }
    }

    pub fn update(&self) {
        if let Some(msg) = self.ai.receive() {
            println!("[Assistant] Received: {}", msg.content);

            let reply = AIMessage {
                content: format!("Echo: {}", msg.content),
            };

            self.ai.send(reply);
        }
    }
}
