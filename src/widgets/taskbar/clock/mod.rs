use gtk4::Label;
use gtk4::prelude::*;
use chrono::Local;
use glib::timeout_add_seconds_local;

pub fn create() -> Label {
    let label = Label::new(None);

    update(&label);

    let label_clone = label.clone();

    timeout_add_seconds_local(1, move || {
        update(&label_clone);

        glib::ControlFlow::Continue
    });

    label
        .set_css_classes(&["clock"]);

    label
}


fn update(label: &Label) {
    let now = Local::now();

    label.set_text(
        &now.format("%H:%M:%S").to_string()
    );
}