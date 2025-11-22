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