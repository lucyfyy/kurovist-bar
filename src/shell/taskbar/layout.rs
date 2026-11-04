use gtk4::prelude::*;
use gtk4::{Box, Button, Label, Orientation};

pub fn create(start_menu: &Box, workspace: &Box, datetime_btn: &Button) -> Box {
    let container = Box::new(Orientation::Horizontal, 0);
    container.add_css_class("bar-container");

    let left = Box::new(Orientation::Horizontal, 4);
    left.add_css_class("left-box");

    let right = Box::new(Orientation::Horizontal, 4);
    right.add_css_class("right-box");

    // --- Left Side: Start Button & Workspace Box ---
// --- Left Side: Start Menu Widget (Menggantikan tombol start biasa) ---
    left.append(start_menu);

    workspace.add_css_class("workspace-box");
    workspace.set_widget_name("workspace");
    left.append(workspace);   

    workspace.add_css_class("workspace-box");
    workspace.set_widget_name("workspace");
    left.append(workspace);

    // --- Right Side: DateTime Widget Menyatu ---
    datetime_btn.set_css_classes(&["datetime-button"]);
    datetime_btn.set_widget_name("datetime");
    right.append(datetime_btn);

    // Assembly Layout Utama
    container.append(&left);

    // Spacer di Tengah
    let spacer = Label::new(None);
    spacer.set_hexpand(true);
    container.append(&spacer);

    container.append(&right);

    container
}