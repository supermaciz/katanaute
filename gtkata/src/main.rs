mod api;
mod auth_window;
mod config;
mod main_window;
mod models;
mod session_dialog;

use gtk::prelude::*;
use gtk::{glib, Application};
use libadwaita as adw;

use auth_window::{AuthMessage, AuthWindow};
use config::Config;
use main_window::MainWindow;

const APP_ID: &str = "com.katanaute.gtkata";

fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();

    app.connect_activate(build_ui);

    app.run()
}

fn build_ui(app: &Application) {
    // Load Adwaita styles
    adw::init().expect("Failed to initialize Adwaita");

    // Check for saved token
    if let Ok(Some(token)) = Config::load_token() {
        // Validate token by fetching current user
        let mut api = api::ApiClient::new(None);
        api.set_token(token.clone());

        if let Ok(user) = api.get_current_user() {
            // Token is valid, show main window
            let main_window = MainWindow::new(app, token, user);
            main_window.show();
            return;
        } else {
            // Token is invalid, clear it
            let _ = Config::clear_all();
        }
    }

    // No valid token, show auth window
    let (auth_window, rx) = AuthWindow::new(app);
    auth_window.show();

    // Handle auth messages using glib's channel mechanism
    let app_clone = app.clone();
    let auth_window_clone = auth_window.clone();
    
    glib::idle_add_local(move || {
        match rx.try_recv() {
            Ok(msg) => {
                match &msg {
                    AuthMessage::LoginSuccess(auth_response) | AuthMessage::DeviceFlowSuccess(auth_response) => {
                        // Save token and user email
                        if let Err(e) = Config::save_token(&auth_response.access_token) {
                            eprintln!("Failed to save token: {}", e);
                        }
                        if let Err(e) = Config::save_user_email(&auth_response.user.email) {
                            eprintln!("Failed to save user email: {}", e);
                        }

                        // Close auth window
                        auth_window_clone.close();

                        // Show main window
                        let main_window = MainWindow::new(
                            &app_clone,
                            auth_response.access_token.clone(),
                            auth_response.user.clone(),
                        );
                        main_window.show();
                    }
                    AuthMessage::DeviceFlowInitiated { user_code, verification_uri: _ } => {
                        // Update UI with device flow information
                        // Note: The labels are updated in the background thread before this message
                        eprintln!("Device flow initiated. User code: {}", user_code);
                    }
                    AuthMessage::Error(error_msg) => {
                        eprintln!("Authentication error: {}", error_msg);
                        // TODO: Show error in UI
                    }
                }
                glib::ControlFlow::Continue
            }
            Err(std::sync::mpsc::TryRecvError::Empty) => {
                glib::ControlFlow::Continue
            }
            Err(std::sync::mpsc::TryRecvError::Disconnected) => {
                glib::ControlFlow::Break
            }
        }
    });
}
