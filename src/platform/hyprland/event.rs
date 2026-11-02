use std::env;
use std::io::{BufRead, BufReader};
use std::os::unix::net::UnixStream;

pub fn listen(sender: std::sync::mpsc::Sender<String>) {
    let signature =
        env::var("HYPRLAND_INSTANCE_SIGNATURE").expect("HYPRLAND_INSTANCE_SIGNATURE missing");

    let runtime = env::var("XDG_RUNTIME_DIR").expect("XDG_RUNTIME_DIR missing");

    let socket = format!("{}/hypr/{}/.socket2.sock", runtime, signature);

    let stream = UnixStream::connect(socket).expect("Failed connect event socket");

    let reader = BufReader::new(stream);

    for line in reader.lines() {
        if let Ok(event) = line {
            sender.send(event).ok();
        }
    }
}
