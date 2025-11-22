use crate::models::{
    ApiResponse, DeviceCodeResponse, ErrorResponse, Kata, Session, SessionInput, TokenResponse,
};
use reqwest::Client;
use std::collections::HashMap;

/// API client for Katanaute backend
#[derive(Clone, Debug)]
pub struct ApiClient {
    client: Client,
    base_url: String,
    token: Option<String>,
}

impl ApiClient {
    /// Create a new API client
    pub fn new(base_url: String, token: Option<String>) -> Self {
        Self {
            client: Client::new(),
            base_url,
            token,
        }
    }

    /// Update the API token
    pub fn set_token(&mut self, token: String) {
        self.token = Some(token);
    }

    /// Clear the API token
    pub fn clear_token(&mut self) {
        self.token = None;
    }

    /// Fetch all sessions
    pub async fn fetch_sessions(&self) -> Result<Vec<Session>, Box<dyn std::error::Error>> {
        let url = format!("{}/sessions", self.base_url);

        let mut request = self.client.get(&url);

        if let Some(token) = &self.token {
            request = request.header("Authorization", format!("Bearer {}", token));
        }

        let response = request.send().await?;

        if response.status() == 401 {
            return Err("Unauthorized: please login first".into());
        }

        let api_response: ApiResponse<Vec<Session>> = response.json().await?;

        Ok(api_response.data)
    }

    /// Fetch all katas
    pub async fn fetch_katas(&self) -> Result<Vec<Kata>, Box<dyn std::error::Error>> {
        let url = format!("{}/katas", self.base_url);

        let response = self.client.get(&url).send().await?;

        if response.status() == 401 {
            return Err("Unauthorized: please login first".into());
        }

        let api_response: ApiResponse<Vec<Kata>> = response.json().await?;

        Ok(api_response.data)
    }

    /// Create a new session
    pub async fn create_session(
        &self,
        session: SessionInput,
    ) -> Result<Session, Box<dyn std::error::Error>> {
        let url = format!("{}/sessions", self.base_url);

        let mut body = HashMap::new();
        body.insert("session", session);

        let mut request = self.client.post(&url).json(&body);

        if let Some(token) = &self.token {
            request = request.header("Authorization", format!("Bearer {}", token));
        }

        let response = request.send().await?;

        if response.status() == 401 {
            return Err("Unauthorized: please login first".into());
        }

        if response.status() != 201 {
            let error_text = response.text().await?;
            return Err(format!("Failed to create session: {}", error_text).into());
        }

        let api_response: ApiResponse<Session> = response.json().await?;

        Ok(api_response.data)
    }

    /// Initiate device flow authentication
    pub async fn initiate_device_flow(
        &self,
    ) -> Result<DeviceCodeResponse, Box<dyn std::error::Error>> {
        let url = format!("{}/auth/device/code", self.base_url);

        let response = self.client.post(&url).send().await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Err(format!("Failed to initiate device flow: {}", error_text).into());
        }

        let api_response: ApiResponse<DeviceCodeResponse> = response.json().await?;

        Ok(api_response.data)
    }

    /// Poll for device authorization completion
    pub async fn poll_for_token(
        &self,
        device_code: &str,
    ) -> Result<TokenResponse, Box<dyn std::error::Error>> {
        let url = format!("{}/auth/device/token", self.base_url);

        let mut body = HashMap::new();
        body.insert("device_code", device_code);

        let response = self.client.post(&url).json(&body).send().await?;

        let status = response.status();
        let response_text = response.text().await?;

        // Try to parse as error response first
        if let Ok(error_response) = serde_json::from_str::<ErrorResponse>(&response_text) {
            if error_response.error == "authorization_pending" {
                return Err("authorization_pending".into());
            }
            return Err(format!(
                "Authorization failed: {} - {}",
                error_response.error,
                error_response.error_description.unwrap_or_default()
            )
            .into());
        }

        // Try to parse as successful token response
        if !status.is_success() {
            return Err(format!("Unexpected status: {} - {}", status, response_text).into());
        }

        let api_response: ApiResponse<TokenResponse> = serde_json::from_str(&response_text)?;

        Ok(api_response.data)
    }
}