use gtk4::prelude::*;
use gtk4::{Box, Button, Orientation};

pub struct WorkspaceWidget {
    pub container: Box,
    pub buttons: Vec<Button>,
}

impl WorkspaceWidget {
    pub fn new(workspaces: Vec<i32>) -> Self {
        let container = Box::new(Orientation::Horizontal, 5);

        let mut widget = Self {
            container,
            buttons: Vec::new(),
        };

        for id in workspaces {
            widget.add_workspace(id);
        }

        widget
    }

    fn add_workspace(&mut self, id: i32) {
        let button = Button::with_label(&id.to_string());

        button.set_css_classes(&["workspace-button"]);

        button.connect_clicked(move |_| {
            crate::platform::hyprland::command::workspace(id);
        });

        self.container.append(&button);

        self.buttons.push(button);
    }

    pub fn update(&mut self, workspaces: Vec<i32>) {
        let old_buttons = std::mem::take(&mut self.buttons);

        for button in old_buttons {
            self.container.remove(&button);
        }

        for id in workspaces {
            self.add_workspace(id);
        }
    }
}
