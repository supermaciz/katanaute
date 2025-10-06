# Katanaute

A kata training tracker with a Phoenix web backend and a terminal UI client.

## Project Structure

- **katanaute/** - Phoenix web application (Elixir/Phoenix)
- **katago/** - Terminal UI client (Go/Bubble Tea)

## Quick Start

### Backend (Phoenix)

```bash
cd katanaute
mix setup
mix phx.server
```

Visit [localhost:4000](http://localhost:4000)

### CLI Client (Go)

```bash
cd katago
go build
./katago
```

## Features

- Track coding kata practice sessions
- Record session notes with Markdown support
- Browse sessions via web UI or terminal
- RESTful API for external integrations

## Configuration

The TUI client connects to `http://localhost:4000/api` by default. Override with:

```bash
export KATANAUTE_API_URL=http://your-server:port/api
```
