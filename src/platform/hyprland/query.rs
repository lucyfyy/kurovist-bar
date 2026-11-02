use std::env;
use std::io::{Read, Write};
use std::os::unix::net::UnixStream;
use crate::platform::hyprland::models::Workspace;

pub fn workspaces() -> String {

    let signature = env::var("HYPRLAND_INSTANCE_SIGNATURE")
        .expect("HYPRLAND_INSTANCE_SIGNATURE missing");

    let runtime = env::var("XDG_RUNTIME_DIR")
        .expect("XDG_RUNTIME_DIR missing");


    let socket = format!(
        "{}/hypr/{}/.socket.sock",
        runtime,
        signature
    );


    let mut stream = UnixStream::connect(socket)
        .expect("Failed connect Hyprland socket");


    stream
        .write_all(b"j/workspaces")
        .expect("Failed send query");


    let mut response = String::new();

    stream
        .read_to_string(&mut response)
        .expect("Failed read response");


    response
}

pub fn get_workspaces() -> Vec<Workspace> {

    let json = workspaces();

    let mut result: Vec<Workspace> =
        serde_json::from_str(&json)
        .expect("Failed parse workspace JSON");


    result.retain(|ws| {
        ws.id > 0
    });


    result.sort_by_key(|ws| ws.id);


    result
}