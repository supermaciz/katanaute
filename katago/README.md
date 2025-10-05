# Katago

A terminal user interface (TUI) client for managing kata training sessions with the Katanaute API.

## Overview

Katago is a Go-based CLI application that provides an interactive terminal interface for viewing and managing coding kata practice sessions. It connects to the Katanaute backend API to fetch and display training sessions in a user-friendly list view.

## Features

- **Interactive TUI**: Browse kata training sessions using an elegant terminal interface built with Bubble Tea
- **Session Management**: View session details including kata name, practice date, and notes
- **API Integration**: Seamlessly connects to the Katanaute backend API

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

The TUI will display a list of your kata training sessions. Use:
- Arrow keys or `j`/`k` to navigate
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

