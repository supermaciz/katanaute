package main

import (
    "encoding/json"
    "io"
    "net/http"
    "net/http/httptest"
    "strings"
    "testing"
    "time"
)

// newTestConfig creates a Config using the provided baseURL.
func newTestConfig(baseURL string) *Config {
    return &Config{katanauteBaseURL: baseURL}
}

func TestFetchSessionsSuccess(t *testing.T) {
    practicedAt := time.Date(2025, 1, 1, 15, 0, 0, 0, time.UTC)
    ts := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
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
        w.WriteHeader(http.StatusOK)
        _, _ = w.Write(data)
    }))
    defer ts.Close()

    config := newTestConfig(ts.URL)

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
    ts := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
        if r.URL.Path != "/sessions" {
            t.Fatalf("expected /sessions path, got %s", r.URL.Path)
        }
        w.WriteHeader(http.StatusOK)
        _, _ = w.Write([]byte(`{"data":[`))
    }))
    defer ts.Close()

    _, err := FetchSessions(newTestConfig(ts.URL))
    if err == nil {
        t.Fatalf("expected error due to invalid JSON but got nil")
    }
}

func TestFetchKatasSuccess(t *testing.T) {
    ts := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
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
        w.WriteHeader(http.StatusOK)
        _, _ = w.Write(data)
    }))
    defer ts.Close()

    katas, err := FetchKatas(newTestConfig(ts.URL))
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
    ts := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
        w.WriteHeader(http.StatusOK)
        _, _ = w.Write([]byte("not valid json"))
    }))
    defer ts.Close()

    if _, err := FetchKatas(newTestConfig(ts.URL)); err == nil {
        t.Fatalf("expected unmarshal error but got nil")
    }
}

func TestCreateSessionSuccess(t *testing.T) {
    var receivedBody string
    ts := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
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
        w.WriteHeader(http.StatusCreated)
        _, _ = w.Write([]byte(""))
    }))
    defer ts.Close()

    session := &SessionInput{}
    if err := CreateSession(newTestConfig(ts.URL), session); err != nil {
        t.Fatalf("CreateSession returned error: %v", err)
    }
    if !strings.Contains(receivedBody, `"session"`) {
        t.Fatalf("expected request body to contain session wrapper, got %s", receivedBody)
    }
}

func TestCreateSessionUnexpectedStatus(t *testing.T) {
    ts := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
        w.WriteHeader(http.StatusBadRequest)
        _, _ = w.Write([]byte(`{"error":"invalid"}`))
    }))
    defer ts.Close()
    err := CreateSession(newTestConfig(ts.URL), &SessionInput{})
    if err == nil {
        t.Fatalf("expected error for non-201 response")
    }
    if !strings.Contains(err.Error(), "unexpected status code") {
        t.Fatalf("expected error to mention unexpected status code, got %v", err)
    }
}
