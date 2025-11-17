use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum KataLevel {
    Yellow,
    Orange,
    Green,
    Blue,
    Brown,
    Shodan,
}

impl KataLevel {
    pub fn color(&self) -> &str {
        match self {
            KataLevel::Yellow => "#FFC107",
            KataLevel::Orange => "#FF9800",
            KataLevel::Green => "#4CAF50",
            KataLevel::Blue => "#2196F3",
            KataLevel::Brown => "#795548",
            KataLevel::Shodan => "#000000",
        }
    }

    pub fn to_string(&self) -> &str {
        match self {
            KataLevel::Yellow => "Yellow",
            KataLevel::Orange => "Orange",
            KataLevel::Green => "Green",
            KataLevel::Blue => "Blue",
            KataLevel::Brown => "Brown",
            KataLevel::Shodan => "Shodan",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Kata {
    pub id: i32,
    pub name: String,
    pub level: KataLevel,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub id: i32,
    pub practiced_at: DateTime<Utc>,
    pub in_course: bool,
    pub notes: Option<String>,
    pub kata_id: i32,
    pub kata: Option<Kata>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub email: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthResponse {
    pub access_token: String,
    pub token_type: String,
    pub user: User,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub data: T,
}

#[derive(Debug, Serialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct RegisterRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct CreateSessionRequest {
    pub kata_id: i32,
    pub practiced_at: DateTime<Utc>,
    pub in_course: bool,
    pub notes: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct DeviceCodeResponse {
    pub device_code: String,
    pub user_code: String,
    pub verification_uri: String,
    pub verification_uri_complete: String,
    pub expires_in: i32,
    pub interval: i32,
}

#[derive(Debug, Serialize)]
pub struct DeviceTokenRequest {
    pub device_code: String,
}

#[derive(Debug, Deserialize)]
pub struct DeviceTokenError {
    pub error: String,
}
