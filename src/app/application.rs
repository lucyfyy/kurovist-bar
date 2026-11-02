use gtk4::prelude::*;
use gtk4::Application;

pub fn run() {
    let config = crate::config::loader::load();

    crate::app::services::hyprland::start();

    let app = Application::builder()
        .application_id("com.kurovistop.shell")
        .build();

    app.connect_activate(move |app| {
        crate::shell::taskbar::create(app, &config);
    });

    app.run();
}