package main

import (
	"fmt"
	"log"
	"sort"
	"time"

	"fyne.io/fyne/v2"
	"fyne.io/fyne/v2/app"
	"fyne.io/fyne/v2/container"
	"fyne.io/fyne/v2/dialog"
	"fyne.io/fyne/v2/layout"
	"fyne.io/fyne/v2/widget"
)

// App holds the application state
type App struct {
	config     *Config
	fyneApp    fyne.App
	mainWindow fyne.Window
	sessions   []*Session
	katas      []*Kata
}

func main() {
	// Load config
	config, err := NewConfig()
	if err != nil {
		log.Fatal("Failed to load config:", err)
	}

	// Create Fyne app
	fyneApp := app.New()
	mainWindow := fyneApp.NewWindow("Katafyne - Kata Training Tracker")
	mainWindow.Resize(fyne.NewSize(800, 600))

	application := &App{
		config:     config,
		fyneApp:    fyneApp,
		mainWindow: mainWindow,
	}

	// Check if authenticated
	if config.APIToken == "" {
		application.showLoginView()
	} else {
		application.showMainView()
	}

	mainWindow.ShowAndRun()
}

// showLoginView displays the authentication screen
func (a *App) showLoginView() {
	title := widget.NewLabel("Katafyne Authentication")
	title.TextStyle = fyne.TextStyle{Bold: true}

	instructions := widget.NewLabel("Click below to start authentication")
	statusLabel := widget.NewLabel("")
	statusLabel.Wrapping = fyne.TextWrapWord

	loginButton := widget.NewButton("Login with Device Flow", func() {
		loginButton.Disable()
		statusLabel.SetText("Starting authentication...")

		tokenChan, errChan := AuthenticateWithDeviceFlow(a.config.BaseURL, func(message string) {
			statusLabel.SetText(message)
		})

		// Wait for result in a goroutine
		go func() {
			select {
			case token := <-tokenChan:
				if token != "" {
					// Save token
					configFile := &ConfigFile{
						APIToken: token,
						BaseURL:  a.config.BaseURL,
					}
					if err := SaveConfig(configFile); err != nil {
						dialog.ShowError(fmt.Errorf("failed to save config: %w", err), a.mainWindow)
						loginButton.Enable()
						return
					}

					a.config.APIToken = token
					statusLabel.SetText("Authentication successful!")

					// Show main view after a short delay
					time.AfterFunc(1*time.Second, func() {
						a.showMainView()
					})
				}
			case err := <-errChan:
				if err != nil {
					dialog.ShowError(err, a.mainWindow)
					loginButton.Enable()
					statusLabel.SetText("Authentication failed. Click to retry.")
				}
			}
		}()
	})

	content := container.NewVBox(
		layout.NewSpacer(),
		container.NewCenter(title),
		layout.NewSpacer(),
		container.NewCenter(instructions),
		layout.NewSpacer(),
		container.NewCenter(loginButton),
		layout.NewSpacer(),
		statusLabel,
		layout.NewSpacer(),
	)

	a.mainWindow.SetContent(content)
}

// showMainView displays the main application screen
func (a *App) showMainView() {
	// Fetch sessions and katas
	a.refreshData()

	// Create UI components
	title := widget.NewLabel("Training Sessions")
	title.TextStyle = fyne.TextStyle{Bold: true}

	sessionList := widget.NewList(
		func() int {
			return len(a.sessions)
		},
		func() fyne.CanvasObject {
			return container.NewVBox(
				widget.NewLabel(""),
				widget.NewLabel(""),
			)
		},
		func(id widget.ListItemID, obj fyne.CanvasObject) {
			if id >= len(a.sessions) {
				return
			}
			session := a.sessions[id]
			container := obj.(*fyne.Container)

			// Title: Kata name with level badge
			titleLabel := container.Objects[0].(*widget.Label)
			titleLabel.SetText(fmt.Sprintf("%s [%s]", session.Kata.Name, session.Kata.Level))
			titleLabel.TextStyle = fyne.TextStyle{Bold: true}

			// Subtitle: Date and course indicator
			subtitleLabel := container.Objects[1].(*widget.Label)
			courseIndicator := ""
			if session.InCourse {
				courseIndicator = " 📚"
			}
			subtitleLabel.SetText(fmt.Sprintf("%s%s", session.PracticedAt.Format("2006-01-02 15:04"), courseIndicator))
		},
	)

	// Detail panel for selected session
	detailLabel := widget.NewLabel("Select a session to view details")
	detailLabel.Wrapping = fyne.TextWrapWord

	sessionList.OnSelected = func(id widget.ListItemID) {
		if id >= len(a.sessions) {
			return
		}
		session := a.sessions[id]
		details := fmt.Sprintf("Kata: %s (%s)\nDate: %s\nIn Course: %t\n\nNotes:\n%s",
			session.Kata.Name,
			session.Kata.Level,
			session.PracticedAt.Format("2006-01-02 15:04"),
			session.InCourse,
			session.Notes)
		detailLabel.SetText(details)
	}

	// Buttons
	refreshButton := widget.NewButton("Refresh", func() {
		a.refreshData()
		sessionList.Refresh()
		detailLabel.SetText("Select a session to view details")
	})

	addButton := widget.NewButton("Add Session", func() {
		a.showCreateSessionDialog()
	})

	logoutButton := widget.NewButton("Logout", func() {
		a.logout()
	})

	// Layout
	leftPanel := container.NewBorder(
		container.NewVBox(title, container.NewHBox(refreshButton, addButton, logoutButton)),
		nil,
		nil,
		nil,
		sessionList,
	)

	rightPanel := container.NewBorder(
		widget.NewLabel("Session Details"),
		nil,
		nil,
		nil,
		container.NewScroll(detailLabel),
	)

	split := container.NewHSplit(leftPanel, rightPanel)
	split.Offset = 0.5

	a.mainWindow.SetContent(split)
}

// showCreateSessionDialog shows a dialog to create a new session
func (a *App) showCreateSessionDialog() {
	// Create form
	kataSelect := widget.NewSelect([]string{}, func(value string) {})
	for _, kata := range a.katas {
		kataSelect.Options = append(kataSelect.Options, fmt.Sprintf("%s (%s)", kata.Name, kata.Level))
	}

	notesEntry := widget.NewMultiLineEntry()
	notesEntry.SetPlaceHolder("Training notes (Markdown supported)")

	inCourseCheck := widget.NewCheck("Part of structured course", func(bool) {})

	form := &widget.Form{
		Items: []*widget.FormItem{
			{Text: "Kata", Widget: kataSelect},
			{Text: "Notes", Widget: notesEntry},
			{Text: "In Course", Widget: inCourseCheck},
		},
		OnSubmit: func() {
			// Find selected kata
			var selectedKata *Kata
			for i, opt := range kataSelect.Options {
				if opt == kataSelect.Selected {
					if i < len(a.katas) {
						selectedKata = a.katas[i]
					}
					break
				}
			}

			if selectedKata == nil {
				dialog.ShowError(fmt.Errorf("please select a kata"), a.mainWindow)
				return
			}

			// Create session
			session := &SessionInput{
				Session: Session{
					PracticedAt: time.Now(),
					InCourse:    inCourseCheck.Checked,
					Notes:       notesEntry.Text,
				},
				KataID: selectedKata.ID,
			}

			err := CreateSession(a.config, session)
			if err != nil {
				dialog.ShowError(err, a.mainWindow)
				return
			}

			dialog.ShowInformation("Success", "Session created successfully!", a.mainWindow)
			a.refreshData()
			a.showMainView()
		},
		OnCancel: func() {},
	}

	dialog.ShowForm("Create Training Session", "Create", "Cancel", form.Items, func(submitted bool) {
		if submitted {
			form.OnSubmit()
		}
	}, a.mainWindow)
}

// refreshData fetches sessions and katas from the API
func (a *App) refreshData() {
	sessions, err := FetchSessions(a.config)
	if err != nil {
		dialog.ShowError(fmt.Errorf("failed to fetch sessions: %w", err), a.mainWindow)
		return
	}

	// Sort sessions by date (newest first)
	sort.Slice(sessions, func(i, j int) bool {
		return sessions[i].PracticedAt.After(sessions[j].PracticedAt)
	})

	a.sessions = sessions

	katas, err := FetchKatas(a.config)
	if err != nil {
		dialog.ShowError(fmt.Errorf("failed to fetch katas: %w", err), a.mainWindow)
		return
	}

	a.katas = katas
}

// logout clears the stored token and returns to login view
func (a *App) logout() {
	configFile := &ConfigFile{
		APIToken: "",
		BaseURL:  a.config.BaseURL,
	}
	if err := SaveConfig(configFile); err != nil {
		dialog.ShowError(err, a.mainWindow)
		return
	}

	a.config.APIToken = ""
	a.showLoginView()
}
