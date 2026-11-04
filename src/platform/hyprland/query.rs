use crate::platform::hyprland::models::Workspace;
use serde::Deserialize;
use std::env;
use std::io::{Read, Write};
use std::os::unix::net::UnixStream;

// Struct pembantu internal untuk deserialization ID active workspace
#[derive(Deserialize)]
struct ActiveWorkspaceResponse {
    id: i32,
}

pub fn workspaces() -> String {
    let signature =
        env::var("HYPRLAND_INSTANCE_SIGNATURE").expect("HYPRLAND_INSTANCE_SIGNATURE missing");

    let runtime = env::var("XDG_RUNTIME_DIR").expect("XDG_RUNTIME_DIR missing");

    let socket = format!("{}/hypr/{}/.socket.sock", runtime, signature);

    let mut stream = UnixStream::connect(socket).expect("Failed connect Hyprland socket");

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

    let mut result: Vec<Workspace> = match serde_json::from_str(&json) {
        Ok(parsed) => parsed,
        Err(err) => {
            eprintln!(
                "[kurovistop] Warning: Failed to parse workspace JSON: {}",
                err
            );
            Vec::new()
        }
    };

    result.retain(|ws| ws.id > 0);
    result.sort_by_key(|ws| ws.id);

    result
}


pub fn active_workspace() -> String {
    let signature =
        env::var("HYPRLAND_INSTANCE_SIGNATURE").expect("HYPRLAND_INSTANCE_SIGNATURE missing");

    let runtime = env::var("XDG_RUNTIME_DIR").expect("XDG_RUNTIME_DIR missing");

    let socket = format!("{}/hypr/{}/.socket.sock", runtime, signature);

    if let Ok(mut stream) = UnixStream::connect(socket) {
        if stream.write_all(b"j/activeworkspace").is_ok() {
            let mut response = String::new();
            if stream.read_to_string(&mut response).is_ok() {
                return response;
            }
        }
    }

    String::new()
}

pub fn get_active_workspace_id() -> i32 {
    let json = active_workspace();
    match serde_json::from_str::<ActiveWorkspaceResponse>(&json) {
        Ok(active) => active.id,
        Err(_) => 1, // Fallback default ke workspace 1 jika terjadi kegagalan parse/koneksi
    }
}