use crate::api::ApiClient;
use std::time::Duration;
use tokio::time::sleep;

/// Result of device flow authentication
#[derive(Debug, Clone)]
pub struct DeviceFlowInfo {
    pub device_code: String,
    pub user_code: String,
    pub verification_uri: String,
    pub interval: u64,
}

/// Initiate device flow and return information for user
pub async fn initiate_device_flow(
    api_client: &ApiClient,
) -> Result<DeviceFlowInfo, Box<dyn std::error::Error>> {
    let response = api_client.initiate_device_flow().await?;

    Ok(DeviceFlowInfo {
        device_code: response.device_code,
        user_code: response.user_code,
        verification_uri: response.verification_uri,
        interval: response.interval as u64,
    })
}

/// Poll for device authorization completion
pub async fn poll_for_authorization(
    api_client: &ApiClient,
    device_code: String,
    interval: u64,
) -> Result<String, Box<dyn std::error::Error>> {
    let poll_interval = Duration::from_secs(if interval > 0 { interval } else { 5 });

    loop {
        sleep(poll_interval).await;

        match api_client.poll_for_token(&device_code).await {
            Ok(token_response) => {
                return Ok(token_response.access_token);
            }
            Err(e) => {
                let err_msg = e.to_string();
                if err_msg.contains("authorization_pending") {
                    // Continue polling
                    continue;
                } else {
                    return Err(e);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::ApiClient;
    use hyper::service::{make_service_fn, service_fn};
    use hyper::{Body, Method, Request, Response, Server};
    use serde_json::json;
    use std::convert::Infallible;
    use std::sync::Arc;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use tokio::net::TcpListener;
    use tokio::sync::oneshot;

    async fn start_device_token_server(
        responder: impl Fn(usize) -> Response<Body> + Send + Sync + 'static,
    ) -> Result<(String, oneshot::Sender<()>), Box<dyn std::error::Error>> {
        let listener = TcpListener::bind("127.0.0.1:0").await?;
        let addr = listener.local_addr()?;
        let (shutdown_tx, shutdown_rx) = oneshot::channel();
        let counter = Arc::new(AtomicUsize::new(0));
        let counter_clone = counter.clone();
        let responder = Arc::new(responder);
        let responder_clone = responder.clone();

        let server = Server::from_tcp(listener.into_std()?)?
            .serve(make_service_fn(move |_conn| {
                let counter = counter_clone.clone();
                let responder = responder_clone.clone();
                async move {
                    Ok::<_, Infallible>(service_fn(move |req: Request<Body>| {
                        let counter = counter.clone();
                        let responder = responder.clone();
                        async move {
                            if req.method() == Method::POST
                                && req.uri().path() == "/api/auth/device/token"
                            {
                                let idx = counter.fetch_add(1, Ordering::SeqCst);
                                return Ok::<_, Infallible>(responder(idx));
                            }
                            Ok::<_, Infallible>(
                                Response::builder().status(404).body(Body::empty()).unwrap(),
                            )
                        }
                    }))
                }
            }))
            .with_graceful_shutdown(async {
                let _ = shutdown_rx.await;
            });

        tokio::spawn(server);
        Ok((format!("http://{}", addr), shutdown_tx))
    }

    #[tokio::test]
    async fn poll_for_authorization_retries_and_returns_token()
    -> Result<(), Box<dyn std::error::Error>> {
        let device_code = "test-device-code".to_string();

        let (base, shutdown) = start_device_token_server(|idx| {
            if idx == 0 {
                let body = json!({"error": "authorization_pending", "error_description": null})
                    .to_string();
                return Response::builder()
                    .status(200)
                    .header("content-type", "application/json")
                    .body(Body::from(body))
                    .unwrap();
            }

            let body = json!({
                "data": {
                    "access_token": "token123",
                    "token_type": "Bearer",
                    "user": {"id": 1, "email": "test@example.com", "confirmed_at": null}
                }
            })
            .to_string();

            Response::builder()
                .status(200)
                .header("content-type", "application/json")
                .body(Body::from(body))
                .unwrap()
        })
        .await?;

        let client = ApiClient::new(format!("{}/api", base), None);
        let result = poll_for_authorization(&client, device_code, 1).await?;

        shutdown.send(()).ok();

        assert_eq!(result, "token123");
        Ok(())
    }

    #[tokio::test]
    async fn poll_for_authorization_fails_on_non_pending_error()
    -> Result<(), Box<dyn std::error::Error>> {
        let device_code = "test-device-code".to_string();

        let (base, shutdown) = start_device_token_server(|_| {
            let body = json!({
                "error": "access_denied",
                "error_description": "User denied the request"
            })
            .to_string();

            Response::builder()
                .status(200)
                .header("content-type", "application/json")
                .body(Body::from(body))
                .unwrap()
        })
        .await?;

        let client = ApiClient::new(format!("{}/api", base), None);
        let result = poll_for_authorization(&client, device_code, 1).await;

        shutdown.send(()).ok();

        assert!(result.is_err());
        let error_msg = result.unwrap_err().to_string();
        assert!(error_msg.contains("access_denied"));

        Ok(())
    }
}
