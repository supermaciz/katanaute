package main

import (
	"bytes"
	"encoding/json"
	"fmt"
	"io"
	"net/http"
	"time"
)

// DeviceCodeResponse represents the response from device code initiation
type DeviceCodeResponse struct {
	DeviceCode      string `json:"device_code"`
	UserCode        string `json:"user_code"`
	VerificationURI string `json:"verification_uri"`
	ExpiresIn       int    `json:"expires_in"`
	Interval        int    `json:"interval"`
}

// DeviceTokenResponse represents the response when polling for token
type DeviceTokenResponse struct {
	AccessToken string `json:"access_token"`
	TokenType   string `json:"token_type"`
	User        User   `json:"user"`
}

// User represents a user in the system
type User struct {
	ID          int     `json:"id"`
	Email       string  `json:"email"`
	ConfirmedAt *string `json:"confirmed_at"`
}

// InitiateDeviceFlow starts the device authorization flow
func InitiateDeviceFlow(baseURL string) (*DeviceCodeResponse, error) {
	resp, err := http.Post(baseURL+"/auth/device/code", "application/json", nil)
	if err != nil {
		return nil, fmt.Errorf("failed to initiate device flow: %w", err)
	}
	defer resp.Body.Close()

	if resp.StatusCode != http.StatusOK {
		body, _ := io.ReadAll(resp.Body)
		return nil, fmt.Errorf("unexpected status code: %d, body: %s", resp.StatusCode, string(body))
	}

	data, err := io.ReadAll(resp.Body)
	if err != nil {
		return nil, fmt.Errorf("failed to read response: %w", err)
	}

	var response struct {
		Data DeviceCodeResponse `json:"data"`
	}
	if err := json.Unmarshal(data, &response); err != nil {
		return nil, fmt.Errorf("failed to parse response: %w", err)
	}

	return &response.Data, nil
}

// PollForToken polls the server for device authorization completion
func PollForToken(baseURL, deviceCode string) (*DeviceTokenResponse, error) {
	requestBody := map[string]string{
		"device_code": deviceCode,
	}
	jsonData, err := json.Marshal(requestBody)
	if err != nil {
		return nil, fmt.Errorf("failed to marshal request: %w", err)
	}

	resp, err := http.Post(baseURL+"/auth/device/token", "application/json", bytes.NewBuffer(jsonData))
	if err != nil {
		return nil, fmt.Errorf("failed to poll for token: %w", err)
	}
	defer resp.Body.Close()

	data, err := io.ReadAll(resp.Body)
	if err != nil {
		return nil, fmt.Errorf("failed to read response: %w", err)
	}

	if resp.StatusCode != http.StatusOK && resp.StatusCode != http.StatusBadRequest {
		return nil, fmt.Errorf("unexpected status code: %d, body: %s", resp.StatusCode, string(data))
	}

	// Check if response contains an error field (pending, denied, etc.)
	// Backend returns HTTP 200 with error field when authorization is pending
	var errorCheck struct {
		Error            string `json:"error"`
		ErrorDescription string `json:"error_description"`
	}
	if err := json.Unmarshal(data, &errorCheck); err == nil && errorCheck.Error != "" {
		if errorCheck.Error == "authorization_pending" {
			return nil, fmt.Errorf("authorization_pending")
		}
		return nil, fmt.Errorf("authorization failed: %s - %s", errorCheck.Error, errorCheck.ErrorDescription)
	}

	// Parse successful response
	var response struct {
		Data DeviceTokenResponse `json:"data"`
	}
	if err := json.Unmarshal(data, &response); err != nil {
		return nil, fmt.Errorf("failed to parse response: %w", err)
	}

	return &response.Data, nil
}

// AuthenticateWithDeviceFlow performs the complete device flow authentication
func AuthenticateWithDeviceFlow(baseURL string, onCodeReceived func(userCode, verificationURI string)) (string, error) {
	// Initiate device flow
	deviceCodeResp, err := InitiateDeviceFlow(baseURL)
	if err != nil {
		return "", err
	}

	// Notify caller with the user code and verification URI
	onCodeReceived(deviceCodeResp.UserCode, deviceCodeResp.VerificationURI)

	// Poll for authorization
	interval := time.Duration(deviceCodeResp.Interval) * time.Second
	if interval == 0 {
		interval = 5 * time.Second
	}

	expiresAt := time.Now().Add(time.Duration(deviceCodeResp.ExpiresIn) * time.Second)

	for time.Now().Before(expiresAt) {
		time.Sleep(interval)

		tokenResp, err := PollForToken(baseURL, deviceCodeResp.DeviceCode)
		if err != nil {
			if err.Error() == "authorization_pending" {
				continue
			}
			return "", err
		}

		// Success! Return the token
		return tokenResp.AccessToken, nil
	}

	return "", fmt.Errorf("device authorization timed out")
}
