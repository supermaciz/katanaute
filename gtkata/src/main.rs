mod markdown;

use adw::prelude::*;
use glib::clone;
use gtk4::prelude::*;
use gtk4::{gdk, gio, glib};
use katarustcore::{
    ApiClient, Config, Kata, Session, SessionInput, initiate_device_flow, poll_for_authorization,
};
use libadwaita as adw;
use std::cell::RefCell;
use std::rc::Rc;
use tokio::runtime::Runtime;

const APP_ID: &str = "org.katanaute.GTKata";

const BELT_CSS: &str = r#"
.belt-pill {
  padding: 3px 10px;
  min-width: 56px;
  min-height: 0;
  line-height: 1.0;
  border-radius: 999px;
  text-align: center;
}

/* Light belts: dark text */
.belt-yellow {
  background-color: #ffe500;
  color: #000000;
}

.belt-orange {
  background-color: #ff9900;
  color: #000000;
}

/* Darker belts: light text */
.belt-green {
  background-color: #00a63a;
  color: #ffffff;
}

.belt-blue {
  background-color: #0068d9;
  color: #ffffff;
}

.belt-brown {
  background-color: #795548;
  color: #ffffff;
}

/* Shodan: near-black so it's readable in both modes */
.belt-shodan {
  background-color: #191919;
  color: #ffffff;
}
"#;

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

/// Replace the entire navigation stack with the provided page so root transitions never hang.
fn set_root_page(nav_view: &adw::NavigationView, page: &adw::NavigationPage) {
    nav_view.replace(std::slice::from_ref(page));
}

fn build_ui(app: &adw::Application) {
    // Load custom CSS for belt badges
    let provider = gtk4::CssProvider::new();
    #[allow(deprecated)]
    provider.load_from_data(BELT_CSS);
    if let Some(display) = gdk::Display::default() {
        gtk4::style_context_add_provider_for_display(
            &display,
            &provider,
            gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
    }

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

    // Install logout action once on the application
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
    app.add_action(&logout_action);

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

fn show_session_list(nav_view: &adw::NavigationView, state: Rc<RefCell<AppState>>) {
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

    set_root_page(nav_view, &page);

    // Load sessions
    load_sessions(
        state.clone(),
        list_box.clone(),
        status_label.clone(),
        content_stack.clone(),
        nav_view,
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
        #[weak]
        nav_view,
        move |_| {
            load_sessions(
                state.clone(),
                list_box.clone(),
                status_label.clone(),
                content_stack.clone(),
                &nav_view,
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
    verification_label.set_use_markup(true);
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

    login_button.connect_clicked(clone!(
        #[strong]
        state,
        #[weak]
        nav_view,
        #[weak]
        login_button,
        #[weak]
        status_label,
        #[weak]
        user_code_box,
        #[weak]
        user_code_label,
        #[weak]
        verification_label,
        #[weak]
        error_label,
        move |_| {
            error_label.set_visible(false);
            user_code_box.set_visible(false);
            status_label.set_text("Requesting verification code...");
            login_button.set_sensitive(false);

            let api_client = state.borrow().api_client.clone();

            glib::spawn_future_local(clone!(
                #[strong]
                state,
                #[weak]
                nav_view,
                #[weak]
                login_button,
                #[weak]
                status_label,
                #[weak]
                user_code_box,
                #[weak]
                user_code_label,
                #[weak]
                verification_label,
                #[weak]
                error_label,
                async move {
                    match initiate_device_flow(&api_client).await {
                        Ok(flow_info) => {
                            let device_code = flow_info.device_code.clone();
                            let interval = flow_info.interval;

                            user_code_label.set_text(&flow_info.user_code);
                            let verification_url = format!(
                                "{}?user_code={}",
                                flow_info.verification_uri, flow_info.user_code
                            );
                            let escaped_url = glib::markup_escape_text(&verification_url);
                            let verification_markup = format!(
                                "Visit <a href=\"{url}\">{display}</a> and enter the code above",
                                url = verification_url,
                                display = escaped_url
                            );
                            verification_label.set_markup(&verification_markup);
                            user_code_box.set_visible(true);
                            status_label.set_text("Waiting for authorization...");

                            match poll_for_authorization(&api_client, device_code, interval).await {
                                Ok(token) => {
                                    status_label
                                        .set_text("Authentication successful! Loading sessions...");
                                    state.borrow_mut().save_token(token);
                                    show_session_list(&nav_view, state.clone());
                                }
                                Err(e) => {
                                    error_label.set_text(&format!(
                                        "Failed to complete authorization: {}",
                                        e
                                    ));
                                    error_label.set_visible(true);
                                    status_label
                                        .set_text("Authorization failed. Please try again.");
                                    login_button.set_sensitive(true);
                                }
                            }
                        }
                        Err(e) => {
                            error_label.set_text(&format!("Failed to initiate login: {}", e));
                            error_label.set_visible(true);
                            status_label.set_text("Please authenticate to continue");
                            login_button.set_sensitive(true);
                        }
                    }
                }
            ));
        }
    ));

    // Create navigation page
    let page = adw::NavigationPage::builder()
        .title("Authentication")
        .tag("auth")
        .child(&content)
        .can_pop(false)
        .build();

    set_root_page(nav_view, &page);
}

fn load_sessions(
    state: Rc<RefCell<AppState>>,
    list_box: gtk4::ListBox,
    status_label: gtk4::Label,
    content_stack: gtk4::Stack,
    nav_view: &adw::NavigationView,
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
        #[weak]
        nav_view,
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
                            let row = create_session_row(&session, &nav_view, state.clone());
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

fn create_session_row(
    session: &Session,
    nav_view: &adw::NavigationView,
    state: Rc<RefCell<AppState>>,
) -> adw::ActionRow {
    let row = adw::ActionRow::new();

    let kata_name = session
        .kata
        .as_ref()
        .map(|k| k.name.as_str())
        .unwrap_or("Unknown");
    row.set_title(kata_name);

    let date_str = session.practiced_at.format("%Y-%m-%d").to_string();
    row.set_subtitle(&date_str);

    // Add kata level badge and in-course indicator in a container for alignment
    if let Some(kata) = &session.kata {
        let level_label = gtk4::Label::new(Some(&kata.level));
        level_label.add_css_class("caption");
        level_label.add_css_class("belt-pill");
        level_label.set_valign(gtk4::Align::Center);

        // Belt-specific color classes
        let belt_class = match kata.level.as_str() {
            "yellow" => "belt-yellow",
            "orange" => "belt-orange",
            "green" => "belt-green",
            "blue" => "belt-blue",
            "brown" => "belt-brown",
            "shodan" => "belt-shodan",
            _ => "",
        };
        if !belt_class.is_empty() {
            level_label.add_css_class(belt_class);
        }

        // Create a container for belt badge and optional in-course icon
        let badge_container = gtk4::Box::new(gtk4::Orientation::Horizontal, 6);
        badge_container.set_valign(gtk4::Align::Center);

        // Keep belts visually aligned by placing the icon before the badge
        if session.in_course {
            let course_icon = gtk4::Image::from_icon_name("emblem-default-symbolic");
            course_icon.set_tooltip_text(Some("Part of course"));
            badge_container.append(&course_icon);
        }
        badge_container.append(&level_label);

        row.add_suffix(&badge_container);
    }

    // Add chevron for expandable row
    let chevron = gtk4::Image::from_icon_name("go-next-symbolic");
    row.add_suffix(&chevron);

    // Make row activatable to show details
    row.set_activatable(true);

    // Connect click handler to show session details
    let session_clone = session.clone();
    row.connect_activated(clone!(
        #[weak]
        nav_view,
        #[strong]
        state,
        move |_| {
            show_session_details(&nav_view, state.clone(), session_clone.clone());
        }
    ));

    row
}

fn show_session_details(
    nav_view: &adw::NavigationView,
    state: Rc<RefCell<AppState>>,
    session: Session,
) {
    let toolbar_view = adw::ToolbarView::new();

    // Header bar with back button
    let header_bar = adw::HeaderBar::new();
    toolbar_view.add_top_bar(&header_bar);

    // Main content
    let content_box = gtk4::Box::new(gtk4::Orientation::Vertical, 24);
    content_box.set_margin_top(24);
    content_box.set_margin_bottom(24);
    content_box.set_margin_start(24);
    content_box.set_margin_end(24);

    // Session info group
    let info_group = adw::PreferencesGroup::new();
    info_group.set_title("Session Details");

    // Kata name row
    let kata_row = adw::ActionRow::new();
    kata_row.set_title("Kata");
    let kata_name = session
        .kata
        .as_ref()
        .map(|k| k.name.as_str())
        .unwrap_or("Unknown");
    kata_row.set_subtitle(kata_name);

    // Add kata level badge
    if let Some(kata) = &session.kata {
        let level_label = gtk4::Label::new(Some(&kata.level));
        level_label.add_css_class("caption");
        level_label.add_css_class("belt-pill");
        level_label.set_valign(gtk4::Align::Center);

        let belt_class = match kata.level.as_str() {
            "yellow" => "belt-yellow",
            "orange" => "belt-orange",
            "green" => "belt-green",
            "blue" => "belt-blue",
            "brown" => "belt-brown",
            "shodan" => "belt-shodan",
            _ => "",
        };
        if !belt_class.is_empty() {
            level_label.add_css_class(belt_class);
        }

        kata_row.add_suffix(&level_label);
    }
    info_group.add(&kata_row);

    // Date and time row
    let datetime_row = adw::ActionRow::new();
    datetime_row.set_title("Practice Date");
    let datetime_str = session
        .practiced_at
        .format("%Y-%m-%d %H:%M UTC")
        .to_string();
    datetime_row.set_subtitle(&datetime_str);
    info_group.add(&datetime_row);

    // In-course row
    let course_row = adw::ActionRow::new();
    course_row.set_title("Part of Course");
    if session.in_course {
        course_row.set_subtitle("Yes");
        let course_icon = gtk4::Image::from_icon_name("emblem-default-symbolic");
        course_row.add_suffix(&course_icon);
    } else {
        course_row.set_subtitle("No");
    }
    info_group.add(&course_row);

    content_box.append(&info_group);

    // Notes group (if notes exist)
    if let Some(notes) = &session.notes
        && !notes.trim().is_empty()
    {
        let notes_group = adw::PreferencesGroup::new();
        notes_group.set_title("Notes");

        // Try to render as markdown, fallback to plain text
        match markdown::render_input(notes, markdown::RenderConfig::default()) {
            Ok(viewport) => {
                // Wrap in clamp to limit width and scrolled window for scrolling
                let clamp = adw::Clamp::builder()
                    .maximum_size(800)
                    .tightening_threshold(400)
                    .build();

                let scrolled = gtk4::ScrolledWindow::new();
                scrolled.set_policy(gtk4::PolicyType::Automatic, gtk4::PolicyType::Automatic);
                scrolled.set_min_content_height(160);
                scrolled.set_vexpand(true);
                viewport.set_vexpand(true);
                scrolled.set_child(Some(&viewport));

                // Allow this group to claim extra vertical space in the page
                notes_group.set_vexpand(true);

                clamp.set_child(Some(&scrolled));
                notes_group.add(&clamp);
            }
            Err(_) => {
                // Fallback to plain text label if markdown rendering fails
                let notes_label = gtk4::Label::new(Some(notes));
                notes_label.set_wrap(true);
                notes_label.set_wrap_mode(gtk4::pango::WrapMode::Word);
                notes_label.set_xalign(0.0); // Left-align
                notes_label.set_margin_top(12);
                notes_label.set_margin_bottom(12);
                notes_label.set_margin_start(12);
                notes_label.set_margin_end(12);
                notes_label.add_css_class("body");

                notes_group.add(&notes_label);
            }
        }

        content_box.append(&notes_group);
    }

    // Danger zone group for deletion
    let danger_group = adw::PreferencesGroup::new();

    // Error label for deletion failures
    let delete_error_label = gtk4::Label::new(None);
    delete_error_label.add_css_class("error");
    delete_error_label.set_wrap(true);
    delete_error_label.set_visible(false);
    danger_group.add(&delete_error_label);

    // Delete session button
    let delete_button = gtk4::Button::with_label("Delete Session");
    delete_button.add_css_class("destructive-action");
    delete_button.add_css_class("pill");
    delete_button.set_halign(gtk4::Align::Start);

    // Disable button if session has no ID (shouldn't happen, but be safe)
    if session.id.is_none() {
        delete_button.set_sensitive(false);
    }

    danger_group.add(&delete_button);
    content_box.append(&danger_group);

    // Create scrolled window for content
    let scrolled = gtk4::ScrolledWindow::new();
    scrolled.set_child(Some(&content_box));
    scrolled.set_vexpand(true);

    toolbar_view.set_content(Some(&scrolled));

    let page = adw::NavigationPage::builder()
        .title("Session Details")
        .tag("session-details")
        .child(&toolbar_view)
        .build();

    // Delete button handler
    if let Some(session_id) = session.id {
        delete_button.connect_clicked(clone!(
            #[weak]
            nav_view,
            #[strong]
            state,
            #[weak]
            delete_error_label,
            #[weak]
            delete_button,
            move |_| {
                // Create confirmation dialog
                let dialog = adw::AlertDialog::builder()
                    .heading("Delete Session?")
                    .body("This action cannot be undone. The session will be permanently deleted.")
                    .build();

                // Add responses
                dialog.add_responses(&[("cancel", "Cancel"), ("delete", "Delete")]);

                // Style delete button as destructive
                dialog.set_response_appearance("delete", adw::ResponseAppearance::Destructive);

                // Set default and close responses
                dialog.set_default_response(Some("cancel"));
                dialog.set_close_response("cancel");

                // Handle response
                dialog.connect_response(
                    None,
                    clone!(
                        #[weak]
                        nav_view,
                        #[strong]
                        state,
                        #[weak]
                        delete_error_label,
                        #[weak]
                        delete_button,
                        move |_, response| {
                            if response == "delete" {
                                // Disable button and hide errors during deletion
                                delete_button.set_sensitive(false);
                                delete_error_label.set_visible(false);

                                let api_client = state.borrow().api_client.clone();

                                glib::spawn_future_local(clone!(
                                    #[weak]
                                    nav_view,
                                    #[weak]
                                    delete_error_label,
                                    #[weak]
                                    delete_button,
                                    #[strong]
                                    state,
                                    async move {
                                        match api_client.delete_session(session_id).await {
                                            Ok(_) => {
                                                // Remove session from state
                                                state
                                                    .borrow_mut()
                                                    .sessions
                                                    .retain(|s| s.id != Some(session_id));

                                                // Go back to session list and refresh it
                                                show_session_list(&nav_view, state.clone());
                                            }
                                            Err(e) => {
                                                delete_error_label.set_text(&format!(
                                                    "Failed to delete session: {}",
                                                    e
                                                ));
                                                delete_error_label.set_visible(true);
                                                delete_button.set_sensitive(true);
                                            }
                                        }
                                    }
                                ));
                            }
                        }
                    ),
                );

                dialog.present(Some(&nav_view));
            }
        ));
    }

    nav_view.push(&page);
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
        level_label.add_css_class("belt-pill");
        level_label.set_valign(gtk4::Align::Center);

        let belt_class = match kata.level.as_str() {
            "yellow" => "belt-yellow",
            "orange" => "belt-orange",
            "green" => "belt-green",
            "blue" => "belt-blue",
            "brown" => "belt-brown",
            "shodan" => "belt-shodan",
            _ => "",
        };
        if !belt_class.is_empty() {
            level_label.add_css_class(belt_class);
        }

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

    // Date and time selection group
    let datetime_group = adw::PreferencesGroup::new();
    datetime_group.set_title("Practice Date and Time");

    // Create calendar for date selection
    let calendar = gtk4::Calendar::new();
    calendar.set_show_heading(true);
    calendar.set_show_day_names(true);
    calendar.set_show_week_numbers(false);

    // Set calendar to current date using select_day (deprecated in v4_20, but kept for compatibility)
    let now =
        glib::DateTime::now_local().unwrap_or_else(|_| glib::DateTime::from_unix_utc(0).unwrap());
    #[allow(deprecated)]
    calendar.select_day(&now);

    // Create time entry
    let time_entry = gtk4::Entry::new();
    time_entry.set_placeholder_text(Some("HH:MM (24-hour format)"));
    time_entry.set_text(&now.format("%H:%M").unwrap_or_default());
    time_entry.set_max_length(5);

    // Date row with calendar
    let date_row = adw::ActionRow::new();
    date_row.set_title("Date");
    date_row.set_subtitle(&now.format("%Y-%m-%d").unwrap_or_default());

    let date_button = gtk4::Button::new();
    date_button.set_label("Change Date");
    date_button.add_css_class("flat");

    // Create popover for calendar
    let calendar_popover = gtk4::Popover::new();
    calendar_popover.set_child(Some(&calendar));
    calendar_popover.set_parent(&date_button);

    date_button.connect_clicked(clone!(
        #[weak]
        calendar_popover,
        move |_| {
            calendar_popover.popup();
        }
    ));

    date_row.add_suffix(&date_button);
    datetime_group.add(&date_row);

    // Time row
    let time_row = adw::ActionRow::new();
    time_row.set_title("Time");
    time_row.add_suffix(&time_entry);
    datetime_group.add(&time_row);

    // Store selected datetime
    let selected_datetime: Rc<RefCell<Option<glib::DateTime>>> =
        Rc::new(RefCell::new(Some(now.clone())));

    // Update selected datetime when calendar date changes
    calendar.connect_day_selected(clone!(
        #[strong]
        selected_datetime,
        #[weak]
        time_entry,
        #[weak]
        date_row,
        move |calendar| {
            let selected_date = calendar.date();
            let time_text = time_entry.text().to_string();

            // Parse time or use current time
            let (hour, minute) = if let Ok((h, m)) = parse_time(&time_text) {
                (h, m)
            } else {
                (now.hour(), now.minute())
            };

            // Create new datetime with selected date and time using from_unix_local
            let timestamp = selected_date.to_unix();
            if let Ok(base_datetime) = glib::DateTime::from_unix_local(timestamp)
                && let Ok(combined_datetime) = glib::DateTime::new(
                    &base_datetime.timezone(),
                    selected_date.year(),
                    selected_date.month(),
                    selected_date.day_of_month(),
                    hour,
                    minute,
                    0.0,
                )
            {
                *selected_datetime.borrow_mut() = Some(combined_datetime.clone());
                // Update the subtitle to show selected date
                date_row.set_subtitle(&combined_datetime.format("%Y-%m-%d").unwrap_or_default());
            }
        }
    ));

    // Update selected datetime when time entry changes
    time_entry.connect_changed(clone!(
        #[strong]
        selected_datetime,
        #[weak]
        calendar,
        move |entry| {
            let time_text = entry.text().to_string();
            if let Ok((hour, minute)) = parse_time(&time_text) {
                let calendar_date = calendar.date();
                let timestamp = calendar_date.to_unix();
                if let Ok(base_datetime) = glib::DateTime::from_unix_local(timestamp)
                    && let Ok(combined_datetime) = glib::DateTime::new(
                        &base_datetime.timezone(),
                        calendar_date.year(),
                        calendar_date.month(),
                        calendar_date.day_of_month(),
                        hour,
                        minute,
                        0.0,
                    )
                {
                    *selected_datetime.borrow_mut() = Some(combined_datetime);
                }
            }
        }
    ));

    form_box.append(&datetime_group);

    // Notes entry
    let notes_group = adw::PreferencesGroup::new();
    notes_group.set_title("Notes (optional)");

    let notes_entry = gtk4::TextView::new();
    notes_entry.set_wrap_mode(gtk4::WrapMode::Word);
    notes_entry.set_vexpand(true);
    notes_entry.add_css_class("card");

    let scrolled = gtk4::ScrolledWindow::new();
    scrolled.set_child(Some(&notes_entry));
    scrolled.set_vexpand(true);
    scrolled.set_height_request(200);

    notes_group.set_vexpand(true);
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
        #[strong]
        selected_datetime,
        #[weak]
        notes_entry,
        #[weak]
        course_switch,
        #[weak]
        error_label,
        #[weak]
        create_button,
        move |_| {
            let kata_id = *selected_kata_id.borrow();
            if kata_id.is_none() {
                error_label.set_text("Please select a kata");
                error_label.set_visible(true);
                return;
            }

            let selected_dt = selected_datetime.borrow().clone();
            let practiced_at = if let Some(dt) = selected_dt {
                // Convert glib::DateTime to chrono::DateTime<Utc>
                let timestamp = dt.to_unix();
                chrono::DateTime::from_timestamp(timestamp, 0).unwrap_or_else(chrono::Utc::now)
            } else {
                chrono::Utc::now()
            };

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
                practiced_at,
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

/// Parse time string in HH:MM format and return (hour, minute)
fn parse_time(time_str: &str) -> Result<(i32, i32), ()> {
    let parts: Vec<&str> = time_str.split(':').collect();
    if parts.len() != 2 {
        return Err(());
    }

    let hour: i32 = parts[0].parse().map_err(|_| ())?;
    let minute: i32 = parts[1].parse().map_err(|_| ())?;

    // Validate ranges
    if !(0..=23).contains(&hour) || !(0..=59).contains(&minute) {
        return Err(());
    }

    Ok((hour, minute))
}
