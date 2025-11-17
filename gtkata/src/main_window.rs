use gtk::prelude::*;
use gtk::{
    glib, Application, ApplicationWindow, Box, Button, Label, ListBox, Orientation, ScrolledWindow,
    SearchEntry, Separator,
};
use std::rc::Rc;
use std::thread;
use std::sync::{Arc, Mutex};

use crate::api::ApiClient;
use crate::config::Config;
use crate::models::{Session, Kata, User};
use crate::session_dialog::SessionDialog;

pub struct MainWindow {
    window: ApplicationWindow,
    api: Arc<Mutex<ApiClient>>,
    sessions: Arc<Mutex<Vec<Session>>>,
    katas: Arc<Mutex<Vec<Kata>>>,
}

impl MainWindow {
    pub fn new(app: &Application, token: String, user: User) -> Self {
        let mut api = ApiClient::new(None);
        api.set_token(token.clone());

        let window = ApplicationWindow::builder()
            .application(app)
            .title("GTKata - Kata Training Tracker")
            .default_width(800)
            .default_height(600)
            .build();

        let main_window = MainWindow {
            window: window.clone(),
            api: Arc::new(Mutex::new(api)),
            sessions: Arc::new(Mutex::new(Vec::new())),
            katas: Arc::new(Mutex::new(Vec::new())),
        };

        main_window.build_ui(&user);
        main_window.load_data();

        main_window
    }

    fn build_ui(&self, user: &User) {
        let main_box = Box::new(Orientation::Vertical, 0);

        // Header bar
        let header = self.build_header(user);
        main_box.append(&header);

        main_box.append(&Separator::new(Orientation::Horizontal));

        // Toolbar
        let toolbar = self.build_toolbar();
        main_box.append(&toolbar);

        // Session list
        let session_list = self.build_session_list();
        main_box.append(&session_list);

        self.window.set_child(Some(&main_box));
    }

    fn build_header(&self, user: &User) -> Box {
        let header_box = Box::new(Orientation::Horizontal, 12);
        header_box.set_margin_top(12);
        header_box.set_margin_bottom(12);
        header_box.set_margin_start(12);
        header_box.set_margin_end(12);

        let title_box = Box::new(Orientation::Vertical, 4);
        let title = Label::new(Some("GTKata"));
        title.add_css_class("title-3");
        title.set_halign(gtk::Align::Start);
        title_box.append(&title);

        let user_label = Label::new(Some(&format!("Logged in as: {}", user.email)));
        user_label.add_css_class("dim-label");
        user_label.set_halign(gtk::Align::Start);
        title_box.append(&user_label);

        header_box.append(&title_box);

        // Spacer
        let spacer = Box::new(Orientation::Horizontal, 0);
        spacer.set_hexpand(true);
        header_box.append(&spacer);

        // Logout button
        let logout_button = Button::with_label("Logout");
        logout_button.add_css_class("destructive-action");

        let api = self.api.clone();
        let window = self.window.clone();
        logout_button.connect_clicked(move |_| {
            let api = api.lock().unwrap();
            let _ = api.logout();
            let _ = Config::clear_all();
            window.close();
            std::process::exit(0);
        });

        header_box.append(&logout_button);

        header_box
    }

    fn build_toolbar(&self) -> Box {
        let toolbar = Box::new(Orientation::Horizontal, 12);
        toolbar.set_margin_top(12);
        toolbar.set_margin_bottom(12);
        toolbar.set_margin_start(12);
        toolbar.set_margin_end(12);

        // Search entry
        let search_entry = SearchEntry::new();
        search_entry.set_placeholder_text(Some("Search sessions..."));
        search_entry.set_hexpand(true);
        toolbar.append(&search_entry);

        // Refresh button
        let refresh_button = Button::with_label("Refresh");
        let api = self.api.clone();
        let sessions = self.sessions.clone();
        let window = self.window.clone();

        refresh_button.connect_clicked(move |_| {
            let api = api.clone();
            let sessions = sessions.clone();

            thread::spawn(move || {
                let api = api.lock().unwrap();
                if let Ok(new_sessions) = api.get_sessions() {
                    glib::idle_add_once(move || {
                        *sessions.lock().unwrap() = new_sessions;
                        // TODO: Refresh the list view
                    });
                }
            });
        });

        toolbar.append(&refresh_button);

        // Add session button
        let add_button = Button::with_label("+ New Session");
        add_button.add_css_class("suggested-action");

        let api = self.api.clone();
        let katas = self.katas.clone();
        let sessions = self.sessions.clone();
        let window = self.window.clone();

        add_button.connect_clicked(move |_| {
            let katas = katas.lock().unwrap().clone();
            if katas.is_empty() {
                return;
            }

            let dialog = Rc::new(SessionDialog::new(&window, katas));
            let api = api.clone();
            let sessions = sessions.clone();

            let dialog_clone = dialog.clone();
            dialog.connect_response(move |response| {
                if response == gtk::ResponseType::Ok {
                    if let Some(request) = dialog_clone.get_session_data() {
                        let api = api.clone();
                        let sessions = sessions.clone();

                        thread::spawn(move || {
                            let api = api.lock().unwrap();
                            if let Ok(new_session) = api.create_session(request) {
                                glib::idle_add_once(move || {
                                    sessions.lock().unwrap().push(new_session);
                                    // TODO: Refresh the list view
                                });
                            }
                        });
                    }
                }
                dialog_clone.close();
            });

            dialog.show();
        });

        toolbar.append(&add_button);

        toolbar
    }

    fn build_session_list(&self) -> ScrolledWindow {
        let scrolled = ScrolledWindow::new();
        scrolled.set_vexpand(true);
        scrolled.set_hexpand(true);

        let list_box = ListBox::new();
        list_box.set_selection_mode(gtk::SelectionMode::None);
        list_box.add_css_class("boxed-list");

        scrolled.set_child(Some(&list_box));

        scrolled
    }

    fn load_data(&self) {
        let api = self.api.clone();
        let sessions = self.sessions.clone();
        let katas = self.katas.clone();

        thread::spawn(move || {
            let api_ref = api.lock().unwrap();

            // Load katas
            if let Ok(kata_list) = api_ref.get_katas() {
                *katas.lock().unwrap() = kata_list;
            }

            // Load sessions
            if let Ok(session_list) = api_ref.get_sessions() {
                glib::idle_add_once(move || {
                    *sessions.lock().unwrap() = session_list;
                    // TODO: Populate the list view
                });
            }
        });
    }

    pub fn show(&self) {
        self.window.present();
    }
}
