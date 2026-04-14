mod window;
mod event_loop;

use crate::core::utils::logger::{Logger, LogLevel};
use crate::core::utils::config::Config;

use window::Window;
use event_loop::AppEventLoop;

fn main() {
    // Load config
    let config = Config::load("config.cfg");

    // Setup logger
    let log_level = match config.log_level.as_str() {
        "debug" => LogLevel::Debug,
        "warn" => LogLevel::Warn,
        "error" => LogLevel::Error,
        _ => LogLevel::Info,
    };

    let logger = Logger::new(log_level);
    logger.info("Starting MatrixNet client...");

    // Create window
    let window = Window::new(
        "MatrixNet",
        config.window_width,
        config.window_height,
    );

    logger.info("Window created");

    // Run event loop
    AppEventLoop::run(window.event_loop, move || {
        // This will later:
        // - update scene
        // - render
        // - process networking

        // TEMP:
        // println!("Frame tick");
    });
}
