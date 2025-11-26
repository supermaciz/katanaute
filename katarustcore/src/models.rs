use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Represents a kata in the curriculum
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Kata {
    pub id: i32,
    pub name: String,
    pub level: String,
}

/// Represents a training session
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    pub in_course: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    pub practiced_at: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kata: Option<Kata>,
}

/// Input format for creating a session
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionInput {
    pub kata_id: i32,
    pub in_course: bool,
    pub notes: Option<String>,
    pub practiced_at: DateTime<Utc>,
}

/// Generic API response wrapper
#[derive(Debug, Deserialize)]
pub struct ApiResponse<T> {
    pub data: T,
}

/// User information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub confirmed_at: Option<String>,
}

/// Device code response from initiating device flow
#[derive(Debug, Deserialize)]
pub struct DeviceCodeResponse {
    pub device_code: String,
    pub user_code: String,
    pub verification_uri: String,
    #[allow(dead_code)]
    pub expires_in: i32,
    pub interval: i32,
}

/// Token response from successful authentication
#[derive(Debug, Deserialize)]
pub struct TokenResponse {
    pub access_token: String,
    #[allow(dead_code)]
    pub token_type: String,
    #[allow(dead_code)]
    pub user: User,
}

/// API error response
#[derive(Debug, Deserialize)]
pub struct ErrorResponse {
    pub error: String,
    pub error_description: Option<String>,
}

impl Kata {
    /// Get color for the kata level badge
    pub fn level_color(&self) -> [f32; 3] {
        match self.level.as_str() {
            "yellow" => [1.0, 0.9, 0.0], // Yellow
            "orange" => [1.0, 0.6, 0.0], // Orange
            "green" => [0.0, 0.8, 0.0],  // Green
            "blue" => [0.0, 0.5, 1.0],   // Blue
            "brown" => [0.6, 0.4, 0.2],  // Brown
            "shodan" => [0.1, 0.1, 0.1], // Black
            _ => [0.5, 0.5, 0.5],        // Gray (fallback)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn level_color_yellow() {
        let kata = Kata { id: 1, name: "Test".into(), level: "yellow".into() };
        assert_eq!(kata.level_color(), [1.0, 0.9, 0.0]);
    }

    #[test]
    fn level_color_orange() {
        let kata = Kata { id: 1, name: "Test".into(), level: "orange".into() };
        assert_eq!(kata.level_color(), [1.0, 0.6, 0.0]);
    }

    #[test]
    fn level_color_green() {
        let kata = Kata { id: 1, name: "Test".into(), level: "green".into() };
        assert_eq!(kata.level_color(), [0.0, 0.8, 0.0]);
    }

    #[test]
    fn level_color_blue() {
        let kata = Kata { id: 1, name: "Test".into(), level: "blue".into() };
        assert_eq!(kata.level_color(), [0.0, 0.5, 1.0]);
    }

    #[test]
    fn level_color_brown() {
        let kata = Kata { id: 1, name: "Test".into(), level: "brown".into() };
        assert_eq!(kata.level_color(), [0.6, 0.4, 0.2]);
    }

    #[test]
    fn level_color_shodan() {
        let kata = Kata { id: 1, name: "Test".into(), level: "shodan".into() };
        assert_eq!(kata.level_color(), [0.1, 0.1, 0.1]);
    }

    #[test]
    fn level_color_fallback_for_unknown_level() {
        let kata = Kata { id: 1, name: "Test".into(), level: "foobar".into() };
        assert_eq!(kata.level_color(), [0.5, 0.5, 0.5]);
    }
}