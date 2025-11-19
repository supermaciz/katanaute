mod api;
mod auth;
mod config;
mod models;

use api::ApiClient;
use auth::{initiate_device_flow, poll_for_authorization};
use chrono::Utc;
use config::Config;
use iced::widget::{button, column, container, row, scrollable, text, text_input};
use iced::{Alignment, Color, Element, Length, Task};
use models::{Kata, Session, SessionInput};

fn main() -> iced::Result {
    iced::application(
        "Katarouille - Kata Training Tracker",
        KatarouillePage::update,
        KatarouillePage::view,
    )
    .theme(|_| iced::Theme::Dark)
    .run_with(KatarouillePage::new)
}

#[derive(Debug)]
struct KatarouillePage {
    config: Config,
    api_client: ApiClient,
    state: AppState,
}

#[derive(Debug, Clone)]
enum AppState {
    Loading,
    Authentication {
        user_code: Option<String>,
        verification_uri: Option<String>,
        polling: bool,
        error: Option<String>,
    },
    SessionList {
        sessions: Vec<Session>,
        selected_session: Option<usize>,
        error: Option<String>,
    },
    SessionCreate {
        katas: Vec<Kata>,
        selected_kata_id: Option<i32>,
        notes: String,
        in_course: bool,
        error: Option<String>,
    },
}

#[derive(Debug, Clone)]
enum Message {
    // Initialization
    ConfigLoaded(Result<Config, String>),
    CheckAuthentication,

    // Authentication
    StartAuthentication,
    DeviceFlowInitiated(Result<auth::DeviceFlowInfo, String>),
    AuthenticationComplete(Result<String, String>),

    // Session list
    SessionsFetched(Result<Vec<Session>, String>),
    SelectSession(usize),
    ShowCreateSession,
    Refresh,

    // Session creation
    KatasFetched(Result<Vec<Kata>, String>),
    SelectKata(i32),
    NotesChanged(String),
    ToggleInCourse,
    SubmitSession,
    SessionCreated(Result<(), String>),
    CancelCreate,

    // General
    Logout,
}

impl KatarouillePage {
    fn new() -> (Self, Task<Message>) {
        let task = Task::perform(
            async { Config::load().map_err(|e| e.to_string()) },
            Message::ConfigLoaded,
        );

        (
            Self {
                config: Config {
                    base_url: String::from("http://localhost:4000/api"),
                    api_token: None,
                },
                api_client: ApiClient::new(String::from("http://localhost:4000/api"), None),
                state: AppState::Loading,
            },
            task,
        )
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::ConfigLoaded(result) => match result {
                Ok(config) => {
                    self.api_client =
                        ApiClient::new(config.base_url.clone(), config.api_token.clone());
                    self.config = config;
                    self.update(Message::CheckAuthentication)
                }
                Err(e) => {
                    self.state = AppState::Authentication {
                        user_code: None,
                        verification_uri: None,
                        polling: false,
                        error: Some(format!("Failed to load config: {}", e)),
                    };
                    Task::none()
                }
            },

            Message::CheckAuthentication => {
                if self.config.api_token.is_some() {
                    // Already authenticated, fetch sessions
                    let api_client = self.api_client.clone();
                    self.state = AppState::Loading;
                    Task::perform(
                        async move { api_client.fetch_sessions().await.map_err(|e| e.to_string()) },
                        Message::SessionsFetched,
                    )
                } else {
                    // Not authenticated, show auth screen
                    self.state = AppState::Authentication {
                        user_code: None,
                        verification_uri: None,
                        polling: false,
                        error: None,
                    };
                    Task::none()
                }
            }

            Message::StartAuthentication => {
                self.state = AppState::Authentication {
                    user_code: None,
                    verification_uri: None,
                    polling: false,
                    error: None,
                };

                let api_client = self.api_client.clone();
                Task::perform(
                    async move {
                        initiate_device_flow(&api_client)
                            .await
                            .map_err(|e| e.to_string())
                    },
                    Message::DeviceFlowInitiated,
                )
            }

            Message::DeviceFlowInitiated(result) => match result {
                Ok(info) => {
                    let device_code = info.device_code.clone();
                    let interval = info.interval;

                    self.state = AppState::Authentication {
                        user_code: Some(info.user_code),
                        verification_uri: Some(info.verification_uri),
                        polling: true,
                        error: None,
                    };

                    let api_client = self.api_client.clone();
                    Task::perform(
                        async move {
                            poll_for_authorization(&api_client, device_code, interval)
                                .await
                                .map_err(|e| e.to_string())
                        },
                        Message::AuthenticationComplete,
                    )
                }
                Err(e) => {
                    self.state = AppState::Authentication {
                        user_code: None,
                        verification_uri: None,
                        polling: false,
                        error: Some(format!("Failed to initiate authentication: {}", e)),
                    };
                    Task::none()
                }
            },

            Message::AuthenticationComplete(result) => match result {
                Ok(token) => {
                    self.api_client.set_token(token.clone());

                    // Save token to config
                    if let Err(e) = self.config.save_token(token.clone()) {
                        eprintln!("Failed to save token: {}", e);
                    }

                    self.config.api_token = Some(token);

                    // Fetch sessions
                    let api_client = self.api_client.clone();
                    self.state = AppState::Loading;
                    Task::perform(
                        async move { api_client.fetch_sessions().await.map_err(|e| e.to_string()) },
                        Message::SessionsFetched,
                    )
                }
                Err(e) => {
                    self.state = AppState::Authentication {
                        user_code: None,
                        verification_uri: None,
                        polling: false,
                        error: Some(format!("Authentication failed: {}", e)),
                    };
                    Task::none()
                }
            },

            Message::SessionsFetched(result) => match result {
                Ok(mut sessions) => {
                    // Sort sessions by date (newest first)
                    sessions.sort_by(|a, b| b.practiced_at.cmp(&a.practiced_at));

                    self.state = AppState::SessionList {
                        sessions,
                        selected_session: None,
                        error: None,
                    };
                    Task::none()
                }
                Err(e) => {
                    self.state = AppState::SessionList {
                        sessions: vec![],
                        selected_session: None,
                        error: Some(format!("Failed to fetch sessions: {}", e)),
                    };
                    Task::none()
                }
            },

            Message::SelectSession(index) => {
                if let AppState::SessionList {
                    ref mut selected_session,
                    ..
                } = self.state
                {
                    *selected_session = Some(index);
                }
                Task::none()
            }

            Message::ShowCreateSession => {
                let api_client = self.api_client.clone();
                self.state = AppState::Loading;
                Task::perform(
                    async move { api_client.fetch_katas().await.map_err(|e| e.to_string()) },
                    Message::KatasFetched,
                )
            }

            Message::Refresh => {
                let api_client = self.api_client.clone();
                self.state = AppState::Loading;
                Task::perform(
                    async move { api_client.fetch_sessions().await.map_err(|e| e.to_string()) },
                    Message::SessionsFetched,
                )
            }

            Message::KatasFetched(result) => match result {
                Ok(katas) => {
                    self.state = AppState::SessionCreate {
                        katas,
                        selected_kata_id: None,
                        notes: String::new(),
                        in_course: false,
                        error: None,
                    };
                    Task::none()
                }
                Err(e) => {
                    self.state = AppState::SessionList {
                        sessions: vec![],
                        selected_session: None,
                        error: Some(format!("Failed to fetch katas: {}", e)),
                    };
                    Task::none()
                }
            },

            Message::SelectKata(kata_id) => {
                if let AppState::SessionCreate {
                    ref mut selected_kata_id,
                    ..
                } = self.state
                {
                    *selected_kata_id = Some(kata_id);
                }
                Task::none()
            }

            Message::NotesChanged(notes) => {
                if let AppState::SessionCreate {
                    notes: ref mut current_notes,
                    ..
                } = self.state
                {
                    *current_notes = notes;
                }
                Task::none()
            }

            Message::ToggleInCourse => {
                if let AppState::SessionCreate {
                    ref mut in_course, ..
                } = self.state
                {
                    *in_course = !*in_course;
                }
                Task::none()
            }

            Message::SubmitSession => {
                if let AppState::SessionCreate {
                    selected_kata_id: Some(kata_id),
                    ref notes,
                    in_course,
                    ..
                } = self.state
                {
                    let session_input = SessionInput {
                        kata_id,
                        in_course,
                        notes: if notes.is_empty() {
                            None
                        } else {
                            Some(notes.clone())
                        },
                        practiced_at: Utc::now(),
                    };

                    let api_client = self.api_client.clone();
                    Task::perform(
                        async move {
                            api_client
                                .create_session(session_input)
                                .await
                                .map(|_| ())
                                .map_err(|e| e.to_string())
                        },
                        Message::SessionCreated,
                    )
                } else {
                    Task::none()
                }
            }

            Message::SessionCreated(result) => match result {
                Ok(_) => {
                    // Refresh session list
                    let api_client = self.api_client.clone();
                    self.state = AppState::Loading;
                    Task::perform(
                        async move { api_client.fetch_sessions().await.map_err(|e| e.to_string()) },
                        Message::SessionsFetched,
                    )
                }
                Err(e) => {
                    if let AppState::SessionCreate { ref mut error, .. } = self.state {
                        *error = Some(format!("Failed to create session: {}", e));
                    }
                    Task::none()
                }
            },

            Message::CancelCreate => {
                // Go back to session list
                let api_client = self.api_client.clone();
                self.state = AppState::Loading;
                Task::perform(
                    async move { api_client.fetch_sessions().await.map_err(|e| e.to_string()) },
                    Message::SessionsFetched,
                )
            }

            Message::Logout => {
                self.api_client.clear_token();
                if let Err(e) = self.config.clear_token() {
                    eprintln!("Failed to clear token: {}", e);
                }
                self.config.api_token = None;

                self.state = AppState::Authentication {
                    user_code: None,
                    verification_uri: None,
                    polling: false,
                    error: None,
                };
                Task::none()
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let content: Element<_> = match &self.state {
            AppState::Loading => container(text("Loading...").size(24))
                .center(Length::Fill)
                .into(),

            AppState::Authentication {
                user_code,
                verification_uri,
                polling,
                error,
            } => {
                let mut col = column![
                    text("Katarouille - Kata Training Tracker").size(32),
                    text("Authentication Required").size(24),
                ]
                .spacing(20)
                .align_x(Alignment::Center);

                if let Some(code) = user_code {
                    col = col.push(text(format!("User Code: {}", code)).size(20));
                }

                if let Some(uri) = verification_uri {
                    col = col.push(text(format!("Visit: {}", uri)).size(16));
                }

                if *polling {
                    col = col.push(text("Waiting for authorization...").size(16));
                } else if user_code.is_none() {
                    col = col.push(button("Login").on_press(Message::StartAuthentication));
                }

                if let Some(err) = error {
                    col = col.push(text(err).size(16).color(Color::from_rgb(1.0, 0.0, 0.0)));
                }

                container(col).center(Length::Fill).padding(20).into()
            }

            AppState::SessionList {
                sessions,
                selected_session,
                error,
            } => {
                let header = row![
                    text("Training Sessions").size(28),
                    button("New Session").on_press(Message::ShowCreateSession),
                    button("Refresh").on_press(Message::Refresh),
                    button("Logout").on_press(Message::Logout),
                ]
                .spacing(10)
                .align_y(Alignment::Center);

                let mut session_list = column![].spacing(10);

                if sessions.is_empty() {
                    session_list = session_list.push(text("No sessions found").size(16));
                } else {
                    for (index, session) in sessions.iter().enumerate() {
                        let is_selected = Some(index) == *selected_session;

                        let kata_name = session
                            .kata
                            .as_ref()
                            .map(|k| k.name.as_str())
                            .unwrap_or("Unknown");

                        let kata_level = session
                            .kata
                            .as_ref()
                            .map(|k| k.level.as_str())
                            .unwrap_or("unknown");

                        let level_color = session
                            .kata
                            .as_ref()
                            .map(|k| {
                                let rgb = k.level_color();
                                Color::from_rgb(rgb[0], rgb[1], rgb[2])
                            })
                            .unwrap_or(Color::from_rgb(0.5, 0.5, 0.5));

                        let date_str = session.practiced_at.format("%Y-%m-%d").to_string();

                        let session_row = row![
                            text(kata_name).size(18),
                            container(text(kata_level).size(14))
                                .padding(5)
                                .style(move |_theme| {
                                    container::Style {
                                        background: Some(level_color.into()),
                                        text_color: Some(Color::WHITE),
                                        ..Default::default()
                                    }
                                }),
                            text(date_str).size(16),
                        ]
                        .spacing(10)
                        .align_y(Alignment::Center);

                        let btn = button(session_row)
                            .on_press(Message::SelectSession(index))
                            .width(Length::Fill);

                        session_list = session_list.push(btn);

                        if is_selected && let Some(notes) = &session.notes {
                            let notes_view = container(
                                text(notes).size(14).color(Color::from_rgb(0.8, 0.8, 0.8)),
                            )
                            .padding(10)
                            .style(|_theme| container::Style {
                                background: Some(Color::from_rgb(0.2, 0.2, 0.2).into()),
                                ..Default::default()
                            });

                            session_list = session_list.push(notes_view);
                        }
                    }
                }

                let mut main_col = column![header, scrollable(session_list)]
                    .spacing(20)
                    .padding(20);

                if let Some(err) = error {
                    main_col =
                        main_col.push(text(err).size(16).color(Color::from_rgb(1.0, 0.0, 0.0)));
                }

                container(main_col).into()
            }

            AppState::SessionCreate {
                katas,
                selected_kata_id,
                notes,
                in_course,
                error,
            } => {
                let header = row![
                    text("Create New Session").size(28),
                    button("Cancel").on_press(Message::CancelCreate),
                ]
                .spacing(10)
                .align_y(Alignment::Center);

                let mut form = column![text("Select Kata:").size(18)].spacing(10);

                for kata in katas {
                    let is_selected = Some(kata.id) == *selected_kata_id;
                    let level_color = {
                        let rgb = kata.level_color();
                        Color::from_rgb(rgb[0], rgb[1], rgb[2])
                    };

                    let kata_row = row![
                        text(&kata.name).size(16),
                        container(text(&kata.level).size(14))
                            .padding(5)
                            .style(move |_theme| {
                                container::Style {
                                    background: Some(level_color.into()),
                                    text_color: Some(Color::WHITE),
                                    ..Default::default()
                                }
                            }),
                    ]
                    .spacing(10)
                    .align_y(Alignment::Center);

                    let btn = button(kata_row)
                        .on_press(Message::SelectKata(kata.id))
                        .style(move |_theme, _status| {
                            let mut style = button::Style::default();
                            if is_selected {
                                style.background = Some(Color::from_rgb(0.3, 0.3, 0.5).into());
                            }
                            style
                        });

                    form = form.push(btn);
                }

                form = form.push(text("Notes (optional):").size(18));
                form = form.push(
                    text_input("Enter notes in Markdown format...", notes)
                        .on_input(Message::NotesChanged)
                        .padding(10),
                );

                form = form.push(
                    button(if *in_course {
                        "Part of Course: Yes"
                    } else {
                        "Part of Course: No"
                    })
                    .on_press(Message::ToggleInCourse),
                );

                if selected_kata_id.is_some() {
                    form = form.push(button("Create Session").on_press(Message::SubmitSession));
                }

                if let Some(err) = error {
                    form = form.push(text(err).size(16).color(Color::from_rgb(1.0, 0.0, 0.0)));
                }

                let main_col = column![header, scrollable(form)].spacing(20).padding(20);

                container(main_col).into()
            }
        };

        content
    }
}
