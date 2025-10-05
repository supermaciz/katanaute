package main

import (
	"encoding/json"
	"fmt"
	"io"
	"net/http"
	"time"
)

type Kata struct {
	ID    int    `json:"id"`
	Name  string `json:"name"`
	Level string `json:"level"`
}

type Session struct {
	ID          int       `json:"id"`
	InCourse    bool      `json:"in_course"`
	Notes       string    `json:"notes"`
	PracticedAt time.Time `json:"practiced_at"`
	Kata        *Kata     `json:"kata"`
}

func (s Session) FilterValue() string {
	return s.Kata.Name
}

func (s Session) Title() string {
	return s.Kata.Name
}

func (s Session) Description() string {
	return fmt.Sprintf("(%s)", s.PracticedAt.Format("2006-01-02"))
}

func (s Session) String() string {
	return fmt.Sprintf("%s (%s): %s", s.Kata.Name, s.PracticedAt, s.Notes)
}

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
