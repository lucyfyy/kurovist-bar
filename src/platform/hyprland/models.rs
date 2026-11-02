use serde::Deserialize;


#[derive(Debug, Deserialize)]
pub struct Workspace {
    pub id: i32,
    pub windows: i32,
}