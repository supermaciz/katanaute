package main

import (
	"fmt"

	"github.com/supermaciz/katanaute/katagocore"
)

// SessionListItem wraps katagocore.Session to implement list.Item interface
type SessionListItem struct {
	*katagocore.Session
}

func (s SessionListItem) FilterValue() string {
	return s.Kata.Name
}

func (s SessionListItem) Title() string {
	return s.Kata.Name
}

func (s SessionListItem) Description() string {
	return fmt.Sprintf("(%s)", s.PracticedAt.Format("2006-01-02"))
}
