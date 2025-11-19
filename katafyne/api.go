package main

import (
	"bytes"
	"encoding/json"
	"fmt"
	"io"
	"net/http"
)

// Data is a generic wrapper for API responses
type Data[T Session | Kata] struct {
	Data []*T `json:"data"`
}

// createAuthRequest creates an HTTP request with authentication headers
func createAuthRequest(method, url, token string, body io.Reader) (*http.Request, error) {
	req, err := http.NewRequest(method, url, body)
	if err != nil {
		return nil, err
	}

	if token != "" {
		req.Header.Set("Authorization", "Bearer "+token)
	}
	if body != nil {
		req.Header.Set("Content-Type", "application/json")
	}

	return req, nil
}

// FetchSessions returns all training sessions
func FetchSessions(config *Config) ([]*Session, error) {
	req, err := createAuthRequest("GET", config.BaseURL+"/sessions", config.APIToken, nil)
	if err != nil {
		return nil, err
	}

	client := &http.Client{}
	resp, err := client.Do(req)
	if err != nil {
		return nil, err
	}
	defer resp.Body.Close()

	if resp.StatusCode == http.StatusUnauthorized {
		return nil, fmt.Errorf("unauthorized: please login first")
	}

	data, err := io.ReadAll(resp.Body)
	if err != nil {
		return nil, err
	}

	var sessions Data[Session]
	err = json.Unmarshal(data, &sessions)
	if err != nil {
		return nil, err
	}
	return sessions.Data, nil
}

// CreateSession creates a new training session
func CreateSession(config *Config, session *SessionInput) error {
	jsonData, err := json.Marshal(map[string]interface{}{"session": session})
	if err != nil {
		return fmt.Errorf("failed to marshal session: %w", err)
	}

	req, err := createAuthRequest("POST", config.BaseURL+"/sessions", config.APIToken, bytes.NewBuffer(jsonData))
	if err != nil {
		return fmt.Errorf("failed to create request: %w", err)
	}

	client := &http.Client{}
	resp, err := client.Do(req)
	if err != nil {
		return fmt.Errorf("failed to post session: %w", err)
	}
	defer resp.Body.Close()

	if resp.StatusCode == http.StatusUnauthorized {
		return fmt.Errorf("unauthorized: please login first")
	}

	if resp.StatusCode != http.StatusCreated {
		body, _ := io.ReadAll(resp.Body)
		return fmt.Errorf("unexpected status code: %d, body: %s", resp.StatusCode, string(body))
	}

	return nil
}

// FetchKatas returns all available katas
func FetchKatas(config *Config) ([]*Kata, error) {
	req, err := createAuthRequest("GET", config.BaseURL+"/katas", config.APIToken, nil)
	if err != nil {
		return nil, fmt.Errorf("failed to create request: %w", err)
	}

	client := &http.Client{}
	resp, err := client.Do(req)
	if err != nil {
		return nil, fmt.Errorf("failed to get katas: %w", err)
	}
	defer resp.Body.Close()

	if resp.StatusCode == http.StatusUnauthorized {
		return nil, fmt.Errorf("unauthorized: please login first")
	}

	data, err := io.ReadAll(resp.Body)
	if err != nil {
		return nil, fmt.Errorf("failed to read response body: %w", err)
	}

	var katas Data[Kata]
	err = json.Unmarshal(data, &katas)
	if err != nil {
		return nil, fmt.Errorf("failed to unmarshal katas: %w", err)
	}
	return katas.Data, nil
}
