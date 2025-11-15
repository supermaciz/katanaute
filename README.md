# Katanaute

A kata (Uechi-Ryu Karate) training tracker with a Phoenix backend and multiple clients.

## Purpose

It's useless. I'm doing this for fun and to learn some stuff. 

## Project Structure

- **katanaute/** - Backend: API and LiveView UI (Elixir/Phoenix)
- **katago/** - Terminal UI client (Go/Bubble Tea)
- **katareact/** - Web UI (JavaScript/React)

## Quick Start

### Backend (Elixir/Phoenix)

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

- Track kata practice sessions
- Record session notes with Markdown support
- Browse sessions via web UI or terminal
- RESTful API for external integrations

## Configuration

The TUI client connects to `http://localhost:4000/api` by default. Override with:

```bash
export KATANAUTE_API_URL=http://your-server:port/api
```
