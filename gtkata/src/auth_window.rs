use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow, Box, Button, Entry, Label, Orientation, Stack};
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use crate::api::ApiClient;
use crate::config::Config;
use crate::models::AuthResponse;

#[derive(Clone)]
pub enum AuthMessage {
    LoginSuccess(AuthResponse),
    DeviceFlowSuccess(AuthResponse),
    Error(String),
}

pub struct AuthWindow {
    window: ApplicationWindow,
    tx: glib::Sender<AuthMessage>,
}

impl AuthWindow {
    pub fn new(app: &Application) -> (Self, glib::Receiver<AuthMessage>) {
        let (tx, rx) = glib::MainContext::channel(glib::Priority::DEFAULT);

        let window = ApplicationWindow::builder()
            .application(app)
            .title("GTKata - Login")
            .default_width(400)
            .default_height(500)
            .build();

        let auth_window = AuthWindow {
            window: window.clone(),
            tx: tx.clone(),
        };

        auth_window.build_ui();

        (auth_window, rx)
    }

    fn build_ui(&self) {
        let main_box = Box::new(Orientation::Vertical, 20);
        main_box.set_margin_top(40);
        main_box.set_margin_bottom(40);
        main_box.set_margin_start(40);
        main_box.set_margin_end(40);
        main_box.set_valign(gtk::Align::Center);

        // Title
        let title = Label::new(Some("GTKata"));
        title.add_css_class("title-1");
        main_box.append(&title);

        let subtitle = Label::new(Some("Kata Training Tracker"));
        subtitle.add_css_class("dim-label");
        main_box.append(&subtitle);

        // Stack for switching between login and device flow
        let stack = Stack::new();
        stack.set_transition_type(gtk::StackTransitionType::SlideLeftRight);

        // Login/Register page
        let login_page = self.build_login_page();
        stack.add_titled(&login_page, Some("login"), "Email Login");

        // Device flow page
        let device_page = self.build_device_flow_page();
        stack.add_titled(&device_page, Some("device"), "Device Flow");

        // Stack switcher
        let switcher = gtk::StackSwitcher::new();
        switcher.set_stack(Some(&stack));
        switcher.set_halign(gtk::Align::Center);

        main_box.append(&switcher);
        main_box.append(&stack);

        self.window.set_child(Some(&main_box));
    }

    fn build_login_page(&self) -> Box {
        let page = Box::new(Orientation::Vertical, 12);
        page.set_valign(gtk::Align::Center);

        // Email entry
        let email_label = Label::new(Some("Email"));
        email_label.set_halign(gtk::Align::Start);
        page.append(&email_label);

        let email_entry = Entry::new();
        email_entry.set_placeholder_text(Some("email@example.com"));
        page.append(&email_entry);

        // Password entry
        let password_label = Label::new(Some("Password"));
        password_label.set_halign(gtk::Align::Start);
        password_label.set_margin_top(8);
        page.append(&password_label);

        let password_entry = Entry::new();
        password_entry.set_placeholder_text(Some("Password"));
        password_entry.set_visibility(false);
        password_entry.set_input_purpose(gtk::InputPurpose::Password);
        page.append(&password_entry);

        // Error label
        let error_label = Label::new(None);
        error_label.add_css_class("error");
        error_label.set_margin_top(8);
        page.append(&error_label);

        // Buttons
        let button_box = Box::new(Orientation::Horizontal, 12);
        button_box.set_halign(gtk::Align::Center);
        button_box.set_margin_top(20);

        let login_button = Button::with_label("Login");
        login_button.add_css_class("suggested-action");
        button_box.append(&login_button);

        let register_button = Button::with_label("Register");
        button_box.append(&register_button);

        page.append(&button_box);

        // Clone for closures
        let tx = self.tx.clone();
        let tx_reg = self.tx.clone();
        let email_clone = email_entry.clone();
        let password_clone = password_entry.clone();
        let error_clone = error_label.clone();

        // Login handler
        login_button.connect_clicked(move |_| {
            let email = email_clone.text().to_string();
            let password = password_clone.text().to_string();
            let tx = tx.clone();
            let error_label = error_clone.clone();

            if email.is_empty() || password.is_empty() {
                error_label.set_text("Email and password are required");
                return;
            }

            error_label.set_text("");

            thread::spawn(move || {
                let api = ApiClient::new(None);
                match api.login(&email, &password) {
                    Ok(auth_response) => {
                        let _ = tx.send(AuthMessage::LoginSuccess(auth_response));
                    }
                    Err(e) => {
                        let _ = tx.send(AuthMessage::Error(format!("Login failed: {}", e)));
                    }
                }
            });
        });

        // Register handler
        let email_clone2 = email_entry.clone();
        let password_clone2 = password_entry.clone();
        let error_clone2 = error_label.clone();

        register_button.connect_clicked(move |_| {
            let email = email_clone2.text().to_string();
            let password = password_clone2.text().to_string();
            let tx = tx_reg.clone();
            let error_label = error_clone2.clone();

            if email.is_empty() || password.is_empty() {
                error_label.set_text("Email and password are required");
                return;
            }

            error_label.set_text("");

            thread::spawn(move || {
                let api = ApiClient::new(None);
                match api.register(&email, &password) {
                    Ok(auth_response) => {
                        let _ = tx.send(AuthMessage::LoginSuccess(auth_response));
                    }
                    Err(e) => {
                        let _ = tx.send(AuthMessage::Error(format!("Registration failed: {}", e)));
                    }
                }
            });
        });

        // Allow Enter key to submit
        let login_button_clone = login_button.clone();
        password_entry.connect_activate(move |_| {
            login_button_clone.activate();
        });

        page
    }

    fn build_device_flow_page(&self) -> Box {
        let page = Box::new(Orientation::Vertical, 12);
        page.set_valign(gtk::Align::Center);

        let instruction_label = Label::new(Some(
            "Click 'Start Device Flow' to authenticate\nwithout entering your password here.",
        ));
        instruction_label.set_justify(gtk::Justification::Center);
        page.append(&instruction_label);

        // User code display
        let user_code_box = Box::new(Orientation::Vertical, 6);
        user_code_box.set_margin_top(20);
        user_code_box.set_visible(false);

        let user_code_label = Label::new(Some("Enter this code in your browser:"));
        user_code_box.append(&user_code_label);

        let user_code_display = Label::new(None);
        user_code_display.add_css_class("title-2");
        user_code_display.set_selectable(true);
        user_code_box.append(&user_code_display);

        let verification_label = Label::new(None);
        verification_label.set_use_markup(true);
        verification_label.set_margin_top(8);
        user_code_box.append(&verification_label);

        let status_label = Label::new(None);
        status_label.set_margin_top(8);
        status_label.add_css_class("dim-label");
        user_code_box.append(&status_label);

        page.append(&user_code_box);

        // Error label
        let error_label = Label::new(None);
        error_label.add_css_class("error");
        error_label.set_margin_top(8);
        page.append(&error_label);

        // Start button
        let start_button = Button::with_label("Start Device Flow");
        start_button.add_css_class("suggested-action");
        start_button.set_halign(gtk::Align::Center);
        start_button.set_margin_top(20);
        page.append(&start_button);

        // Device flow handler
        let tx = self.tx.clone();
        let user_code_box_clone = user_code_box.clone();
        let user_code_display_clone = user_code_display.clone();
        let verification_label_clone = verification_label.clone();
        let status_label_clone = status_label.clone();
        let error_label_clone = error_label.clone();
        let start_button_clone = start_button.clone();

        start_button.connect_clicked(move |_| {
            let tx = tx.clone();
            let user_code_box = user_code_box_clone.clone();
            let user_code_display = user_code_display_clone.clone();
            let verification_label = verification_label_clone.clone();
            let status_label = status_label_clone.clone();
            let error_label = error_label_clone.clone();
            let start_button = start_button_clone.clone();

            error_label.set_text("");
            start_button.set_sensitive(false);

            thread::spawn(move || {
                let api = ApiClient::new(None);

                // Initiate device flow
                let device_response = match api.initiate_device_flow() {
                    Ok(response) => response,
                    Err(e) => {
                        let _ = tx.send(AuthMessage::Error(format!(
                            "Failed to start device flow: {}",
                            e
                        )));
                        return;
                    }
                };

                let device_code = device_response.device_code.clone();
                let interval = device_response.interval as u64;

                // Update UI with user code
                glib::idle_add_once({
                    let user_code_box = user_code_box.clone();
                    let user_code_display = user_code_display.clone();
                    let verification_label = verification_label.clone();
                    let status_label = status_label.clone();
                    let user_code = device_response.user_code.clone();
                    let verification_uri = device_response.verification_uri_complete.clone();

                    move || {
                        user_code_display.set_text(&user_code);
                        verification_label.set_markup(&format!(
                            "<a href=\"{}\">{}</a>",
                            verification_uri, "Open in browser"
                        ));
                        status_label.set_text("Waiting for authorization...");
                        user_code_box.set_visible(true);
                    }
                });

                // Poll for completion
                let max_attempts = device_response.expires_in / device_response.interval;
                for _ in 0..max_attempts {
                    thread::sleep(Duration::from_secs(interval));

                    match api.poll_device_token(&device_code) {
                        Ok(Some(auth_response)) => {
                            let _ = tx.send(AuthMessage::DeviceFlowSuccess(auth_response));
                            return;
                        }
                        Ok(None) => {
                            // Still pending, continue polling
                            continue;
                        }
                        Err(e) => {
                            let _ = tx.send(AuthMessage::Error(format!("Device flow error: {}", e)));
                            return;
                        }
                    }
                }

                // Timeout
                let _ = tx.send(AuthMessage::Error(
                    "Device authorization timed out".to_string(),
                ));
            });
        });

        page
    }

    pub fn show(&self) {
        self.window.present();
    }

    pub fn close(&self) {
        self.window.close();
    }
}
