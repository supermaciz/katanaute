package main

import (
	"fmt"
	"os"

	"github.com/charmbracelet/bubbles/list"
	tea "github.com/charmbracelet/bubbletea"
)

type Config struct {
	katanauteBaseURL string
}

func NewConfig() *Config {
	return &Config{
		katanauteBaseURL: "http://localhost:4000/api",
	}
}

func main() {
	config := NewConfig()
	sessions, err := GetSessions(config)
	if err != nil {
		fmt.Println("GET /sessions error", err)
		os.Exit(1)
	}
	//fmt.Println(sessions)
	items := make([]list.Item, len(sessions))
	for i, session := range sessions {
		items[i] = session
	}
	m := model{list: list.New(items, list.NewDefaultDelegate(), 0, 0)}
	m.list.Title = "Kata training sessions"
	p := tea.NewProgram(m, tea.WithAltScreen())
	if _, err := p.Run(); err != nil {
		fmt.Println("Oh no! Something went wrong:", err)
		os.Exit(1)
	}
}
