package katagocore

import (
	"fmt"
	"time"
)

// Kata represents a kata in the curriculum
type Kata struct {
	ID    int    `json:"id"`
	Name  string `json:"name"`
	Level string `json:"level"`
}

func (k Kata) String() string {
	return fmt.Sprintf("%s (%s)", k.Name, k.Level)
}

// Session represents a training session
type Session struct {
	ID          int       `json:"id,omitempty"`
	InCourse    bool      `json:"in_course"`
	Notes       string    `json:"notes,omitempty"`
	PracticedAt time.Time `json:"practiced_at"`
	Kata        *Kata     `json:"kata,omitempty"`
}

// SessionInput is used when creating a new session
type SessionInput struct {
	Session
	KataID int `json:"kata_id"`
}

// User represents a user in the system
type User struct {
	ID          int     `json:"id"`
	Email       string  `json:"email"`
	ConfirmedAt *string `json:"confirmed_at"`
}
