mod layout;

use gtk4::gdk::Display;
use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow, CssProvider};

use gtk4_layer_shell::{Edge, Layer, LayerShell};

fn load_css(theme: &str) {
    let css = match theme {
        "dark" => include_str!("../../../assets/styles/dark.css"),
        "light" => include_str!("../../../assets/styles/light.css"),
        _ => include_str!("../../../assets/styles/dark.css"),
    };

    let provider = CssProvider::new();

    provider.load_from_data(css);

    gtk4::style_context_add_provider_for_display(
        &Display::default().unwrap(),
        &provider,
        gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}

pub fn create(
    app: &Application,
    config: &crate::config::Config,
) {
    load_css(&config.taskbar.theme);

    let window = ApplicationWindow::builder()
        .application(app)
        .build();

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

    // Workspace widget
    let workspaces = crate::platform::hyprland::query::get_workspaces();

    let visible =
        crate::state::workspace::visible_workspaces(
            workspaces,
            1,
        );

    let workspace =
        crate::widgets::taskbar::workspace::WorkspaceWidget::new(
            visible,
        );

    // Clock widget
    let clock =
        crate::widgets::taskbar::clock::create();

    // Layout
    let content =
        layout::create(
            &workspace.container,
            &clock,
        );

    window.set_child(Some(&content));

    window.present();
}