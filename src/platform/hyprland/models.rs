use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Workspace {
    #[serde(default)]
    pub id: i32,

    #[serde(default)]
    pub name: String,

    #[serde(default, rename = "monitor")]
    pub monitor: String,

    #[serde(default, rename = "windows")]
    pub windows: u32,

    #[serde(default, rename = "hasfullscreen")]
    pub has_fullscreen: bool,
}
