# Katago

A terminal user interface (TUI) client for managing kata training sessions with the Katanaute API.

## Overview

Katago is a Go-based CLI application that provides an interactive terminal interface for viewing and managing coding kata practice sessions. It connects to the Katanaute backend API to fetch and display training sessions in a user-friendly list view.

## Features

- **Device Flow Authentication**: Secure OAuth2-style device flow for headless authentication
- **Interactive TUI**: Browse kata training sessions using an elegant terminal interface built with Bubble Tea
- **Session Management**: View and create sessions with kata details, practice dates, and notes
- **API Integration**: Seamlessly connects to the Katanaute backend API with Bearer token authentication

## Prerequisites

- Go 1.25 or higher
- A running instance of the Katanaute backend (default: `http://localhost:4000`)

## Installation

```bash
go build
```

## Usage

Start the Katanaute backend server, then run:

```bash
./katago
```

On first run, you'll need to authenticate using the device flow:
1. The app will display a user code and verification URL
2. Visit the URL in your browser and log in
3. Enter the user code to authorize the device
4. The app will automatically continue once authorized

After authentication, the TUI will display your kata training sessions. Use:
- Arrow keys or `j`/`k` to navigate
- `a` to add a new session
- `Ctrl+C` to quit

## Configuration

By default, Katago connects to `http://localhost:4000/api`. To modify the base URL, update the `katanauteBaseURL` in the `Config` struct in `main.go`.

## Architecture

- **main.go**: Application entry point and configuration
- **katanaute.go**: API client and data models for Sessions and Katas
- **tui.go**: Bubble Tea TUI implementation

## Dependencies

- [Bubble Tea](https://github.com/charmbracelet/bubbletea) - Terminal UI framework
- [Bubbles](https://github.com/charmbracelet/bubbles) - TUI components
- [Lip Gloss](https://github.com/charmbracelet/lipgloss) - Terminal styling

## TODO
- [ ] Implement session editing and deletion
- [X] Show sessions notes in the TUI
- [ ] Clean views (first spinner)
- [ ] Add unit tests

