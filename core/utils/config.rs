use std::fs;

#[derive(Debug, Clone)]
pub struct Config {
    pub window_width: u32,
    pub window_height: u32,
    pub fullscreen: bool,
    pub log_level: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            window_width: 1280,
            window_height: 720,
            fullscreen: false,
            log_level: "info".into(),
        }
    }
}

impl Config {
    pub fn load(path: &str) -> Self {
        match fs::read_to_string(path) {
            Ok(contents) => Self::parse(&contents),
            Err(_) => {
                println!("Config not found, using defaults");
                Self::default()
            }
        }
    }

    fn parse(contents: &str) -> Self {
        let mut config = Self::default();

        for line in contents.lines() {
            let parts: Vec<&str> = line.split('=').collect();
            if parts.len() != 2 {
                continue;
            }

            let key = parts[0].trim();
            let value = parts[1].trim();

            match key {
                "window_width" => config.window_width = value.parse().unwrap_or(1280),
                "window_height" => config.window_height = value.parse().unwrap_or(720),
                "fullscreen" => config.fullscreen = value == "true",
                "log_level" => config.log_level = value.to_string(),
                _ => {}
            }
        }

        config
    }
}
