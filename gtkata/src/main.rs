mod api;
mod auth_window;
mod config;
mod main_window;
mod models;
mod session_dialog;

use gtk::prelude::*;
use gtk::{glib, Application};

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
    adwaita::init().expect("Failed to initialize Adwaita");

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

    // Handle auth messages
    let app_clone = app.clone();
    rx.attach(None, move |msg| {
        match msg {
            AuthMessage::LoginSuccess(auth_response) | AuthMessage::DeviceFlowSuccess(auth_response) => {
                // Save token and user email
                if let Err(e) = Config::save_token(&auth_response.access_token) {
                    eprintln!("Failed to save token: {}", e);
                }
                if let Err(e) = Config::save_user_email(&auth_response.user.email) {
                    eprintln!("Failed to save user email: {}", e);
                }

                // Close auth window
                auth_window.close();

                // Show main window
                let main_window = MainWindow::new(
                    &app_clone,
                    auth_response.access_token,
                    auth_response.user,
                );
                main_window.show();
            }
            AuthMessage::Error(error_msg) => {
                eprintln!("Authentication error: {}", error_msg);
                // TODO: Show error in UI
            }
        }

        glib::ControlFlow::Continue
    });
}
