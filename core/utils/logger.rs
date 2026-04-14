use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Copy)]
pub enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
}

#[derive(Clone)]
pub struct Logger {
    level: LogLevel,
    inner: Arc<Mutex<()>>,
}

impl Logger {
    pub fn new(level: LogLevel) -> Self {
        Self {
            level,
            inner: Arc::new(Mutex::new(())),
        }
    }

    fn should_log(&self, level: LogLevel) -> bool {
        use LogLevel::*;
        match (self.level, level) {
            (Debug, _) => true,
            (Info, Debug) => false,
            (Info, _) => true,
            (Warn, Error) | (Warn, Warn) => true,
            (Error, Error) => true,
            _ => false,
        }
    }

    fn timestamp() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
    }

    pub fn log(&self, level: LogLevel, msg: &str) {
        if !self.should_log(level) {
            return;
        }

        let _lock = self.inner.lock().unwrap();
        println!(
            "[{}][{:?}] {}",
            Self::timestamp(),
            level,
            msg
        );
    }

    pub fn debug(&self, msg: &str) {
        self.log(LogLevel::Debug, msg);
    }

    pub fn info(&self, msg: &str) {
        self.log(LogLevel::Info, msg);
    }

    pub fn warn(&self, msg: &str) {
        self.log(LogLevel::Warn, msg);
    }

    pub fn error(&self, msg: &str) {
        self.log(LogLevel::Error, msg);
    }
}
