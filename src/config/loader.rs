use super::Config;
use std::fs;

pub fn load() -> Config {
    let content = fs::read_to_string("config/config.toml")
        .expect("Failed to read config file");

    toml::from_str(&content)
        .expect("Failed to parse config")
}