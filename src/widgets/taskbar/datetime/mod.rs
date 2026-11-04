use chrono::{Datelike, Local, NaiveDate};
use glib::ControlFlow;
use gtk4::prelude::*;
use gtk4::{Box, Button, EventControllerMotion, GestureClick, Label, Orientation, Window};
use gtk4_layer_shell::{Edge, Layer, LayerShell};
use std::cell::Cell;
use std::rc::Rc;
use std::time::Duration;

pub struct DateTimeWidget;

impl DateTimeWidget {
    pub fn create() -> Button {
        // --- Date Time ---
        let content = Box::new(Orientation::Horizontal, 6);
        content.add_css_class("datetime-content");

        let icon_label = Label::new(Some("󰸗"));
        icon_label.add_css_class("datetime-icon");

        let text_label = Label::new(None);
        text_label.add_css_class("datetime-text");

        content.append(&icon_label);
        content.append(&text_label);

        // --- State Toggle Detik & Mode Kalender ---
        let show_seconds = Rc::new(Cell::new(true)); // Default tampilkan detik
        let is_month_mode = Rc::new(Cell::new(true));

        Self::update_datetime_text(&text_label, show_seconds.get());

        let label_clone = text_label.clone();
        let show_seconds_clone = show_seconds.clone();

        glib::timeout_add_local(Duration::from_secs(1), move || {
            Self::update_datetime_text(&label_clone, show_seconds_clone.get());
            ControlFlow::Continue
        });

        // --- Calendar Window ---
        let cal_window = Window::new();
        cal_window.add_css_class("calendar-window");
        cal_window.init_layer_shell();
        cal_window.set_layer(Layer::Overlay);

        cal_window.set_anchor(Edge::Top, true);
        cal_window.set_anchor(Edge::Right, true);
        cal_window.set_margin(Edge::Top, 5);
        cal_window.set_margin(Edge::Right, 5);

        // --- Calendar Layout ---
        let layout_box = Box::new(Orientation::Vertical, 5);
        layout_box.add_css_class("calendar-box");

        let year_header = Label::new(None);
        year_header.add_css_class("calendar-year-header");
        year_header.set_use_markup(true);
        year_header.set_xalign(0.5);

        let cal_label = Label::new(None);
        cal_label.add_css_class("datetime-calendar-text");
        cal_label.set_use_markup(true);
        cal_label.set_xalign(0.0);

        layout_box.append(&year_header);
        layout_box.append(&cal_label);

        cal_window.set_child(Some(&layout_box));

        // --- Button ---
        let button = Button::new();
        button.set_child(Some(&content));
        button.set_css_classes(&["datetime-button"]);

        Self::refresh_calendar(
            &cal_window,
            &year_header,
            &cal_label,
            true,
            false,
        );

        // --- Left Click: Toggle Sembunyikan/Tampilkan Detik ---
        let gesture = GestureClick::new();
        gesture.set_button(1); // Klik Kiri

        let text_label_click = text_label.clone();
        let show_seconds_click = show_seconds.clone();

        gesture.connect_pressed(move |_, _, _, _| {
            let next_state = !show_seconds_click.get();
            show_seconds_click.set(next_state);
            Self::update_datetime_text(&text_label_click, next_state);
        });

        button.add_controller(gesture);

        // --- Right Click: Toggle Mode Kalender (Bulan <-> Tahun) ---
        let right_gesture = GestureClick::new();
        right_gesture.set_button(3); // Klik Kanan

        let cal_window_right = cal_window.clone();
        let year_header_right = year_header.clone();
        let cal_label_right = cal_label.clone();
        let mode_right = is_month_mode.clone();

        right_gesture.connect_pressed(move |_, _, _, _| {
            let next_mode = !mode_right.get();
            mode_right.set(next_mode);

            if cal_window_right.is_visible() {
                Self::refresh_calendar(
                    &cal_window_right,
                    &year_header_right,
                    &cal_label_right,
                    next_mode,
                    true,
                );
            }
        });

        button.add_controller(right_gesture);

        // --- Hover: Show Calendar ---
        let motion_controller = EventControllerMotion::new();

        let cal_window_hover = cal_window.clone();
        let year_header_hover = year_header.clone();
        let cal_label_hover = cal_label.clone();
        let mode_hover = is_month_mode.clone();

        motion_controller.connect_enter(move |_, _, _| {
            Self::refresh_calendar(
                &cal_window_hover,
                &year_header_hover,
                &cal_label_hover,
                mode_hover.get(),
                true,
            );
        });

        // --- Leave: Hide Calendar ---
        let cal_window_leave = cal_window.clone();

        motion_controller.connect_leave(move |_| {
            cal_window_leave.hide();
        });

        button.add_controller(motion_controller);

        button
    }

    // --- Refresh Calendar ---
    fn refresh_calendar(
        window: &Window,
        header: &Label,
        calendar: &Label,
        is_month: bool,
        show_window: bool,
    ) {
        let now = Local::now();

        if is_month {
            header.set_markup(&format!(
                "<span color='#89b4fa'>{} {}</span>",
                now.format("%B"),
                now.year()
            ));
            calendar.set_markup(&Self::generate_month_pango());
        } else {
            header.set_markup(&format!(
                "<span color='#89b4fa'>Year {}</span>",
                now.year()
            ));
            calendar.set_markup(&Self::generate_year_pango());
        }

        window.hide();
        window.queue_resize();

        if let Some(child) = window.child() {
            child.queue_resize();
        }

        if show_window {
            window.present();
        }
    }

    // --- Date Time dengan Toggle Detik ---
    fn update_datetime_text(label: &Label, show_seconds: bool) {
        let now = Local::now();
        let formatted = if show_seconds {
            now.format("%d %b · %I:%M:%S %p").to_string()
        } else {
            now.format("%d %b · %I:%M %p").to_string()
        };
        
        label.set_text(&formatted);
    }

    // --- Month Calendar ---
    fn generate_month_pango() -> String {
        let now = Local::now();
        let year = now.year();
        let month = now.month();
        let today = now.day();

        let mut out = String::new();
        out.push_str(
            "<span color='#a6adc8' style='italic'>\
             Su Mo Tu We Th Fr Sa\
             </span>\n",
        );

        let first_day = NaiveDate::from_ymd_opt(year, month, 1)
            .unwrap()
            .weekday()
            .num_days_from_sunday();

        for _ in 0..first_day {
            out.push_str("   ");
        }

        let days_in_month = if month == 12 {
            NaiveDate::from_ymd_opt(year + 1, 1, 1)
        } else {
            NaiveDate::from_ymd_opt(year, month + 1, 1)
        }
        .unwrap()
        .signed_duration_since(
            NaiveDate::from_ymd_opt(year, month, 1).unwrap(),
        )
        .num_days();

        let mut current_col = first_day;

        for day in 1..=days_in_month {
            let is_friday = current_col % 7 == 5;
            let is_sunday = current_col % 7 == 0;

            if day == today as i64 {
                out.push_str(&format!(
                    "<span color='#89b4fa' weight='bold'><u>{:2}</u></span> ",
                    day
                ));
            } else if is_friday {
                out.push_str(&format!(
                    "<span color='#a6e3a1'>{:2}</span> ",
                    day
                ));
            } else if is_sunday {
                out.push_str(&format!(
                    "<span color='#f38ba8'>{:2}</span> ",
                    day
                ));
            } else {
                out.push_str(&format!("{:2} ", day));
            }

            current_col += 1;
            if current_col % 7 == 0 {
                out.push('\n');
            }
        }

        out
    }

    // --- Year Calendar ---
    fn generate_year_pango() -> String {
        let now = Local::now();
        let year = now.year();
        let current_month = now.month();
        let today = now.day();

        let mut out = String::new();

        for row in 0..2 {
            let months: Vec<u32> = (1..=6).map(|m| row * 6 + m).collect();

            let mut header_str = String::new();
            for (i, &m) in months.iter().enumerate() {
                let m_name = NaiveDate::from_ymd_opt(year, m, 1)
                    .unwrap()
                    .format("%B")
                    .to_string();
                header_str.push_str(&format!("{:<20}", m_name));
                if i < 5 {
                    header_str.push_str("    ");
                }
            }
            out.push_str(&format!(
                "<span color='#cba6f7' weight='bold'>{}</span>\n",
                header_str
            ));

            let mut days_header = String::new();
            for i in 0..6 {
                days_header.push_str("Su Mo Tu We Th Fr Sa");
                if i < 5 {
                    days_header.push_str("    ");
                }
            }
            out.push_str(&format!(
                "<span color='#a6adc8' style='italic'>{}</span>\n",
                days_header
            ));

            let mut month_days: Vec<Vec<String>> =
                vec![vec![String::new(); 42]; 6];

            for (m_idx, &m) in months.iter().enumerate() {
                let first_day = NaiveDate::from_ymd_opt(year, m, 1)
                    .unwrap()
                    .weekday()
                    .num_days_from_sunday() as usize;

                let days_in_m = if m == 12 {
                    NaiveDate::from_ymd_opt(year + 1, 1, 1)
                } else {
                    NaiveDate::from_ymd_opt(year, m + 1, 1)
                }
                .unwrap()
                .signed_duration_since(
                    NaiveDate::from_ymd_opt(year, m, 1).unwrap(),
                )
                .num_days();

                for d in 0..first_day {
                    month_days[m_idx][d] = "  ".to_string();
                }

                for day in 1..=days_in_m {
                    let idx = first_day + day as usize - 1;
                    let is_friday = idx % 7 == 5;
                    let is_sunday = idx % 7 == 0;

                    let d_str = if m == current_month && day == today as i64 {
                        format!(
                            "<span color='#89b4fa' weight='bold'><u>{:2}</u></span>",
                            day
                        )
                    } else if is_friday {
                        format!("<span color='#a6e3a1'>{:2}</span>", day)
                    } else if is_sunday {
                        format!("<span color='#f38ba8'>{:2}</span>", day)
                    } else {
                        format!("{:2}", day)
                    };

                    month_days[m_idx][idx] = d_str;
                }

                for d in (first_day + days_in_m as usize)..42 {
                    month_days[m_idx][d] = "  ".to_string();
                }
            }

            for week in 0..6 {
                let mut row_str = String::new();
                for m_idx in 0..6 {
                    for day_idx in 0..7 {
                        let cell = &month_days[m_idx][week * 7 + day_idx];
                        if cell.is_empty() {
                            row_str.push_str("  ");
                        } else {
                            row_str.push_str(cell);
                        }
                        if day_idx < 6 {
                            row_str.push(' ');
                        }
                    }
                    if m_idx < 5 {
                        row_str.push_str("    ");
                    }
                }
                if !row_str.trim().is_empty() {
                    out.push_str(&row_str);
                    out.push('\n');
                }
            }

            if row == 0 {
                out.push('\n');
            }
        }

        out.trim_end().to_string()
    }
}