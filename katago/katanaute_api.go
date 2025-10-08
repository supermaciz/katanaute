package main

import (
	"bytes"
	"encoding/json"
	"fmt"
	"io"
	"net/http"
)

type Data[T Session | Kata] struct {
	Data []*T `json:"data"`
}

// FetchSessions returns all training sessions
func FetchSessions(config *Config) ([]*Session, error) {
	resp, err := http.Get(config.katanauteBaseURL + "/sessions")
	if err != nil {
		return nil, err
	}
	defer resp.Body.Close()
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

	resp, err := http.Post(config.katanauteBaseURL+"/sessions", "application/json", bytes.NewBuffer(jsonData))
	if err != nil {
		return fmt.Errorf("failed to post session: %w", err)
	}
	defer resp.Body.Close()

	if resp.StatusCode != http.StatusCreated {
		body, _ := io.ReadAll(resp.Body)
		return fmt.Errorf("unexpected status code: %d, body: %s", resp.StatusCode, string(body))
	}

	return nil
}

func FetchKatas(config *Config) ([]*Kata, error) {
	var katas Data[Kata]

	resp, err := http.Get(config.katanauteBaseURL + "/katas")
	if err != nil {
		return nil, fmt.Errorf("failed to get katas: %w", err)
	}
	defer resp.Body.Close()
	data, err := io.ReadAll(resp.Body)
	if err != nil {
		return nil, fmt.Errorf("failed to read response body: %w", err)
	}
	err = json.Unmarshal(data, &katas)
	if err != nil {
		fmt.Println("Debug: ", string(data))
		return nil, fmt.Errorf("failed to unmarshal katas: %w", err)
	}
	return katas.Data, nil
}
