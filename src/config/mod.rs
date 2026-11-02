pub mod loader;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub taskbar: TaskbarConfig,
}

#[derive(Debug, Deserialize)]
pub struct TaskbarConfig {
    pub position: String,
    pub height: i32,
    pub theme: String,
}