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

        #[derive(serde::Serialize)]
        struct CreateSessionRequest {
            session: SessionInput,
        }

        let body = CreateSessionRequest { session };

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

    /// Delete a session by id
    pub async fn delete_session(&self, session_id: i32) -> Result<(), Box<dyn std::error::Error>> {
        let url = format!("{}/sessions/{}", self.base_url, session_id);

        let mut request = self.client.delete(&url);

        if let Some(token) = &self.token {
            request = request.header("Authorization", format!("Bearer {}", token));
        }

        let response = request.send().await?;

        if response.status() == 401 {
            return Err("Unauthorized: please login first".into());
        }

        if response.status() != 204 {
            let error_text = response.text().await?;
            return Err(format!("Failed to delete session: {}", error_text).into());
        }

        Ok(())
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

#[cfg(test)]
mod tests {
    use super::*;

    use hyper::service::{make_service_fn, service_fn};
    use hyper::{Body, Method, Request, Response, Server};
    use tokio::net::TcpListener;
    use tokio::sync::oneshot;

    use std::convert::Infallible;
    use std::sync::{Arc, Mutex};

    async fn start_mock_server(
        responder: impl Fn(Request<Body>) -> Response<Body> + Send + Sync + 'static,
    ) -> Result<(String, oneshot::Sender<()>), Box<dyn std::error::Error>> {
        let listener = TcpListener::bind("127.0.0.1:0").await?;
        let addr = listener.local_addr()?;
        let (shutdown_tx, shutdown_rx) = oneshot::channel();
        let responder = Arc::new(responder);
        let responder_clone = responder.clone();

        let server = Server::from_tcp(listener.into_std()?)?
            .serve(make_service_fn(move |_conn| {
                let responder = responder_clone.clone();
                async move {
                    Ok::<_, Infallible>(service_fn(move |req: Request<Body>| {
                        let responder = responder.clone();
                        async move { Ok::<_, Infallible>(responder(req)) }
                    }))
                }
            }))
            .with_graceful_shutdown(async {
                let _ = shutdown_rx.await;
            });

        tokio::spawn(server);
        Ok((format!("http://{}", addr), shutdown_tx))
    }

    #[test]
    fn set_token_sets_token() {
        let mut client = ApiClient::new("http://example".into(), None);
        client.set_token("abc123".into());
        assert_eq!(client.token, Some("abc123".into()));
    }

    #[test]
    fn clear_token_clears_token() {
        let mut client = ApiClient::new("http://example".into(), Some("initial".into()));
        client.clear_token();
        assert!(client.token.is_none());
    }

    #[tokio::test]
    async fn delete_session_sends_authorization_and_handles_no_content()
    -> Result<(), Box<dyn std::error::Error>> {
        let captured_auth = Arc::new(Mutex::new(None));
        let captured_auth_clone = captured_auth.clone();

        let (base, shutdown) = start_mock_server(move |req| {
            if req.method() == Method::DELETE && req.uri().path() == "/api/sessions/42" {
                let auth_header = req
                    .headers()
                    .get("authorization")
                    .and_then(|h| h.to_str().ok())
                    .map(|h| h.to_string());
                *captured_auth_clone.lock().unwrap() = auth_header;
                return Response::builder().status(204).body(Body::empty()).unwrap();
            }

            Response::builder().status(404).body(Body::empty()).unwrap()
        })
        .await?;

        let client = ApiClient::new(format!("{}/api", base), Some("token123".into()));
        let result = client.delete_session(42).await;

        shutdown.send(()).ok();

        assert!(result.is_ok());
        assert_eq!(
            captured_auth.lock().unwrap().as_deref(),
            Some("Bearer token123")
        );

        Ok(())
    }

    #[tokio::test]
    async fn delete_session_returns_error_on_failure() -> Result<(), Box<dyn std::error::Error>> {
        let (base, shutdown) = start_mock_server(|req| {
            if req.method() == Method::DELETE && req.uri().path() == "/api/sessions/99" {
                return Response::builder()
                    .status(500)
                    .body(Body::from("boom"))
                    .unwrap();
            }

            Response::builder().status(404).body(Body::empty()).unwrap()
        })
        .await?;

        let client = ApiClient::new(format!("{}/api", base), Some("token123".into()));
        let result = client.delete_session(99).await;

        shutdown.send(()).ok();

        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("Failed to delete session: boom")
        );

        Ok(())
    }
}
