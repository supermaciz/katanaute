package main

import (
	"encoding/json"
	"io"
	"net/http"
	"strings"
	"testing"
	"time"
)

type roundTripFunc func(*http.Request) (*http.Response, error)

func (f roundTripFunc) RoundTrip(req *http.Request) (*http.Response, error) {
	return f(req)
}

func stubHTTP(t *testing.T, fn roundTripFunc) {
	t.Helper()
	original := http.DefaultTransport
	http.DefaultTransport = fn
	t.Cleanup(func() {
		http.DefaultTransport = original
	})
}

func newTestConfig() *Config {
	return &Config{katanauteBaseURL: "http://katago.test"}
}

func TestFetchSessionsSuccess(t *testing.T) {
	practicedAt := time.Date(2025, 1, 1, 15, 0, 0, 0, time.UTC)
	stubHTTP(t, func(r *http.Request) (*http.Response, error) {
		if r.Method != http.MethodGet {
			t.Fatalf("expected GET request, got %s", r.Method)
		}
		if r.URL.Path != "/sessions" {
			t.Fatalf("expected path /sessions, got %s", r.URL.Path)
		}
		response := map[string]any{
			"data": []map[string]any{
				{
					"id":           1,
					"in_course":    true,
					"notes":        "Focus on breathing",
					"practiced_at": practicedAt.Format(time.RFC3339),
					"kata": map[string]any{
						"id":    5,
						"name":  "Sanchin",
						"level": "yellow",
					},
				},
			},
		}
		data, err := json.Marshal(response)
		if err != nil {
			t.Fatalf("failed to marshal response: %v", err)
		}
		return &http.Response{
			StatusCode: http.StatusOK,
			Body:       io.NopCloser(strings.NewReader(string(data))),
			Header:     make(http.Header),
		}, nil
	})

	config := newTestConfig()

	sessions, err := FetchSessions(config)
	if err != nil {
		t.Fatalf("FetchSessions returned error: %v", err)
	}
	if len(sessions) != 1 {
		t.Fatalf("expected 1 session, got %d", len(sessions))
	}
	session := sessions[0]
	if session.ID != 1 {
		t.Errorf("expected ID 1, got %d", session.ID)
	}
	if session.Kata == nil || session.Kata.Name != "Sanchin" {
		t.Fatalf("expected kata Sanchin, got %#v", session.Kata)
	}
	if !session.PracticedAt.Equal(practicedAt) {
		t.Errorf("expected practiced_at %s, got %s", practicedAt, session.PracticedAt)
	}
}

func TestFetchSessionsUnmarshalError(t *testing.T) {
	stubHTTP(t, func(r *http.Request) (*http.Response, error) {
		if r.URL.Path != "/sessions" {
			t.Fatalf("expected /sessions path, got %s", r.URL.Path)
		}
		return &http.Response{
			StatusCode: http.StatusOK,
			Body:       io.NopCloser(strings.NewReader(`{"data":[`)),
			Header:     make(http.Header),
		}, nil
	})

	_, err := FetchSessions(newTestConfig())
	if err == nil {
		t.Fatalf("expected error due to invalid JSON but got nil")
	}
}

func TestFetchKatasSuccess(t *testing.T) {
	stubHTTP(t, func(r *http.Request) (*http.Response, error) {
		if r.Method != http.MethodGet {
			t.Fatalf("expected GET, got %s", r.Method)
		}
		if r.URL.Path != "/katas" {
			t.Fatalf("expected path /katas, got %s", r.URL.Path)
		}
		response := map[string]any{
			"data": []map[string]any{
				{"id": 1, "name": "Sanchin", "level": "yellow"},
				{"id": 2, "name": "Seisan", "level": "orange"},
			},
		}
		data, err := json.Marshal(response)
		if err != nil {
			t.Fatalf("failed to marshal response: %v", err)
		}
		return &http.Response{
			StatusCode: http.StatusOK,
			Body:       io.NopCloser(strings.NewReader(string(data))),
			Header:     make(http.Header),
		}, nil
	})

	katas, err := FetchKatas(newTestConfig())
	if err != nil {
		t.Fatalf("FetchKatas returned error: %v", err)
	}
	if len(katas) != 2 {
		t.Fatalf("expected 2 katas, got %d", len(katas))
	}
	if katas[1].Name != "Seisan" {
		t.Errorf("expected second kata Seisan, got %s", katas[1].Name)
	}
}

func TestFetchKatasUnmarshalError(t *testing.T) {
	stubHTTP(t, func(r *http.Request) (*http.Response, error) {
		return &http.Response{
			StatusCode: http.StatusOK,
			Body:       io.NopCloser(strings.NewReader("not valid json")),
			Header:     make(http.Header),
		}, nil
	})

	if _, err := FetchKatas(newTestConfig()); err == nil {
		t.Fatalf("expected unmarshal error but got nil")
	}
}

func TestCreateSessionSuccess(t *testing.T) {
	var receivedBody string
	stubHTTP(t, func(r *http.Request) (*http.Response, error) {
		if r.Method != http.MethodPost {
			t.Fatalf("expected POST, got %s", r.Method)
		}
		if r.URL.Path != "/sessions" {
			t.Fatalf("expected path /sessions, got %s", r.URL.Path)
		}
		body, err := io.ReadAll(r.Body)
		if err != nil {
			t.Fatalf("failed to read body: %v", err)
		}
		receivedBody = string(body)
		return &http.Response{
			StatusCode: http.StatusCreated,
			Body:       io.NopCloser(strings.NewReader("")),
			Header:     make(http.Header),
		}, nil
	})

	session := &SessionInput{}
	if err := CreateSession(newTestConfig(), session); err != nil {
		t.Fatalf("CreateSession returned error: %v", err)
	}
	if !strings.Contains(receivedBody, `"session"`) {
		t.Fatalf("expected request body to contain session wrapper, got %s", receivedBody)
	}
}

func TestCreateSessionUnexpectedStatus(t *testing.T) {
	stubHTTP(t, func(r *http.Request) (*http.Response, error) {
		return &http.Response{
			StatusCode: http.StatusBadRequest,
			Body:       io.NopCloser(strings.NewReader(`{"error":"invalid"}`)),
			Header:     make(http.Header),
		}, nil
	})
	err := CreateSession(newTestConfig(), &SessionInput{})
	if err == nil {
		t.Fatalf("expected error for non-201 response")
	}
	if !strings.Contains(err.Error(), "unexpected status code") {
		t.Fatalf("expected error to mention unexpected status code, got %v", err)
	}
}
