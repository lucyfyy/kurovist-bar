use std::env;
use std::io::Write;
use std::os::unix::net::UnixStream;


pub fn dispatch(command: &str) {
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
        .expect("Failed connect Hyprland command socket");

    stream
        .write_all(command.as_bytes())
        .expect("Failed send command");
}


pub fn workspace(id: i32) {
    let command = format!(
        "dispatch workspace {}",
        id
    );

    dispatch(&command);
}