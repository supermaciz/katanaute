package main

import (
	"fmt"
	"log"
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
	if len(os.Getenv("DEBUG")) > 0 {
		f, err := tea.LogToFile("debug.log", "debug")
		if err != nil {
			fmt.Println("fatal:", err)
			os.Exit(1)
			defer f.Close()
		}
	}
	if katanauteBaseURL, ok := os.LookupEnv("KATANAUTE_API_URL"); ok {
		config.katanauteBaseURL = katanauteBaseURL
	}
	log.Println("Using Katanaute API URL: ", config.katanauteBaseURL)
	m := NewModel(config)
	p := tea.NewProgram(m, tea.WithAltScreen())
	if _, err := p.Run(); err != nil {
		fmt.Println("Oh no! Something went wrong:", err)
		os.Exit(1)
	}
}
