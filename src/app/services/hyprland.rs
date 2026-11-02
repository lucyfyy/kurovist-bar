use std::sync::mpsc;

pub fn start() {
    let (sender, receiver) = mpsc::channel::<String>();

    std::thread::spawn(move || {
        crate::platform::hyprland::event::listen(sender);
    });

    glib::idle_add_local(move || {
        while let Ok(event) = receiver.try_recv() {
            println!("GTK EVENT: {}", event);

            let event_name =
                event.split(">>")
                    .next()
                    .unwrap_or("");

            match event_name {
                "workspace"
                | "openwindow"
                | "closewindow"
                | "destroyworkspace" => {
                    println!("REFRESH WORKSPACE");
                }

                _ => {}
            }
        }

        glib::ControlFlow::Continue
    });
}