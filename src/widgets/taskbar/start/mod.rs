
use gtk4::prelude::*;
use gtk4::{
    Box, Button, Entry, EventControllerMotion, Label, Orientation,
    Popover, Revealer, RevealerTransitionType,
};
use std::process::Command;

use crate::modules::app_loader::load_applications;

pub struct StartMenuWidget;

impl StartMenuWidget {
    pub fn create() -> Box {
        let container = Box::new(Orientation::Horizontal, 4);
        container.add_css_class("start-container");

        // --- Start Button ---
        let icon_button = Button::with_label("󰣇");
        icon_button.add_css_class("start-button");

        // --- Search Bar ---
        let search_entry = Entry::new();
        search_entry.set_placeholder_text(Some("Search apps..."));
        search_entry.add_css_class("start-search-entry");

        let revealer = Revealer::new();
        revealer.set_transition_type(RevealerTransitionType::SlideRight);
        revealer.set_transition_duration(250);
        revealer.set_child(Some(&search_entry));
        revealer.set_reveal_child(false);

        container.append(&icon_button);
        container.append(&revealer);

        // --- Popover ---
        let popover = Popover::new();
        popover.set_parent(&icon_button);
        popover.add_css_class("start-popover");
        popover.set_size_request(360, -1);

        let popup_layout = Box::new(Orientation::Vertical, 8);
        popup_layout.add_css_class("start-popup-box");

        // --- Application List ---
        let all_apps = load_applications();

        let app_list_box = gtk4::ListBox::new();
        app_list_box.add_css_class("app-list-box");
        app_list_box.set_selection_mode(gtk4::SelectionMode::Single);

        let scrolled_win = gtk4::ScrolledWindow::new();
        scrolled_win.set_child(Some(&app_list_box));
        scrolled_win.set_min_content_height(240);
        scrolled_win.set_max_content_height(260);
        scrolled_win.set_policy(
            gtk4::PolicyType::Never,
            gtk4::PolicyType::Automatic,
        );

        let populate_list = {
            let app_list = app_list_box.clone();
            let apps_data = all_apps.clone();
            let popover_ref = popover.clone();

            move |query: &str| {
                while let Some(child) = app_list.first_child() {
                    app_list.remove(&child);
                }

                let query = query.to_lowercase();

                for app in &apps_data {
                    if query.is_empty()
                        || app.name.to_lowercase().contains(&query)
                    {
                        let row_box = Box::new(Orientation::Horizontal, 8);
                        row_box.add_css_class("app-row-item");

                        let image = gtk4::Image::from_icon_name(&app.icon);
                        image.set_pixel_size(20);

                        let label = Label::new(Some(&app.name));
                        label.set_xalign(0.0);
                        label.set_hexpand(true);

                        row_box.append(&image);
                        row_box.append(&label);

                        let row = gtk4::ListBoxRow::new();
                        row.set_child(Some(&row_box));
                        row.set_activatable(true);

                        let exec_cmd = app.exec.clone();
                        let app_name = app.name.clone();
                        let pop_close = popover_ref.clone();

                        row.connect_activate(move |_| {
                            println!("ROW CLICKED: {}", app_name);
                            println!("EXEC COMMAND: {}", exec_cmd);

                            pop_close.popdown();

                            let status = Command::new("sh")
                                .arg("-c")
                                .arg(&exec_cmd)
                                .spawn();

                            match status {
                                Ok(child) => println!("Process spawned: PID {}", child.id()),
                                Err(e) => eprintln!("Execution failed: {}", e),
                            }
                        });

                        app_list.append(&row);
                    }
                }
            }
        };

        populate_list("");

        // --- Search Filter ---
        let populate_clone = populate_list.clone();

        search_entry.connect_changed(move |entry| {
            let text = entry.text();
            populate_clone(&text);
        });

        popup_layout.append(&scrolled_win);

        // --- Power Footer ---
        let power_box = Box::new(Orientation::Horizontal, 8);
        power_box.add_css_class("power-footer-box");

        let btn_lock = Button::with_label("󰌾");
        let btn_suspend = Button::with_label("󰤄");
        let btn_logout = Button::with_label("󰍃");
        let btn_shutdown = Button::with_label("󰐥");

        let power_buttons = [
            &btn_lock,
            &btn_suspend,
            &btn_logout,
            &btn_shutdown,
        ];

        for btn in power_buttons {
            btn.add_css_class("power-btn");
            btn.set_hexpand(true);
        }

        // --- Power Actions ---
        let popover_lock = popover.clone();

        btn_lock.connect_clicked(move |_| {
            popover_lock.popdown();
            let _ = Command::new("hyprlock").spawn();
        });

        let popover_suspend = popover.clone();

        btn_suspend.connect_clicked(move |_| {
            popover_suspend.popdown();
            let _ = Command::new("systemctl")
                .arg("suspend")
                .spawn();
        });

        let popover_logout = popover.clone();

        btn_logout.connect_clicked(move |_| {
            popover_logout.popdown();
            let _ = Command::new("hyprctl")
                .args(["dispatch", "exit"])
                .spawn();
        });

        let popover_shutdown = popover.clone();

        btn_shutdown.connect_clicked(move |_| {
            popover_shutdown.popdown();
            let _ = Command::new("systemctl")
                .arg("poweroff")
                .spawn();
        });

        power_box.append(&btn_lock);
        power_box.append(&btn_suspend);
        power_box.append(&btn_logout);
        power_box.append(&btn_shutdown);

        popup_layout.append(&power_box);
        popover.set_child(Some(&popup_layout));

        // --- Hover Controller ---
        let motion_controller = EventControllerMotion::new();

        let revealer_enter = revealer.clone();

        motion_controller.connect_enter(move |_, _, _| {
            revealer_enter.set_reveal_child(true);
        });

        let revealer_leave = revealer.clone();

        motion_controller.connect_leave(move |_| {
            revealer_leave.set_reveal_child(false);
        });

        container.add_controller(motion_controller);

        // --- Popover Toggle ---
        let popover_toggle = popover.clone();

        icon_button.connect_clicked(move |_| {
            if popover_toggle.is_visible() {
                popover_toggle.popdown();
            } else {
                popover_toggle.popup();
            }
        });

        container
    }
}
