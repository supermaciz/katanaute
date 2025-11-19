package main

import (
	"encoding/json"
	"fmt"
	"os"
	"path/filepath"
)

// ConfigFile represents the persisted configuration
type ConfigFile struct {
	APIToken string `json:"api_token"`
	BaseURL  string `json:"base_url"`
}

// Config holds the runtime configuration
type Config struct {
	BaseURL  string
	APIToken string
}

// GetConfigDir returns the XDG-compliant config directory
func GetConfigDir() (string, error) {
	// Check XDG_CONFIG_HOME first
	configHome := os.Getenv("XDG_CONFIG_HOME")
	if configHome == "" {
		// Fall back to ~/.config
		homeDir, err := os.UserHomeDir()
		if err != nil {
			return "", fmt.Errorf("failed to get home directory: %w", err)
		}
		configHome = filepath.Join(homeDir, ".config")
	}

	configDir := filepath.Join(configHome, "katanaute")

	// Ensure directory exists
	if err := os.MkdirAll(configDir, 0700); err != nil {
		return "", fmt.Errorf("failed to create config directory: %w", err)
	}

	return configDir, nil
}

// GetConfigPath returns the full path to the config file
func GetConfigPath() (string, error) {
	configDir, err := GetConfigDir()
	if err != nil {
		return "", err
	}
	return filepath.Join(configDir, "config.json"), nil
}

// LoadConfig loads the configuration from disk
func LoadConfig() (*ConfigFile, error) {
	configPath, err := GetConfigPath()
	if err != nil {
		return nil, err
	}

	// If config doesn't exist, return empty config
	if _, err := os.Stat(configPath); os.IsNotExist(err) {
		return &ConfigFile{}, nil
	}

	data, err := os.ReadFile(configPath)
	if err != nil {
		return nil, fmt.Errorf("failed to read config file: %w", err)
	}

	var config ConfigFile
	if err := json.Unmarshal(data, &config); err != nil {
		return nil, fmt.Errorf("failed to parse config file: %w", err)
	}

	return &config, nil
}

// SaveConfig saves the configuration to disk
func SaveConfig(config *ConfigFile) error {
	configPath, err := GetConfigPath()
	if err != nil {
		return err
	}

	data, err := json.MarshalIndent(config, "", "  ")
	if err != nil {
		return fmt.Errorf("failed to marshal config: %w", err)
	}

	if err := os.WriteFile(configPath, data, 0600); err != nil {
		return fmt.Errorf("failed to write config file: %w", err)
	}

	return nil
}

// NewConfig creates a new Config instance with defaults
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
		BaseURL:  baseURL,
		APIToken: configFile.APIToken,
	}, nil
}
