use anyhow::{anyhow, Result};
use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use std::time::Duration;

use crate::models::*;

pub struct ApiClient {
    base_url: String,
    client: Client,
    token: Option<String>,
}

impl ApiClient {
    pub fn new(base_url: Option<String>) -> Self {
        let base_url = base_url.unwrap_or_else(|| {
            std::env::var("KATANAUTE_API_URL")
                .unwrap_or_else(|_| "http://localhost:4000/api".to_string())
        });

        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .expect("Failed to create HTTP client");

        ApiClient {
            base_url,
            client,
            token: None,
        }
    }

    pub fn set_token(&mut self, token: String) {
        self.token = Some(token);
    }

    pub fn clear_token(&mut self) {
        self.token = None;
    }

    fn headers(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

        if let Some(ref token) = self.token {
            headers.insert(
                AUTHORIZATION,
                HeaderValue::from_str(&format!("Bearer {}", token))
                    .expect("Invalid token format"),
            );
        }

        headers
    }

    // Authentication endpoints
    pub fn register(&self, email: &str, password: &str) -> Result<AuthResponse> {
        let url = format!("{}/auth/register", self.base_url);
        let req = RegisterRequest {
            email: email.to_string(),
            password: password.to_string(),
        };

        let response = self
            .client
            .post(&url)
            .headers(self.headers())
            .json(&req)
            .send()?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().unwrap_or_default();
            return Err(anyhow!("Registration failed: {} - {}", status, text));
        }

        let api_response: ApiResponse<AuthResponse> = response.json()?;
        Ok(api_response.data)
    }

    pub fn login(&self, email: &str, password: &str) -> Result<AuthResponse> {
        let url = format!("{}/auth/token", self.base_url);
        let req = LoginRequest {
            email: email.to_string(),
            password: password.to_string(),
        };

        let response = self
            .client
            .post(&url)
            .headers(self.headers())
            .json(&req)
            .send()?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().unwrap_or_default();
            return Err(anyhow!("Login failed: {} - {}", status, text));
        }

        let api_response: ApiResponse<AuthResponse> = response.json()?;
        Ok(api_response.data)
    }

    pub fn logout(&self) -> Result<()> {
        let url = format!("{}/auth/token", self.base_url);
        let response = self.client.delete(&url).headers(self.headers()).send()?;

        if !response.status().is_success() {
            return Err(anyhow!("Logout failed: {}", response.status()));
        }

        Ok(())
    }

    pub fn get_current_user(&self) -> Result<User> {
        let url = format!("{}/auth/me", self.base_url);
        let response = self.client.get(&url).headers(self.headers()).send()?;

        if !response.status().is_success() {
            return Err(anyhow!("Failed to get current user: {}", response.status()));
        }

        let api_response: ApiResponse<User> = response.json()?;
        Ok(api_response.data)
    }

    // Device flow endpoints
    pub fn initiate_device_flow(&self) -> Result<DeviceCodeResponse> {
        let url = format!("{}/auth/device/code", self.base_url);
        let response = self.client.post(&url).headers(self.headers()).send()?;

        if !response.status().is_success() {
            return Err(anyhow!(
                "Failed to initiate device flow: {}",
                response.status()
            ));
        }

        Ok(response.json()?)
    }

    pub fn poll_device_token(&self, device_code: &str) -> Result<Option<AuthResponse>> {
        let url = format!("{}/auth/device/token", self.base_url);
        let req = DeviceTokenRequest {
            device_code: device_code.to_string(),
        };

        let response = self
            .client
            .post(&url)
            .headers(self.headers())
            .json(&req)
            .send()?;

        if response.status().is_success() {
            let api_response: ApiResponse<AuthResponse> = response.json()?;
            return Ok(Some(api_response.data));
        }

        // Check for pending/denied status
        if let Ok(error) = response.json::<DeviceTokenError>() {
            if error.error == "authorization_pending" {
                return Ok(None);
            }
            return Err(anyhow!("Device authorization error: {}", error.error));
        }

        Err(anyhow!("Unexpected response from device token endpoint"))
    }

    // Session endpoints
    pub fn get_sessions(&self) -> Result<Vec<Session>> {
        let url = format!("{}/sessions", self.base_url);
        let response = self.client.get(&url).headers(self.headers()).send()?;

        if !response.status().is_success() {
            return Err(anyhow!("Failed to fetch sessions: {}", response.status()));
        }

        let api_response: ApiResponse<Vec<Session>> = response.json()?;
        Ok(api_response.data)
    }

    pub fn create_session(&self, request: CreateSessionRequest) -> Result<Session> {
        let url = format!("{}/sessions", self.base_url);
        let response = self
            .client
            .post(&url)
            .headers(self.headers())
            .json(&request)
            .send()?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().unwrap_or_default();
            return Err(anyhow!("Failed to create session: {} - {}", status, text));
        }

        let api_response: ApiResponse<Session> = response.json()?;
        Ok(api_response.data)
    }

    // Kata endpoints (public)
    pub fn get_katas(&self) -> Result<Vec<Kata>> {
        let url = format!("{}/katas", self.base_url);
        let response = self.client.get(&url).headers(self.headers()).send()?;

        if !response.status().is_success() {
            return Err(anyhow!("Failed to fetch katas: {}", response.status()));
        }

        let api_response: ApiResponse<Vec<Kata>> = response.json()?;
        Ok(api_response.data)
    }
}
