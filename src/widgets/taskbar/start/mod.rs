
use gtk4::prelude::*;
use gtk4::{
    Box, Button, Entry, EventControllerKey, Label, Orientation,
    Popover, ScrolledWindow,
};
use std::process::Command;

use crate::modules::app_loader::load_applications;

pub struct StartMenuWidget;

impl StartMenuWidget {
    pub fn create() -> Box {
        // --- Main Container ---
        let container = Box::new(Orientation::Horizontal, 4);
        container.add_css_class("start-container");

        let icon_button = Button::with_label("󰣇");
        icon_button.add_css_class("start-button");
        container.append(&icon_button);

        // --- Main Popover ---
        let popover = Popover::new();
        popover.set_parent(&icon_button);
        popover.add_css_class("start-popover");
        popover.set_size_request(380, -1);

        let main_layout = Box::new(Orientation::Vertical, 12);
        main_layout.add_css_class("start-popup-box");

        // --- User Profile ---
        let profile_box = Box::new(Orientation::Horizontal, 10);
        profile_box.add_css_class("start-profile-box");

        let home_dir = std::env::var("HOME")
            .unwrap_or_else(|_| "/home/bam".to_string());

        let profile_pic_path = format!(
            "{}/.config/hyprlock/wallpapers/user.jpg",
            home_dir
        );

        let avatar_image = if std::path::Path::new(&profile_pic_path).exists() {
            gtk4::Image::from_file(&profile_pic_path)
        } else {
            gtk4::Image::from_icon_name("avatar-default-symbolic")
        };

        avatar_image.set_pixel_size(36);
        avatar_image.add_css_class("start-avatar-img");

        let username = std::env::var("USER")
            .unwrap_or_else(|_| "bam".to_string());

        let user_label = Label::new(Some(&username));
        user_label.add_css_class("start-username");
        user_label.set_xalign(0.0);
        user_label.set_hexpand(true);

        profile_box.append(&avatar_image);
        profile_box.append(&user_label);
        main_layout.append(&profile_box);

        // --- Search Entry ---
        let search_entry = Entry::new();
        search_entry.set_placeholder_text(
            Some("Type here to search apps...")
        );
        search_entry.add_css_class("start-search-entry");
        search_entry.set_can_focus(true);
        search_entry.set_hexpand(true);

        main_layout.append(&search_entry);

        // --- Application List ---
        let all_apps = load_applications();

        let app_list_box = gtk4::ListBox::new();
        app_list_box.add_css_class("app-list-box");
        app_list_box.set_selection_mode(
            gtk4::SelectionMode::Single
        );

        let scrolled_win = ScrolledWindow::new();
        scrolled_win.set_child(Some(&app_list_box));
        scrolled_win.set_min_content_height(240);
        scrolled_win.set_max_content_height(260);
        scrolled_win.set_policy(
            gtk4::PolicyType::Never,
            gtk4::PolicyType::Automatic,
        );

        // --- Application Launcher ---
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
                        let row_box = Box::new(
                            Orientation::Horizontal,
                            10,
                        );
                        row_box.add_css_class("app-row-item");

                        let image = gtk4::Image::from_icon_name(
                            &app.icon
                        );
                        image.set_pixel_size(20);

                        let label = Label::new(Some(&app.name));
                        label.set_xalign(0.0);
                        label.set_hexpand(true);

                        row_box.append(&image);
                        row_box.append(&label);

                        let row = gtk4::ListBoxRow::new();
                        row.set_child(Some(&row_box));
                        row.set_activatable(true);

                        // Store application command
                        row.set_widget_name(&app.exec);

                        app_list.append(&row);
                    }
                }

                app_list.unselect_all();

                // Keep reference for launcher
                let _ = &popover_ref;
            }
        };

        populate_list("");

        // --- Search Filter ---
        let populate_clone = populate_list.clone();

        search_entry.connect_changed(move |entry| {
            populate_clone(&entry.text());
        });

        // --- Launch Selected Application ---
        let popover_activate = popover.clone();

        app_list_box.connect_row_activated(move |_, row| {
            let cmd = row.widget_name().to_string();

            if cmd.is_empty() {
                return;
            }

            match Command::new("sh")
                .arg("-c")
                .arg(&cmd)
                .spawn()
            {
                Ok(child) => {
                    println!(
                        "Application launched: {} (PID {})",
                        cmd,
                        child.id()
                    );

                    popover_activate.popdown();
                }
                Err(error) => {
                    eprintln!(
                        "Failed to launch {}: {}",
                        cmd,
                        error
                    );
                }
            }
        });

        // --- Search Keyboard Navigation ---
        let search_key_controller = EventControllerKey::new();

        let app_list_down = app_list_box.clone();

        search_key_controller.connect_key_pressed(
            move |_, key, _, _| {
                if key == gtk4::gdk::Key::Down {
                    if let Some(first_row) =
                        app_list_down.row_at_index(0)
                    {
                        app_list_down.select_row(Some(&first_row));
                        first_row.grab_focus();
                    }

                    return gtk4::glib::Propagation::Stop;
                }

                gtk4::glib::Propagation::Proceed
            },
        );

        search_entry.add_controller(search_key_controller);

        // --- List Keyboard Navigation ---
        let list_key_controller = EventControllerKey::new();

        let search_focus = search_entry.clone();
        let app_list_nav = app_list_box.clone();

        list_key_controller.connect_key_pressed(
            move |_, key, _, modifier| {
                let blocked_modifiers =
                    gtk4::gdk::ModifierType::CONTROL_MASK
                    | gtk4::gdk::ModifierType::ALT_MASK
                    | gtk4::gdk::ModifierType::SUPER_MASK;

                if modifier.intersects(blocked_modifiers) {
                    return gtk4::glib::Propagation::Proceed;
                }

                // Backspace
                if key == gtk4::gdk::Key::BackSpace {
                    let mut text =
                        search_focus.text().to_string();

                    if !text.is_empty() {
                        text.pop();
                        search_focus.set_text(&text);
                        search_focus.set_position(-1);
                    }

                    search_focus.grab_focus();

                    return gtk4::glib::Propagation::Stop;
                }

                // Return to search entry
                if key == gtk4::gdk::Key::Up {
                    if let Some(selected_row) =
                        app_list_nav.selected_row()
                    {
                        if selected_row.index() == 0 {
                            app_list_nav.select_row(
                                None::<&gtk4::ListBoxRow>
                            );

                            search_focus.grab_focus();

                            return gtk4::glib::Propagation::Stop;
                        }
                    }
                }

                // Type while list is focused
                if let Some(character) = key.to_unicode() {
                    if !character.is_control() {
                        let mut text =
                            search_focus.text().to_string();

                        text.push(character);

                        search_focus.set_text(&text);
                        search_focus.set_position(-1);
                        search_focus.grab_focus();

                        return gtk4::glib::Propagation::Stop;
                    }
                }

                gtk4::glib::Propagation::Proceed
            },
        );

        app_list_box.add_controller(list_key_controller);

        // --- Power Menu ---
        let power_box = Box::new(Orientation::Horizontal, 8);
        power_box.add_css_class("power-footer-box");

        let btn_lock = Button::with_label("󰌾");
        let btn_suspend = Button::with_label("󰤄");
        let btn_logout = Button::with_label("󰍃");
        let btn_shutdown = Button::with_label("󰐥");

        // Berikan class umum dan class spesifik per tombol
        let power_buttons = [
            (&btn_lock, "power-btn-lock"),
            (&btn_suspend, "power-btn-suspend"),
            (&btn_logout, "power-btn-logout"),
            (&btn_shutdown, "power-btn-shutdown"),
        ];

        for (btn, class_name) in power_buttons {
            btn.add_css_class("power-btn");
            btn.add_css_class(class_name);
            btn.set_hexpand(true);
        }

        // --- Power Actions ---
        let p1 = popover.clone();

        btn_lock.connect_clicked(move |_| {
            p1.popdown();
            let _ = Command::new("hyprlock").spawn();
        });

        let p2 = popover.clone();

        btn_suspend.connect_clicked(move |_| {
            p2.popdown();
            let _ = Command::new("systemctl")
                .arg("suspend")
                .spawn();
        });

        let p3 = popover.clone();

        btn_logout.connect_clicked(move |_| {
            p3.popdown();
            let _ = Command::new("hyprctl")
                .args(["dispatch", "exit"])
                .spawn();
        });

        let p4 = popover.clone();

        btn_shutdown.connect_clicked(move |_| {
            p4.popdown();
            let _ = Command::new("systemctl")
                .arg("poweroff")
                .spawn();
        });

        power_box.append(&btn_lock);
        power_box.append(&btn_suspend);
        power_box.append(&btn_logout);
        power_box.append(&btn_shutdown);

        // --- Assemble Layout ---
        main_layout.append(&scrolled_win);
        main_layout.append(&power_box);

        popover.set_child(Some(&main_layout));

        // --- Popover Visibility Handler ---
        let search_focus = search_entry.clone();
        let populate_reset = populate_list.clone();
        let app_list_reset = app_list_box.clone();

        popover.connect_visible_notify(move |p| {
            if p.is_visible() {
                search_focus.grab_focus();
            } else {
                search_focus.set_text("");
                app_list_reset.unselect_all();
                populate_reset("");
            }
        });

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
