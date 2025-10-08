package main

import (
	"fmt"
	"time"
)

type Kata struct {
	ID    int    `json:"id"`
	Name  string `json:"name"`
	Level string `json:"level"`
}

func (k Kata) String() string {
	return fmt.Sprintf("%s (%s)", k.Name, k.Level)
}

type Session struct {
	ID          int       `json:"id,omitempty"`
	InCourse    bool      `json:"in_course"`
	Notes       string    `json:"notes,omitempty"`
	PracticedAt time.Time `json:"practiced_at"`
	Kata        *Kata     `json:"kata,omitempty"`
}

type SessionPost struct {
	Session
	KataID         int    `json:"kata_id"`
	TmpPracticedAt string `json:"-"`
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
