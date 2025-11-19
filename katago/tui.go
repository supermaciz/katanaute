package main

import (
	"fmt"
	"log"
	"sort"
	"time"

	"github.com/supermaciz/katanaute/katagocore"

	"github.com/charmbracelet/bubbles/key"
	"github.com/charmbracelet/bubbles/list"
	"github.com/charmbracelet/bubbles/spinner"
	tea "github.com/charmbracelet/bubbletea"
	"github.com/charmbracelet/glamour"
	"github.com/charmbracelet/huh"
	"github.com/charmbracelet/lipgloss"
)

type viewType uint

func (v viewType) String() string {
	return [...]string{"StartView", "ListView", "CreateSessionView"}[v]
}

const (
	StartView viewType = iota
	ListView
	CreateSessionView
)

var listStyle = lipgloss.NewStyle().Margin(1, 2)
var textStyle = lipgloss.NewStyle().Margin(1, 1)
var headerStyle = lipgloss.NewStyle().
	Margin(1, 1).
	Foreground(lipgloss.Color("205")).
	Border(lipgloss.RoundedBorder())
var formStyle = lipgloss.NewStyle().Margin(2, 1)

type Model struct {
	config     *katagocore.Config
	list       list.Model
	spinner    spinner.Model
	sessions   []*katagocore.Session
	katas      []*katagocore.Kata
	viewType   viewType
	form       *huh.Form
	newSession *katagocore.SessionInput
}

func NewModel(config *katagocore.Config) Model {
	s := spinner.New()
	s.Spinner = spinner.Hamburger
	s.Style = lipgloss.NewStyle().Foreground(lipgloss.Color("205"))
	return Model{
		config:   config,
		spinner:  s,
		viewType: StartView,
	}
}

func checkServer(config *katagocore.Config) tea.Msg {
	sessions, err := katagocore.FetchSessions(config)
	if err != nil {
		return errMsg{err}
	}
	return sessions
}

func fetchKataCmd(config *katagocore.Config) tea.Msg {
	katas, err := katagocore.FetchKatas(config)
	if err != nil {
		return err
	}
	log.Printf("Fetched %d katas", len(katas))
	return katas
}

func customKeys() []key.Binding {
	return []key.Binding{
		key.NewBinding(
			key.WithKeys("a"),
			key.WithHelp("a", "add item"),
		),
	}
}

type errMsg struct{ err error }

// For messages that contain errors it's often handy to also implement the
// error interface on the message.
func (e errMsg) Error() string { return e.err.Error() }

func toItems(sessions []*katagocore.Session) []list.Item {
	items := make([]list.Item, len(sessions))
	for i, session := range sessions {
		items[i] = SessionListItem{session}
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
		if m.viewType == ListView && msg.String() == "a" {
			//m.viewType = CreateSessionView
			log.Println("Create session")
			return m, func() tea.Msg { return fetchKataCmd(m.config) }
		}
	case errMsg:
		log.Println("Error:", msg.err)
		return m, tea.Quit
	case []*katagocore.Session:
		m.viewType = ListView
		m.sessions = msg
		sort.Slice(m.sessions, func(i, j int) bool {
			return m.sessions[i].PracticedAt.After(m.sessions[j].PracticedAt)
		})
		m.list = list.New(toItems(m.sessions), list.NewDefaultDelegate(), 0, 0)
		m.list.Title = "Kata training sessions"
		m.list.AdditionalShortHelpKeys = customKeys
		m.list.AdditionalFullHelpKeys = customKeys
		m.list, cmd = m.list.Update(msg)
		return m, cmd
	case []*katagocore.Kata:
		log.Println("Got katas")
		m.viewType = CreateSessionView
		m.katas = msg
		m.buildCreateSessionForm()
		cmd = m.form.Init()
		return m, cmd
	case tea.WindowSizeMsg:
		if m.sessions != nil {
			h, v := listStyle.GetFrameSize()
			m.list.SetSize(msg.Width-h, msg.Height-v)
		}
	}
	if m.form != nil && (m.form.State == huh.StateCompleted) {
		checkServerCmd := func() tea.Msg {
			return checkServer(m.config)
		}
		err := katagocore.CreateSession(m.config, m.newSession)
		if err != nil {
			log.Println("Error creating session:", err)
		}
		m.viewType = StartView
		m.form = nil
		m.newSession = nil
		m.sessions = nil
		return m, tea.Batch(tea.Sequence(checkServerCmd, tea.WindowSize()), m.spinner.Tick)
	} else if m.viewType == CreateSessionView && m.form != nil {
		form, cmd := m.form.Update(msg)
		if f, ok := form.(*huh.Form); ok {
			m.form = f
		}
		return m, cmd
	} else if m.sessions != nil {
		m.list, cmd = m.list.Update(msg)
		return m, cmd
	} else {
		m.spinner, cmd = m.spinner.Update(msg)
		return m, cmd
	}
}

func (m *Model) buildCreateSessionForm() {
	kataOptions := make([]huh.Option[int], len(m.katas))
	for i, kata := range m.katas {
		kataOptions[i] = huh.NewOption(kata.Name, kata.ID)
	}
	m.newSession = new(katagocore.SessionInput)
	var dateTimeVal string
	m.form = huh.NewForm(
		huh.NewGroup(
			huh.NewInput().
				Title("Date and time").
				Placeholder("2025-01-01 15:00").
				Value(&dateTimeVal).
				Validate(func(s string) error {
					parse, err := time.Parse("2006-01-02 15:04", s)
					if err != nil {
						return err
					}
					m.newSession.PracticedAt = parse
					return nil
				}),
			huh.NewSelect[int]().
				Title("Select a kata").
				Options(kataOptions...).
				Value(&(m.newSession.KataID)),
			huh.NewSelect[bool]().
				Title("Location").
				Options(
					huh.NewOption("Regular Course Session (dojo or outside)", true),
					huh.NewOption("Independent Practice", false),
				).
				Value(&(m.newSession.InCourse)),
			huh.NewText().Title("Notes").Value(&(m.newSession.Notes)),
		),
	)
}

func (m Model) View() string {
	switch m.viewType {
	case StartView:
		return m.renderLoadingView()
	case ListView:
		listItem, ok := m.list.SelectedItem().(SessionListItem)
		if !ok {
			return m.renderListOnlyView()
		}
		return m.renderDetailedView(listItem.Session)
	case CreateSessionView:
		//log.Println("Creating session view")
		return m.renderCreateSessionView()
	default:
		return "ERROR: Unknown view type"
	}

}

func (m Model) renderLoadingView() string {
	return listStyle.Render(fmt.Sprintf("\n\n   %s Waiting for Katanaute API\n\n", m.spinner.View()))
}

func (m Model) renderListOnlyView() string {
	return listStyle.Render(m.list.View())
}

func (m Model) renderDetailedView(session *katagocore.Session) string {
	listView := listStyle.Render(m.list.View())
	headerView := m.renderSessionHeader(session)
	notesView := m.renderSessionNotes(session)

	return lipgloss.JoinHorizontal(
		lipgloss.Top,
		listView,
		lipgloss.JoinVertical(lipgloss.Left, headerView, notesView),
	)
}

func (m Model) renderSessionHeader(session *katagocore.Session) string {
	if session.InCourse {
		return headerStyle.Render("  🥋  Dojo Course Session  ")
	}
	return headerStyle.Render("  📝  Independent Practice  ")
}

func (m Model) renderSessionNotes(session *katagocore.Session) string {
	renderedNotes, err := glamour.Render(session.Notes, "dark")
	if err != nil {
		renderedNotes = session.Notes + "\n\n" + err.Error()
	}
	return textStyle.Render(renderedNotes)
}

func (m Model) renderCreateSessionView() string {
	listView := listStyle.Render(m.list.View())
	formView := formStyle.Render(m.form.View())

	return lipgloss.JoinHorizontal(
		lipgloss.Top,
		listView,
		formView,
	)
}
