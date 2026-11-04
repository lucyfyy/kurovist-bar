use gtk4::prelude::*;
use gtk4::{Box, Button, Orientation};
use std::cell::RefCell;
use std::rc::Rc;

/// Struct penampung widget workspace.
/// Menggunakan `Rc<RefCell<...>>` agar aman di-clone dan di-update
/// dari closure event loop GTK4.
#[derive(Clone)]
pub struct WorkspaceWidget {
    pub container: Box,
    pub buttons: Rc<RefCell<Vec<Button>>>,
}

impl WorkspaceWidget {
    /// Inisialisasi widget workspace awal
    pub fn new(workspaces: Vec<i32>, active_id: i32) -> Self {
        let container = Box::new(Orientation::Horizontal, 5);

        let widget = Self {
            container,
            buttons: Rc::new(RefCell::new(Vec::new())),
        };

        widget.update(workspaces, active_id);
        widget
    }

    /// Membuat 1 tombol workspace dan memasangkan CSS class sekaligus
    fn add_workspace(&self, id: i32, active_id: i32) {
        let button = Button::with_label(&id.to_string());

        // Kunci Perbaikan: Pasang class 'workspace-button' dan 'active' dalam 1 array sekaligus!
        let mut classes = vec!["workspace-button"];
        if id == active_id {
            classes.push("active");
        }
        button.set_css_classes(&classes);

        // Click handler ke Hyprland dispatcher
        button.connect_clicked(move |_| {
            crate::platform::hyprland::command::workspace(id);
        });

        self.container.append(&button);
        self.buttons.borrow_mut().push(button);
    }

    /// Merender ulang seluruh tombol workspace saat ada event Hyprland masuk
    pub fn update(&self, workspaces: Vec<i32>, active_id: i32) {
        let old_buttons = std::mem::take(&mut *self.buttons.borrow_mut());

        for button in old_buttons {
            self.container.remove(&button);
        }

        for id in workspaces {
            self.add_workspace(id, active_id);
        }
    }
}