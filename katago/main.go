package main

import (
	"fmt"
	"log"
	"os"

	tea "github.com/charmbracelet/bubbletea"
	"github.com/supermaciz/katanaute/katagocore"
)

func main() {
	if len(os.Getenv("DEBUG")) > 0 {
		f, err := tea.LogToFile("debug.log", "debug")
		if err != nil {
			fmt.Println("fatal:", err)
			os.Exit(1)
		}
		defer f.Close()
	}

	config, err := katagocore.NewConfig()
	if err != nil {
		fmt.Println("Error loading config:", err)
		os.Exit(1)
	}

	log.Println("Using Katanaute API URL:", config.BaseURL)

	// Check if user needs to authenticate
	if config.APIToken == "" {
		fmt.Println("You are not authenticated. Starting login flow...")
		token, err := katagocore.AuthenticateWithDeviceFlow(config.BaseURL, func(userCode, verificationURI string) {
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
		configFile := &katagocore.ConfigFile{
			APIToken: token,
			BaseURL:  config.BaseURL,
		}
		if err := katagocore.SaveConfig(configFile); err != nil {
			fmt.Println("Failed to save config:", err)
			os.Exit(1)
		}

		config.APIToken = token
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
