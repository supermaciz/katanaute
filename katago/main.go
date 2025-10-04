package main

import (
	"fmt"
	"os"

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
	if katanauteBaseURL, ok := os.LookupEnv("KATANAUTE_API_URL"); ok {
		config.katanauteBaseURL = katanauteBaseURL
	}
	m := NewModel(config)
	m.list.Title = "Kata training sessions"
	p := tea.NewProgram(m, tea.WithAltScreen())
	if _, err := p.Run(); err != nil {
		fmt.Println("Oh no! Something went wrong:", err)
		os.Exit(1)
	}
}
