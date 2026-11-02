use gtk4::prelude::*;
use gtk4::{Box, Button, Label, Orientation};

pub fn create(
    workspace: &Box,
    clock: &Label,
) -> Box {

    let container =
        Box::new(
            Orientation::Horizontal,
            10,
        );

    let left =
        Box::new(
            Orientation::Horizontal,
            10,
        );

    let right =
        Box::new(
            Orientation::Horizontal,
            10,
        );



    // Start

    let start =
        Button::with_label("🪟");

    start.set_css_classes(
        &["start-button"]
    );

    start.connect_clicked(|_| {
        println!("Start clicked!");
    });

    left.append(&start);



    // Workspace

    left.append(workspace);



    // Clock

    clock.set_css_classes(
        &["clock"]
    );

    right.append(clock);



    // Left

    container.append(&left);



    // Spacer

    let spacer =
        Label::new(None);

    spacer.set_hexpand(true);

    container.append(&spacer);



    // Right

    container.append(&right);



    container
}