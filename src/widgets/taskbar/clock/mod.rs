use chrono::Local;
use glib::ControlFlow;
use gtk4::prelude::*;
use gtk4::Label;
use std::time::Duration;

pub fn create() -> Label {
    let clock_label = Label::new(None);
    clock_label.add_css_class("clock-label");

    update_clock_label(&clock_label);

    let label_clone = clock_label.clone();
    glib::timeout_add_local(Duration::from_millis(1000), move || {
        update_clock_label(&label_clone);
        ControlFlow::Continue
    });

    clock_label
}

fn update_clock_label(label: &Label) {
    let now = Local::now();
    let formatted = now.format("%H:%M:%S").to_string(); // Format jam murni
    label.set_text(&formatted);
}