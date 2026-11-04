use chrono::{Datelike, Local, NaiveDate};
use gtk4::prelude::*;
use gtk4::{Button, Label, Popover};

pub struct CalendarWidget;

impl CalendarWidget {
    /// Membuat Button Kalender tersendiri (dengan Icon 󰸗 dan Popover CLI Calendar)
    pub fn create() -> Button {
        // Tombol kalender tersendiri
        let button = Button::with_label("󰸗"); // Icon kalender dari Nerd Font
        button.set_css_classes(&["calendar-button"]);

        // Popover Kalender
        let popover = Popover::new();
        popover.add_css_class("calendar-popover");

        let cal_label = Label::new(None);
        cal_label.add_css_class("calendar-text");
        cal_label.set_use_markup(true);

        cal_label.set_markup(&Self::generate_month_pango());

        let label_clone = cal_label.clone();
        popover.connect_show(move |_| {
            label_clone.set_markup(&Self::generate_month_pango());
        });

        popover.set_child(Some(&cal_label));
        popover.set_parent(&button);

        button.connect_clicked(move |_| {
            if popover.is_visible() {
                popover.popdown();
            } else {
                popover.popup();
            }
        });

        button
    }

    fn generate_month_pango() -> String {
        let now = Local::now();
        let year = now.year();
        let month = now.month();
        let today = now.day();

        let mut out = String::new();

        let month_name = now.format("%B").to_string();
        out.push_str(&format!("<b>{:<11} {}</b>\n", month_name, year));
        out.push_str("<i>Su Mo Tu We Th Fr Sa</i>\n");

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
        .signed_duration_since(NaiveDate::from_ymd_opt(year, month, 1).unwrap())
        .num_days();

        let mut current_column = first_day;

        for day in 1..=days_in_month {
            if day == today as i64 {
                out.push_str(&format!("<u><b>{:2}</b></u> ", day));
            } else {
                out.push_str(&format!("{:2} ", day));
            }

            current_column += 1;
            if current_column % 7 == 0 {
                out.push('\n');
            }
        }

        out
    }
}