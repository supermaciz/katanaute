mod api;
mod auth;
mod config;
mod models;

use adw::prelude::*;
use api::ApiClient;
use auth::{initiate_device_flow, poll_for_authorization};
use config::Config;
use glib::clone;
use gtk4::prelude::*;
use gtk4::{gio, glib};
use libadwaita as adw;
use models::{Kata, Session, SessionInput};
use std::cell::RefCell;
use std::rc::Rc;
use tokio::runtime::Runtime;

const APP_ID: &str = "org.katanaute.GTKata";

fn main() -> glib::ExitCode {
    // Start a Tokio runtime so reqwest and tokio utilities have an executor.
    let runtime = Runtime::new().expect("Failed to create Tokio runtime");
    let _runtime_guard = runtime.enter();

    let app = adw::Application::builder().application_id(APP_ID).build();

    app.connect_activate(build_ui);

    let exit_code = app.run();

    drop(_runtime_guard);
    runtime.shutdown_background();

    exit_code
}

// Application state shared across the UI
struct AppState {
    api_client: ApiClient,
    config: Config,
    sessions: Vec<Session>,
    katas: Vec<Kata>,
}

impl AppState {
    fn new() -> Self {
        let config = Config::load().unwrap_or_else(|_| Config {
            base_url: String::from("http://localhost:4000/api"),
            api_token: None,
        });

        let api_client = ApiClient::new(config.base_url.clone(), config.api_token.clone());

        Self {
            api_client,
            config,
            sessions: Vec::new(),
            katas: Vec::new(),
        }
    }

    fn save_token(&mut self, token: String) {
        self.api_client.set_token(token.clone());
        self.config.api_token = Some(token.clone());
        if let Err(e) = self.config.save_token(token) {
            eprintln!("Failed to save token: {}", e);
        }
    }

    fn clear_token(&mut self) {
        self.api_client.clear_token();
        self.config.api_token = None;
        if let Err(e) = self.config.clear_token() {
            eprintln!("Failed to clear token: {}", e);
        }
    }
}

fn build_ui(app: &adw::Application) {
    let state = Rc::new(RefCell::new(AppState::new()));

    // Create main window
    let window = adw::ApplicationWindow::builder()
        .application(app)
        .title("GTKata - Kata Training Tracker")
        .default_width(800)
        .default_height(600)
        .build();

    // Create navigation view for managing different screens
    let nav_view = adw::NavigationView::new();

    // Check if user is authenticated
    let is_authenticated = state.borrow().config.api_token.is_some();

    if is_authenticated {
        // Show session list
        show_session_list(&nav_view, state.clone());
    } else {
        // Show authentication screen
        show_authentication(&nav_view, state.clone());
    }

    window.set_content(Some(&nav_view));
    window.present();
}

fn show_authentication(nav_view: &adw::NavigationView, state: Rc<RefCell<AppState>>) {
    let content = gtk4::Box::new(gtk4::Orientation::Vertical, 24);
    content.set_valign(gtk4::Align::Center);
    content.set_halign(gtk4::Align::Center);
    content.set_margin_top(48);
    content.set_margin_bottom(48);
    content.set_margin_start(48);
    content.set_margin_end(48);

    // Title
    let title = gtk4::Label::new(Some("GTKata"));
    title.add_css_class("title-1");
    content.append(&title);

    let subtitle = gtk4::Label::new(Some("Kata Training Tracker"));
    subtitle.add_css_class("title-3");
    content.append(&subtitle);

    // Status label
    let status_label = gtk4::Label::new(Some("Please authenticate to continue"));
    status_label.set_wrap(true);
    status_label.set_justify(gtk4::Justification::Center);
    content.append(&status_label);

    // User code display (initially hidden)
    let user_code_box = gtk4::Box::new(gtk4::Orientation::Vertical, 12);
    user_code_box.set_visible(false);

    let user_code_label = gtk4::Label::new(None);
    user_code_label.add_css_class("title-2");
    user_code_box.append(&user_code_label);

    let verification_label = gtk4::Label::new(None);
    verification_label.set_wrap(true);
    verification_label.set_selectable(true);
    user_code_box.append(&verification_label);

    content.append(&user_code_box);

    // Login button
    let login_button = gtk4::Button::with_label("Login");
    login_button.add_css_class("suggested-action");
    login_button.add_css_class("pill");
    login_button.set_halign(gtk4::Align::Center);
    content.append(&login_button);

    // Error label
    let error_label = gtk4::Label::new(None);
    error_label.add_css_class("error");
    error_label.set_wrap(true);
    error_label.set_visible(false);
    content.append(&error_label);

    // Create navigation page
    let page = adw::NavigationPage::builder()
        .title("Authentication")
        .tag("auth")
        .child(&content)
        .can_pop(false)
        .build();

    nav_view.add(&page);

    // Login button handler
    login_button.connect_clicked(clone!(
        #[weak]
        nav_view,
        #[strong]
        state,
        #[weak]
        status_label,
        #[weak]
        user_code_box,
        #[weak]
        user_code_label,
        #[weak]
        verification_label,
        #[weak]
        login_button,
        #[weak]
        error_label,
        move |_| {
            login_button.set_sensitive(false);
            status_label.set_text("Initiating authentication...");
            error_label.set_visible(false);

            let api_client = state.borrow().api_client.clone();

            glib::spawn_future_local(clone!(
                #[weak]
                nav_view,
                #[strong]
                state,
                #[weak]
                status_label,
                #[weak]
                user_code_box,
                #[weak]
                user_code_label,
                #[weak]
                verification_label,
                #[weak]
                login_button,
                #[weak]
                error_label,
                async move {
                    match initiate_device_flow(&api_client).await {
                        Ok(flow_info) => {
                            user_code_label.set_text(&flow_info.user_code);
                            verification_label.set_text(&format!(
                                "Visit {} and enter the code above",
                                flow_info.verification_uri
                            ));
                            user_code_box.set_visible(true);
                            status_label.set_text("Waiting for authorization...");

                            let device_code = flow_info.device_code.clone();
                            let interval = flow_info.interval;

                            glib::spawn_future_local(clone!(
                                #[weak]
                                nav_view,
                                #[strong]
                                state,
                                #[weak]
                                error_label,
                                async move {
                                    let api_client = state.borrow().api_client.clone();
                                    match poll_for_authorization(&api_client, device_code, interval)
                                        .await
                                    {
                                        Ok(token) => {
                                            state.borrow_mut().save_token(token);
                                            show_session_list(&nav_view, state.clone());
                                        }
                                        Err(e) => {
                                            error_label
                                                .set_text(&format!("Authentication failed: {}", e));
                                            error_label.set_visible(true);
                                            login_button.set_sensitive(true);
                                        }
                                    }
                                }
                            ));
                        }
                        Err(e) => {
                            error_label
                                .set_text(&format!("Failed to initiate authentication: {}", e));
                            error_label.set_visible(true);
                            login_button.set_sensitive(true);
                        }
                    }
                }
            ));
        }
    ));
}

fn show_session_list(nav_view: &adw::NavigationView, state: Rc<RefCell<AppState>>) {
    // Clear navigation stack and show session list
    nav_view.pop_to_tag("auth");

    let toolbar_view = adw::ToolbarView::new();

    // Header bar
    let header_bar = adw::HeaderBar::new();

    // Menu button with logout
    let menu = gio::Menu::new();
    menu.append(Some("Logout"), Some("app.logout"));

    let menu_button = gtk4::MenuButton::new();
    menu_button.set_icon_name("open-menu-symbolic");
    menu_button.set_menu_model(Some(&menu));
    header_bar.pack_end(&menu_button);

    // Refresh button
    let refresh_button = gtk4::Button::from_icon_name("view-refresh-symbolic");
    refresh_button.set_tooltip_text(Some("Refresh"));
    header_bar.pack_end(&refresh_button);

    // New session button
    let new_button = gtk4::Button::from_icon_name("list-add-symbolic");
    new_button.set_tooltip_text(Some("New Session"));
    new_button.add_css_class("suggested-action");
    header_bar.pack_start(&new_button);

    toolbar_view.add_top_bar(&header_bar);

    // Main content area
    let scrolled = gtk4::ScrolledWindow::new();
    scrolled.set_vexpand(true);

    let list_box = gtk4::ListBox::new();
    list_box.add_css_class("boxed-list");
    list_box.set_selection_mode(gtk4::SelectionMode::None);

    scrolled.set_child(Some(&list_box));

    // Status box for loading/empty state
    let status_box = gtk4::Box::new(gtk4::Orientation::Vertical, 12);
    status_box.set_valign(gtk4::Align::Center);
    status_box.set_halign(gtk4::Align::Center);
    status_box.set_vexpand(true);

    let status_label = gtk4::Label::new(Some("Loading sessions..."));
    status_label.add_css_class("title-3");
    status_box.append(&status_label);

    let content_stack = gtk4::Stack::new();
    content_stack.add_named(&scrolled, Some("sessions"));
    content_stack.add_named(&status_box, Some("status"));
    content_stack.set_visible_child_name("status");

    toolbar_view.set_content(Some(&content_stack));

    let page = adw::NavigationPage::builder()
        .title("Training Sessions")
        .tag("sessions")
        .child(&toolbar_view)
        .can_pop(false)
        .build();

    // Logout action
    let logout_action = gio::SimpleAction::new("logout", None);
    logout_action.connect_activate(clone!(
        #[weak]
        nav_view,
        #[strong]
        state,
        move |_, _| {
            state.borrow_mut().clear_token();
            show_authentication(&nav_view, state.clone());
        }
    ));

    if let Some(window) = nav_view.root().and_downcast::<adw::ApplicationWindow>() {
        if let Some(app) = window.application() {
            app.add_action(&logout_action);
        }
    }

    nav_view.add(&page);

    // Load sessions
    load_sessions(
        state.clone(),
        list_box.clone(),
        status_label.clone(),
        content_stack.clone(),
    );

    // Refresh button handler
    refresh_button.connect_clicked(clone!(
        #[strong]
        state,
        #[weak]
        list_box,
        #[weak]
        status_label,
        #[weak]
        content_stack,
        move |_| {
            load_sessions(
                state.clone(),
                list_box.clone(),
                status_label.clone(),
                content_stack.clone(),
            );
        }
    ));

    // New session button handler
    new_button.connect_clicked(clone!(
        #[weak]
        nav_view,
        #[strong]
        state,
        move |_| {
            show_session_create(&nav_view, state.clone());
        }
    ));
}

fn load_sessions(
    state: Rc<RefCell<AppState>>,
    list_box: gtk4::ListBox,
    status_label: gtk4::Label,
    content_stack: gtk4::Stack,
) {
    status_label.set_text("Loading sessions...");
    content_stack.set_visible_child_name("status");

    let api_client = state.borrow().api_client.clone();

    glib::spawn_future_local(clone!(
        #[strong]
        state,
        #[weak]
        list_box,
        #[weak]
        status_label,
        #[weak]
        content_stack,
        async move {
            match api_client.fetch_sessions().await {
                Ok(mut sessions) => {
                    // Sort by date (newest first)
                    sessions.sort_by(|a, b| b.practiced_at.cmp(&a.practiced_at));
                    state.borrow_mut().sessions = sessions.clone();

                    // Clear list
                    while let Some(child) = list_box.first_child() {
                        list_box.remove(&child);
                    }

                    if sessions.is_empty() {
                        status_label.set_text("No sessions found");
                        content_stack.set_visible_child_name("status");
                    } else {
                        // Add sessions to list
                        for session in sessions {
                            let row = create_session_row(&session);
                            list_box.append(&row);
                        }
                        content_stack.set_visible_child_name("sessions");
                    }
                }
                Err(e) => {
                    status_label.set_text(&format!("Error loading sessions: {}", e));
                    content_stack.set_visible_child_name("status");
                }
            }
        }
    ));
}

fn create_session_row(session: &Session) -> adw::ActionRow {
    let row = adw::ActionRow::new();

    let kata_name = session
        .kata
        .as_ref()
        .map(|k| k.name.as_str())
        .unwrap_or("Unknown");
    row.set_title(kata_name);

    let date_str = session.practiced_at.format("%Y-%m-%d").to_string();
    row.set_subtitle(&date_str);

    // Add kata level badge
    if let Some(kata) = &session.kata {
        let level_label = gtk4::Label::new(Some(&kata.level));
        level_label.add_css_class("caption");
        level_label.add_css_class("pill");

        // Add color styling based on level
        let color_class = match kata.level.as_str() {
            "yellow" => "warning",
            "orange" => "warning",
            "green" => "success",
            "blue" => "accent",
            "brown" => "error",
            "shodan" => "error",
            _ => "",
        };
        if !color_class.is_empty() {
            level_label.add_css_class(color_class);
        }

        row.add_suffix(&level_label);
    }

    // Add in-course indicator
    if session.in_course {
        let course_icon = gtk4::Image::from_icon_name("emblem-default-symbolic");
        course_icon.set_tooltip_text(Some("Part of course"));
        row.add_suffix(&course_icon);
    }

    // Add chevron for expandable row
    let chevron = gtk4::Image::from_icon_name("go-next-symbolic");
    row.add_suffix(&chevron);

    // Make row activatable to show details
    row.set_activatable(true);

    row
}

fn show_session_create(nav_view: &adw::NavigationView, state: Rc<RefCell<AppState>>) {
    let toolbar_view = adw::ToolbarView::new();

    let header_bar = adw::HeaderBar::new();
    toolbar_view.add_top_bar(&header_bar);

    // Status box for loading
    let status_box = gtk4::Box::new(gtk4::Orientation::Vertical, 12);
    status_box.set_valign(gtk4::Align::Center);
    status_box.set_halign(gtk4::Align::Center);
    status_box.set_vexpand(true);

    let status_label = gtk4::Label::new(Some("Loading katas..."));
    status_label.add_css_class("title-3");
    status_box.append(&status_label);

    let content_stack = gtk4::Stack::new();
    content_stack.add_named(&status_box, Some("loading"));

    toolbar_view.set_content(Some(&content_stack));

    let page = adw::NavigationPage::builder()
        .title("New Session")
        .tag("create")
        .child(&toolbar_view)
        .build();

    nav_view.push(&page);

    // Load katas
    let api_client = state.borrow().api_client.clone();

    glib::spawn_future_local(clone!(
        #[weak]
        nav_view,
        #[strong]
        state,
        #[weak]
        content_stack,
        #[weak]
        status_label,
        async move {
            match api_client.fetch_katas().await {
                Ok(katas) => {
                    state.borrow_mut().katas = katas.clone();
                    build_session_form(&nav_view, state.clone(), &content_stack, katas);
                }
                Err(e) => {
                    status_label.set_text(&format!("Error loading katas: {}", e));
                }
            }
        }
    ));
}

fn build_session_form(
    nav_view: &adw::NavigationView,
    state: Rc<RefCell<AppState>>,
    content_stack: &gtk4::Stack,
    katas: Vec<Kata>,
) {
    let form_box = gtk4::Box::new(gtk4::Orientation::Vertical, 24);
    form_box.set_margin_top(24);
    form_box.set_margin_bottom(24);
    form_box.set_margin_start(24);
    form_box.set_margin_end(24);

    // Kata selection group
    let kata_group = adw::PreferencesGroup::new();
    kata_group.set_title("Select Kata");

    let selected_kata_id: Rc<RefCell<Option<i32>>> = Rc::new(RefCell::new(None));
    let mut first_check: Option<gtk4::CheckButton> = None;

    for kata in katas {
        let row = adw::ActionRow::new();
        row.set_title(&kata.name);

        let level_label = gtk4::Label::new(Some(&kata.level));
        level_label.add_css_class("caption");
        row.add_suffix(&level_label);

        let check = gtk4::CheckButton::new();
        row.add_prefix(&check);

        // Make check buttons mutually exclusive (radio button behavior)
        if let Some(ref first) = first_check {
            check.set_group(Some(first));
        } else {
            first_check = Some(check.clone());
        }

        let kata_id = kata.id;
        check.connect_toggled(clone!(
            #[strong]
            selected_kata_id,
            move |check| {
                if check.is_active() {
                    *selected_kata_id.borrow_mut() = Some(kata_id);
                }
            }
        ));

        kata_group.add(&row);
    }

    form_box.append(&kata_group);

    // Notes entry
    let notes_group = adw::PreferencesGroup::new();
    notes_group.set_title("Notes (optional)");

    let notes_entry = gtk4::TextView::new();
    notes_entry.set_wrap_mode(gtk4::WrapMode::Word);
    notes_entry.set_height_request(120);
    notes_entry.add_css_class("card");

    let scrolled = gtk4::ScrolledWindow::new();
    scrolled.set_child(Some(&notes_entry));
    scrolled.set_height_request(120);

    notes_group.add(&scrolled);
    form_box.append(&notes_group);

    // In-course switch
    let course_row = adw::ActionRow::new();
    course_row.set_title("Part of Course");
    course_row.set_subtitle("Mark if this session is part of structured training");

    let course_switch = gtk4::Switch::new();
    course_switch.set_valign(gtk4::Align::Center);
    course_row.add_suffix(&course_switch);
    course_row.set_activatable_widget(Some(&course_switch));

    form_box.append(&course_row);

    // Create button
    let create_button = gtk4::Button::with_label("Create Session");
    create_button.add_css_class("suggested-action");
    create_button.add_css_class("pill");
    create_button.set_halign(gtk4::Align::Center);
    form_box.append(&create_button);

    // Error label
    let error_label = gtk4::Label::new(None);
    error_label.add_css_class("error");
    error_label.set_wrap(true);
    error_label.set_visible(false);
    form_box.append(&error_label);

    let scrolled_form = gtk4::ScrolledWindow::new();
    scrolled_form.set_child(Some(&form_box));

    content_stack.add_named(&scrolled_form, Some("form"));
    content_stack.set_visible_child_name("form");

    // Create button handler
    create_button.connect_clicked(clone!(
        #[weak]
        nav_view,
        #[strong]
        state,
        #[strong]
        selected_kata_id,
        #[weak]
        notes_entry,
        #[weak]
        course_switch,
        #[weak]
        error_label,
        #[weak]
        create_button,
        move |_| {
            let kata_id = selected_kata_id.borrow().clone();
            if kata_id.is_none() {
                error_label.set_text("Please select a kata");
                error_label.set_visible(true);
                return;
            }

            let buffer = notes_entry.buffer();
            let notes_text = buffer
                .text(&buffer.start_iter(), &buffer.end_iter(), false)
                .to_string();
            let notes = if notes_text.is_empty() {
                None
            } else {
                Some(notes_text)
            };

            let session_input = SessionInput {
                kata_id: kata_id.unwrap(),
                in_course: course_switch.is_active(),
                notes,
                practiced_at: chrono::Utc::now(),
            };

            create_button.set_sensitive(false);
            error_label.set_visible(false);

            let api_client = state.borrow().api_client.clone();

            glib::spawn_future_local(clone!(
                #[weak]
                nav_view,
                #[weak]
                error_label,
                #[weak]
                create_button,
                async move {
                    match api_client.create_session(session_input).await {
                        Ok(_) => {
                            nav_view.pop();
                            // Refresh session list
                            if let Some(_page) = nav_view.find_page("sessions") {
                                // The list will be refreshed when page is shown
                            }
                        }
                        Err(e) => {
                            error_label.set_text(&format!("Failed to create session: {}", e));
                            error_label.set_visible(true);
                            create_button.set_sensitive(true);
                        }
                    }
                }
            ));
        }
    ));
}
