package main

import (
	"fmt"

	"github.com/charmbracelet/bubbles/list"
	"github.com/charmbracelet/bubbles/spinner"
	tea "github.com/charmbracelet/bubbletea"
	"github.com/charmbracelet/glamour"
	"github.com/charmbracelet/lipgloss"
)

var listStyle = lipgloss.NewStyle().Margin(1, 2)
var textStyle = lipgloss.NewStyle().
	//Border(lipgloss.RoundedBorder()).
	Padding(1, 3).
	Margin(1, 1)

type Model struct {
	config   *Config
	list     list.Model
	spinner  spinner.Model
	data     any
	sessions []*Session
}

func NewModel(config *Config) Model {
	s := spinner.New()
	s.Spinner = spinner.Hamburger
	s.Style = lipgloss.NewStyle().Foreground(lipgloss.Color("205"))
	return Model{
		config:  config,
		spinner: s,
	}
}

func checkServer(config *Config) tea.Msg {
	sessions, err := FetchSessions(config)
	if err != nil {
		return errMsg{err}
	}
	return sessions
}

type errMsg struct{ err error }

// For messages that contain errors it's often handy to also implement the
// error interface on the message.
func (e errMsg) Error() string { return e.err.Error() }

func toItems(sessions []*Session) []list.Item {
	items := make([]list.Item, len(sessions))
	for i, session := range sessions {
		items[i] = session
	}
	return items
}

func (m Model) Init() tea.Cmd {
	checkServerCmd := func() tea.Msg {
		return checkServer(m.config)
	}

	return tea.Batch(tea.Sequence(checkServerCmd, tea.WindowSize()), m.spinner.Tick)
}

func (m Model) Update(msg tea.Msg) (tea.Model, tea.Cmd) {
	var cmd tea.Cmd
	switch msg := msg.(type) {
	case tea.KeyMsg:
		if msg.String() == "ctrl+c" {
			return m, tea.Quit
		}
	case errMsg:
		return m, tea.Quit
	case []*Session:
		m.sessions = msg
		m.list = list.New(toItems(msg), list.NewDefaultDelegate(), 0, 0)
		m.list.Title = "Kata training sessions"
		m.list, cmd = m.list.Update(msg)
		return m, cmd
	case tea.WindowSizeMsg:
		if m.sessions != nil {
			h, v := listStyle.GetFrameSize()
			m.list.SetSize(msg.Width-h, msg.Height-v)
		}
	}
	if m.sessions != nil {
		m.list, cmd = m.list.Update(msg)
		return m, cmd
	} else {
		m.spinner, cmd = m.spinner.Update(msg)
		return m, cmd
	}
}

func (m Model) View() string {
	if m.sessions == nil {
		return m.renderLoadingView()
	}

	item, ok := m.list.SelectedItem().(*Session)
	if !ok {
		return m.renderListOnlyView()
	}

	return m.renderDetailedView(item)
}

func (m Model) renderLoadingView() string {
	return listStyle.Render(fmt.Sprintf("\n\n   %s Waiting for Katanaute API\n\n", m.spinner.View()))
}

func (m Model) renderListOnlyView() string {
	return listStyle.Render(m.list.View())
}

func (m Model) renderDetailedView(session *Session) string {
	listView := listStyle.Render(m.list.View())
	notesView := m.renderSessionNotes(session)

	return lipgloss.JoinHorizontal(
		lipgloss.Top,
		listView,
		notesView,
	)
}

func (m Model) renderSessionNotes(session *Session) string {
	renderedNotes, err := glamour.Render(session.Notes, "dark")
	if err != nil {
		renderedNotes = session.Notes + "\n\n" + err.Error()
	}
	return textStyle.Render(renderedNotes)
}
