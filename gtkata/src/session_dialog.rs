use chrono::Utc;
use gtk::prelude::*;
use gtk::{
    Box, Button, CheckButton, ComboBoxText, Dialog, Entry, Label, Orientation, ResponseType,
    ScrolledWindow, TextView, Window,
};

use crate::models::{CreateSessionRequest, Kata};

pub struct SessionDialog {
    dialog: Dialog,
    kata_combo: ComboBoxText,
    in_course_check: CheckButton,
    notes_view: TextView,
    katas: Vec<Kata>,
}

impl SessionDialog {
    pub fn new(parent: &Window, katas: Vec<Kata>) -> Self {
        let dialog = Dialog::with_buttons(
            Some("New Training Session"),
            Some(parent),
            gtk::DialogFlags::MODAL,
            &[("Cancel", ResponseType::Cancel), ("Create", ResponseType::Ok)],
        );

        dialog.set_default_width(500);
        dialog.set_default_height(400);

        let content_area = dialog.content_area();
        let main_box = Box::new(Orientation::Vertical, 12);
        main_box.set_margin_top(12);
        main_box.set_margin_bottom(12);
        main_box.set_margin_start(12);
        main_box.set_margin_end(12);

        // Kata selection
        let kata_label = Label::new(Some("Kata"));
        kata_label.set_halign(gtk::Align::Start);
        main_box.append(&kata_label);

        let kata_combo = ComboBoxText::new();
        for kata in &katas {
            kata_combo.append(
                Some(&kata.id.to_string()),
                &format!("{} ({})", kata.name, kata.level.to_string()),
            );
        }
        kata_combo.set_active(Some(0));
        main_box.append(&kata_combo);

        // In course checkbox
        let in_course_check = CheckButton::with_label("Part of structured learning path");
        in_course_check.set_active(true);
        in_course_check.set_margin_top(8);
        main_box.append(&in_course_check);

        // Notes
        let notes_label = Label::new(Some("Notes (Markdown)"));
        notes_label.set_halign(gtk::Align::Start);
        notes_label.set_margin_top(8);
        main_box.append(&notes_label);

        let notes_view = TextView::new();
        notes_view.set_wrap_mode(gtk::WrapMode::Word);
        notes_view.set_left_margin(8);
        notes_view.set_right_margin(8);
        notes_view.set_top_margin(8);
        notes_view.set_bottom_margin(8);

        let scrolled = ScrolledWindow::new();
        scrolled.set_child(Some(&notes_view));
        scrolled.set_vexpand(true);
        scrolled.set_min_content_height(200);
        main_box.append(&scrolled);

        content_area.append(&main_box);

        SessionDialog {
            dialog,
            kata_combo,
            in_course_check,
            notes_view,
            katas,
        }
    }

    pub fn get_session_data(&self) -> Option<CreateSessionRequest> {
        let kata_id = self
            .kata_combo
            .active_id()
            .and_then(|id| id.to_string().parse::<i32>().ok())?;

        let in_course = self.in_course_check.is_active();

        let buffer = self.notes_view.buffer();
        let notes_text = buffer
            .text(&buffer.start_iter(), &buffer.end_iter(), false)
            .to_string();

        let notes = if notes_text.trim().is_empty() {
            None
        } else {
            Some(notes_text)
        };

        Some(CreateSessionRequest {
            kata_id,
            practiced_at: Utc::now(),
            in_course,
            notes,
        })
    }

    pub fn connect_response<F>(&self, f: F)
    where
        F: Fn(&Dialog, ResponseType) + 'static,
    {
        self.dialog.connect_response(f);
    }

    pub fn show(&self) {
        self.dialog.show();
    }

    pub fn close(&self) {
        self.dialog.close();
    }
}
