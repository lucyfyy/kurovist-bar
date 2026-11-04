mod layout;

use gtk4::gdk::Display;
use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow, CssProvider};
use gtk4_layer_shell::{Edge, Layer, LayerShell};
use std::path::Path;
use std::sync::mpsc;
use std::thread;

fn load_css(theme: &str) {
    let display = Display::default().expect("Gagal mendapatkan display default GTK");

    let main_provider = CssProvider::new();
    let main_css_path = "assets/styles/main.css";

    if Path::new(main_css_path).exists() {
        main_provider.load_from_path(main_css_path);
    } else {
        main_provider.load_from_data(include_str!("../../../assets/styles/main.css"));
    }

    gtk4::style_context_add_provider_for_display(
        &display,
        &main_provider,
        gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );

    let theme_provider = CssProvider::new();
    let theme_file = if theme == "light" { "light.css" } else { "dark.css" };
    let theme_css_path = format!("assets/styles/themes/{}", theme_file);

    if Path::new(&theme_css_path).exists() {
        theme_provider.load_from_path(&theme_css_path);
    } else {
        let theme_css = match theme {
            "light" => include_str!("../../../assets/styles/themes/light.css"),
            _ => include_str!("../../../assets/styles/themes/dark.css"),
        };
        theme_provider.load_from_data(theme_css);
    }

    gtk4::style_context_add_provider_for_display(
        &display,
        &theme_provider,
        gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION + 1,
    );
}

pub fn create(app: &Application, config: &crate::config::Config) {
    load_css(&config.taskbar.theme);

    let window = ApplicationWindow::builder().application(app).build();
    window.add_css_class("main-window");

    window.init_layer_shell();
    window.set_layer(Layer::Top);

    match config.taskbar.position.as_str() {
        "top" => window.set_anchor(Edge::Top, true),
        "bottom" => window.set_anchor(Edge::Bottom, true),
        "left" => window.set_anchor(Edge::Left, true),
        "right" => window.set_anchor(Edge::Right, true),
        _ => window.set_anchor(Edge::Bottom, true),
    }

    window.set_anchor(Edge::Left, true);
    window.set_anchor(Edge::Right, true);

    window.set_default_height(config.taskbar.height);
    window.set_exclusive_zone(config.taskbar.height);

    // 1. Workspace Widget Setup
    let workspaces = crate::platform::hyprland::query::get_workspaces();
    let active_workspace_id = crate::platform::hyprland::query::get_active_workspace_id();
    let visible = crate::state::workspace::calculate_visible_workspaces(workspaces, active_workspace_id);
    let workspace = crate::widgets::taskbar::workspace::WorkspaceWidget::new(visible, active_workspace_id);

    // 2. Event Watcher Hyprland
    let workspace_widget = workspace.clone();
    let (tx, rx) = mpsc::channel::<String>();

    thread::spawn(move || {
        crate::platform::hyprland::event::listen(tx);
    });

    glib::idle_add_local(move || {
        let mut should_update = false;

        while let Ok(event) = rx.try_recv() {
            if event.starts_with("workspace>>")
                || event.starts_with("focusedmon>>")
                || event.starts_with("createworkspace>>")
                || event.starts_with("destroyworkspace>>")
                || event.starts_with("openwindow>>")
                || event.starts_with("closewindow>>")
            {
                should_update = true;
            }
        }

        if should_update {
            let workspaces = crate::platform::hyprland::query::get_workspaces();
            let active_id = crate::platform::hyprland::query::get_active_workspace_id();
            let visible = crate::state::workspace::calculate_visible_workspaces(workspaces, active_id);

            workspace_widget.update(visible, active_id);
        }

        glib::ControlFlow::Continue
    });

    // 3. Inisialisasi Widget DateTime menyatu
    let datetime = crate::widgets::taskbar::datetime::DateTimeWidget::create();

    // 4. Inisialisasi Start Menu Widget
    let start_menu = crate::widgets::taskbar::start::StartMenuWidget::create();

    // 5. Render Layout & Penayangan
    let content = layout::create(&start_menu, &workspace.container, &datetime);
    window.set_child(Some(&content));

    window.present();
}