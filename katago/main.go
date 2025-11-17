package main

import (
	"fmt"
	"log"
	"os"

	tea "github.com/charmbracelet/bubbletea"
)

type Config struct {
	katanauteBaseURL string
	apiToken         string
}

func NewConfig() (*Config, error) {
	// Load persisted config
	configFile, err := LoadConfig()
	if err != nil {
		return nil, fmt.Errorf("failed to load config: %w", err)
	}

	baseURL := "http://localhost:4000/api"

	// Override with environment variable if set
	if envURL, ok := os.LookupEnv("KATANAUTE_API_URL"); ok {
		baseURL = envURL
	} else if configFile.BaseURL != "" {
		baseURL = configFile.BaseURL
	}

	return &Config{
		katanauteBaseURL: baseURL,
		apiToken:         configFile.APIToken,
	}, nil
}

func main() {
	if len(os.Getenv("DEBUG")) > 0 {
		f, err := tea.LogToFile("debug.log", "debug")
		if err != nil {
			fmt.Println("fatal:", err)
			os.Exit(1)
		}
		defer f.Close()
	}

	config, err := NewConfig()
	if err != nil {
		fmt.Println("Error loading config:", err)
		os.Exit(1)
	}

	log.Println("Using Katanaute API URL:", config.katanauteBaseURL)

	// Check if user needs to authenticate
	if config.apiToken == "" {
		fmt.Println("You are not authenticated. Starting login flow...")
		token, err := AuthenticateWithDeviceFlow(config.katanauteBaseURL, func(userCode, verificationURI string) {
			fmt.Println("\nTo authenticate, please:")
			fmt.Printf("1. Visit: %s\n", verificationURI)
			fmt.Printf("2. Enter code: %s\n\n", userCode)
			fmt.Println("Waiting for authorization...")
		})
		if err != nil {
			fmt.Println("Authentication failed:", err)
			os.Exit(1)
		}

		// Save token to config
		configFile := &ConfigFile{
			APIToken: token,
			BaseURL:  config.katanauteBaseURL,
		}
		if err := SaveConfig(configFile); err != nil {
			fmt.Println("Failed to save config:", err)
			os.Exit(1)
		}

		config.apiToken = token
		fmt.Println("Authentication successful!")
		fmt.Println("Starting TUI...")
	}

	m := NewModel(config)
	p := tea.NewProgram(m, tea.WithAltScreen())
	if _, err := p.Run(); err != nil {
		fmt.Println("Oh no! Something went wrong:", err)
		os.Exit(1)
	}
}
