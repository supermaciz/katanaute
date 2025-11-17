#!/usr/bin/env python3
"""
PyKata - Python GUI Client for Katanaute Kata Training Tracker
Built with CustomTkinter for a modern, cross-platform desktop experience.
"""

import sys
import webbrowser
import threading
from datetime import datetime
from typing import Optional, List, Dict, Any

import customtkinter as ctk
from dateutil import parser as dateparser

from api_client import APIClient, KatanauteAPIError


# Configure CustomTkinter appearance
ctk.set_appearance_mode("dark")  # Modes: "System", "Dark", "Light"
ctk.set_default_color_theme("blue")  # Themes: "blue", "green", "dark-blue"


# Kata level colors (matching React frontend)
LEVEL_COLORS = {
    'yellow': '#EAB308',
    'orange': '#F97316',
    'green': '#22C55E',
    'blue': '#3B82F6',
    'brown': '#92400E',
    'shodan': '#1F2937'
}


class PyKataApp(ctk.CTk):
    """Main application window."""

    def __init__(self):
        super().__init__()

        self.title("PyKata - Kata Training Tracker")
        self.geometry("900x700")

        # Initialize API client
        self.api = APIClient()

        # Current view tracking
        self.current_frame: Optional[ctk.CTkFrame] = None

        # Show appropriate initial screen
        if self.api.is_authenticated():
            self.show_main_view()
        else:
            self.show_login_view()

    def clear_frame(self):
        """Clear the current frame."""
        if self.current_frame:
            self.current_frame.destroy()
            self.current_frame = None

    def show_login_view(self):
        """Show the login/authentication view."""
        self.clear_frame()
        self.current_frame = LoginView(self, self.api)
        self.current_frame.pack(fill="both", expand=True, padx=20, pady=20)

    def show_main_view(self):
        """Show the main session list view."""
        self.clear_frame()
        self.current_frame = MainView(self, self.api)
        self.current_frame.pack(fill="both", expand=True)

    def show_create_session_view(self):
        """Show the create session form."""
        self.clear_frame()
        self.current_frame = CreateSessionView(self, self.api)
        self.current_frame.pack(fill="both", expand=True, padx=20, pady=20)

    def show_session_detail_view(self, session: Dict[str, Any]):
        """Show session detail view."""
        self.clear_frame()
        self.current_frame = SessionDetailView(self, self.api, session)
        self.current_frame.pack(fill="both", expand=True, padx=20, pady=20)


class LoginView(ctk.CTkFrame):
    """Login and authentication view."""

    def __init__(self, parent: PyKataApp, api: APIClient):
        super().__init__(parent)
        self.parent = parent
        self.api = api

        # Title
        title = ctk.CTkLabel(self, text="PyKata Login", font=ctk.CTkFont(size=24, weight="bold"))
        title.pack(pady=(0, 30))

        # Tab view for different auth methods
        self.tabview = ctk.CTkTabview(self, width=500)
        self.tabview.pack(pady=20)

        self.tabview.add("Email/Password")
        self.tabview.add("Device Flow")
        self.tabview.add("Register")

        # Email/Password tab
        self._create_email_password_tab()

        # Device Flow tab
        self._create_device_flow_tab()

        # Register tab
        self._create_register_tab()

    def _create_email_password_tab(self):
        """Create email/password login tab."""
        tab = self.tabview.tab("Email/Password")

        ctk.CTkLabel(tab, text="Login with email and password").pack(pady=10)

        self.email_entry = ctk.CTkEntry(tab, placeholder_text="Email", width=300)
        self.email_entry.pack(pady=10)

        self.password_entry = ctk.CTkEntry(tab, placeholder_text="Password", width=300, show="*")
        self.password_entry.pack(pady=10)

        self.login_button = ctk.CTkButton(tab, text="Login", command=self._handle_login, width=300)
        self.login_button.pack(pady=10)

        self.login_status = ctk.CTkLabel(tab, text="", text_color="red")
        self.login_status.pack(pady=5)

    def _create_device_flow_tab(self):
        """Create device flow authentication tab."""
        tab = self.tabview.tab("Device Flow")

        ctk.CTkLabel(
            tab,
            text="Authenticate using your browser\n(recommended for headless/remote systems)",
            justify="center"
        ).pack(pady=10)

        self.device_start_button = ctk.CTkButton(
            tab,
            text="Start Device Flow",
            command=self._handle_device_flow,
            width=300
        )
        self.device_start_button.pack(pady=10)

        self.device_status = ctk.CTkLabel(tab, text="", wraplength=400)
        self.device_status.pack(pady=10)

        self.device_code_label = ctk.CTkLabel(
            tab,
            text="",
            font=ctk.CTkFont(size=20, weight="bold"),
            text_color="green"
        )
        self.device_code_label.pack(pady=10)

    def _create_register_tab(self):
        """Create registration tab."""
        tab = self.tabview.tab("Register")

        ctk.CTkLabel(tab, text="Create a new account").pack(pady=10)

        self.reg_email_entry = ctk.CTkEntry(tab, placeholder_text="Email", width=300)
        self.reg_email_entry.pack(pady=10)

        self.reg_password_entry = ctk.CTkEntry(tab, placeholder_text="Password", width=300, show="*")
        self.reg_password_entry.pack(pady=10)

        self.reg_confirm_entry = ctk.CTkEntry(tab, placeholder_text="Confirm Password", width=300, show="*")
        self.reg_confirm_entry.pack(pady=10)

        self.register_button = ctk.CTkButton(tab, text="Register", command=self._handle_register, width=300)
        self.register_button.pack(pady=10)

        self.register_status = ctk.CTkLabel(tab, text="", text_color="red")
        self.register_status.pack(pady=5)

    def _handle_login(self):
        """Handle email/password login."""
        email = self.email_entry.get().strip()
        password = self.password_entry.get()

        if not email or not password:
            self.login_status.configure(text="Please enter email and password", text_color="red")
            return

        self.login_button.configure(state="disabled", text="Logging in...")
        self.login_status.configure(text="", text_color="red")

        def login_thread():
            try:
                self.api.login(email, password)
                # Success - switch to main view
                self.after(0, self.parent.show_main_view)
            except KatanauteAPIError as e:
                self.after(0, lambda: self.login_status.configure(text=str(e), text_color="red"))
            finally:
                self.after(0, lambda: self.login_button.configure(state="normal", text="Login"))

        threading.Thread(target=login_thread, daemon=True).start()

    def _handle_register(self):
        """Handle user registration."""
        email = self.reg_email_entry.get().strip()
        password = self.reg_password_entry.get()
        confirm = self.reg_confirm_entry.get()

        if not email or not password:
            self.register_status.configure(text="Please enter email and password", text_color="red")
            return

        if password != confirm:
            self.register_status.configure(text="Passwords do not match", text_color="red")
            return

        if len(password) < 8:
            self.register_status.configure(text="Password must be at least 8 characters", text_color="red")
            return

        self.register_button.configure(state="disabled", text="Registering...")
        self.register_status.configure(text="", text_color="red")

        def register_thread():
            try:
                self.api.register(email, password)
                # Success - switch to main view
                self.after(0, self.parent.show_main_view)
            except KatanauteAPIError as e:
                self.after(0, lambda: self.register_status.configure(text=str(e), text_color="red"))
            finally:
                self.after(0, lambda: self.register_button.configure(state="normal", text="Register"))

        threading.Thread(target=register_thread, daemon=True).start()

    def _handle_device_flow(self):
        """Handle device flow authentication."""
        self.device_start_button.configure(state="disabled", text="Starting...")
        self.device_status.configure(text="Initializing device flow...")
        self.device_code_label.configure(text="")

        def device_flow_thread():
            try:
                # Start device flow
                flow_data = self.api.start_device_flow()

                device_code = flow_data['device_code']
                user_code = flow_data['user_code']
                verification_uri = flow_data['verification_uri_complete']
                interval = flow_data.get('interval', 5)

                # Update UI with instructions
                self.after(0, lambda: self.device_code_label.configure(text=f"Code: {user_code}"))
                self.after(0, lambda: self.device_status.configure(
                    text=f"Opening browser to authorize...\nIf browser doesn't open, visit:\n{verification_uri}"
                ))

                # Open browser
                webbrowser.open(verification_uri)

                # Poll for completion
                self.after(0, lambda: self.device_status.configure(
                    text=f"Waiting for authorization...\nApprove the request in your browser."
                ))

                success = self.api.poll_device_token(device_code, interval)

                if success:
                    # Success - switch to main view
                    self.after(0, self.parent.show_main_view)
                else:
                    self.after(0, lambda: self.device_status.configure(
                        text="Authorization failed or timed out. Please try again."
                    ))
                    self.after(0, lambda: self.device_code_label.configure(text=""))

            except KatanauteAPIError as e:
                self.after(0, lambda: self.device_status.configure(text=f"Error: {e}"))
            finally:
                self.after(0, lambda: self.device_start_button.configure(state="normal", text="Start Device Flow"))

        threading.Thread(target=device_flow_thread, daemon=True).start()


class MainView(ctk.CTkFrame):
    """Main session list view."""

    def __init__(self, parent: PyKataApp, api: APIClient):
        super().__init__(parent)
        self.parent = parent
        self.api = api
        self.sessions: List[Dict[str, Any]] = []
        self.katas: Dict[int, Dict[str, Any]] = {}

        # Header with user info and logout button
        header = ctk.CTkFrame(self)
        header.pack(fill="x", padx=20, pady=(20, 10))

        user_email = self.api.user.get('email', 'User') if self.api.user else 'User'
        ctk.CTkLabel(
            header,
            text=f"PyKata - Training Sessions",
            font=ctk.CTkFont(size=20, weight="bold")
        ).pack(side="left", padx=10)

        ctk.CTkLabel(header, text=f"Logged in as: {user_email}").pack(side="left", padx=20)

        ctk.CTkButton(header, text="Logout", command=self._handle_logout, width=100).pack(side="right", padx=10)
        ctk.CTkButton(
            header,
            text="+ New Session",
            command=self.parent.show_create_session_view,
            width=120
        ).pack(side="right", padx=5)
        ctk.CTkButton(header, text="Refresh", command=self._load_sessions, width=100).pack(side="right", padx=5)

        # Sessions list (scrollable)
        self.sessions_frame = ctk.CTkScrollableFrame(self, label_text="Training Sessions")
        self.sessions_frame.pack(fill="both", expand=True, padx=20, pady=10)

        # Load data
        self._load_sessions()

    def _load_sessions(self):
        """Load sessions from API."""
        # Clear existing sessions
        for widget in self.sessions_frame.winfo_children():
            widget.destroy()

        loading_label = ctk.CTkLabel(self.sessions_frame, text="Loading sessions...")
        loading_label.pack(pady=20)

        def load_thread():
            try:
                # Load katas first
                katas = self.api.list_katas()
                self.katas = {kata['id']: kata for kata in katas}

                # Load sessions
                sessions = self.api.list_sessions()
                # Sort by practiced_at (newest first)
                sessions.sort(key=lambda s: s.get('practiced_at', ''), reverse=True)
                self.sessions = sessions

                # Update UI
                self.after(0, self._render_sessions)
            except KatanauteAPIError as e:
                self.after(0, lambda: loading_label.configure(text=f"Error loading sessions: {e}"))

        threading.Thread(target=load_thread, daemon=True).start()

    def _render_sessions(self):
        """Render the sessions list."""
        # Clear loading message
        for widget in self.sessions_frame.winfo_children():
            widget.destroy()

        if not self.sessions:
            ctk.CTkLabel(
                self.sessions_frame,
                text="No training sessions yet.\nClick '+ New Session' to create one.",
                font=ctk.CTkFont(size=14)
            ).pack(pady=40)
            return

        # Render each session
        for session in self.sessions:
            self._render_session_item(session)

    def _render_session_item(self, session: Dict[str, Any]):
        """Render a single session item."""
        kata = self.katas.get(session['kata_id'], {})
        kata_name = kata.get('name', 'Unknown')
        kata_level = kata.get('level', 'yellow')

        # Session frame
        session_frame = ctk.CTkFrame(self.sessions_frame)
        session_frame.pack(fill="x", pady=5, padx=10)

        # Left side: kata info
        left_frame = ctk.CTkFrame(session_frame, fg_color="transparent")
        left_frame.pack(side="left", fill="both", expand=True, padx=10, pady=10)

        # Kata name and level badge
        top_frame = ctk.CTkFrame(left_frame, fg_color="transparent")
        top_frame.pack(fill="x")

        ctk.CTkLabel(
            top_frame,
            text=kata_name,
            font=ctk.CTkFont(size=16, weight="bold")
        ).pack(side="left", padx=(0, 10))

        level_color = LEVEL_COLORS.get(kata_level, '#EAB308')
        level_badge = ctk.CTkLabel(
            top_frame,
            text=kata_level.upper(),
            font=ctk.CTkFont(size=10, weight="bold"),
            text_color="white",
            fg_color=level_color,
            corner_radius=4,
            width=70,
            height=20
        )
        level_badge.pack(side="left")

        if session.get('in_course'):
            course_badge = ctk.CTkLabel(
                top_frame,
                text="IN COURSE",
                font=ctk.CTkFont(size=10, weight="bold"),
                text_color="white",
                fg_color="#8B5CF6",
                corner_radius=4,
                width=80,
                height=20
            )
            course_badge.pack(side="left", padx=5)

        # Date
        try:
            practiced_dt = dateparser.parse(session['practiced_at'])
            date_str = practiced_dt.strftime("%B %d, %Y at %I:%M %p")
        except:
            date_str = session.get('practiced_at', 'Unknown date')

        ctk.CTkLabel(
            left_frame,
            text=date_str,
            font=ctk.CTkFont(size=12),
            text_color="gray"
        ).pack(anchor="w", pady=(5, 0))

        # Notes preview
        if session.get('notes'):
            notes_preview = session['notes'][:80] + ('...' if len(session['notes']) > 80 else '')
            ctk.CTkLabel(
                left_frame,
                text=notes_preview,
                font=ctk.CTkFont(size=11),
                text_color="lightgray",
                wraplength=500,
                justify="left"
            ).pack(anchor="w", pady=(5, 0))

        # Right side: actions
        right_frame = ctk.CTkFrame(session_frame, fg_color="transparent")
        right_frame.pack(side="right", padx=10, pady=10)

        ctk.CTkButton(
            right_frame,
            text="View Details",
            command=lambda s=session: self.parent.show_session_detail_view(s),
            width=100
        ).pack(pady=2)

        ctk.CTkButton(
            right_frame,
            text="Delete",
            command=lambda s=session: self._delete_session(s),
            fg_color="darkred",
            hover_color="red",
            width=100
        ).pack(pady=2)

    def _delete_session(self, session: Dict[str, Any]):
        """Delete a session."""
        # Confirm deletion
        dialog = ctk.CTkToplevel(self)
        dialog.title("Confirm Deletion")
        dialog.geometry("400x150")
        dialog.grab_set()  # Make modal

        kata = self.katas.get(session['kata_id'], {})
        kata_name = kata.get('name', 'Unknown')

        ctk.CTkLabel(
            dialog,
            text=f"Delete session for {kata_name}?",
            font=ctk.CTkFont(size=14, weight="bold")
        ).pack(pady=20)

        button_frame = ctk.CTkFrame(dialog, fg_color="transparent")
        button_frame.pack(pady=10)

        def confirm():
            dialog.destroy()
            # Delete in background
            def delete_thread():
                try:
                    self.api.delete_session(session['id'])
                    self.after(0, self._load_sessions)
                except KatanauteAPIError as e:
                    print(f"Error deleting session: {e}")

            threading.Thread(target=delete_thread, daemon=True).start()

        ctk.CTkButton(button_frame, text="Cancel", command=dialog.destroy, width=100).pack(side="left", padx=10)
        ctk.CTkButton(
            button_frame,
            text="Delete",
            command=confirm,
            fg_color="darkred",
            hover_color="red",
            width=100
        ).pack(side="left", padx=10)

    def _handle_logout(self):
        """Handle user logout."""
        self.api.logout()
        self.parent.show_login_view()


class CreateSessionView(ctk.CTkFrame):
    """Create new session view."""

    def __init__(self, parent: PyKataApp, api: APIClient):
        super().__init__(parent)
        self.parent = parent
        self.api = api
        self.katas: List[Dict[str, Any]] = []

        # Header
        header = ctk.CTkFrame(self, fg_color="transparent")
        header.pack(fill="x", pady=(0, 20))

        ctk.CTkLabel(
            header,
            text="Create New Training Session",
            font=ctk.CTkFont(size=20, weight="bold")
        ).pack(side="left")

        ctk.CTkButton(
            header,
            text="← Back",
            command=self.parent.show_main_view,
            width=100
        ).pack(side="right")

        # Form frame
        form = ctk.CTkFrame(self)
        form.pack(fill="both", expand=True, padx=40, pady=20)

        # Kata selection
        ctk.CTkLabel(form, text="Kata:", font=ctk.CTkFont(size=14, weight="bold")).pack(anchor="w", pady=(10, 5))
        self.kata_var = ctk.StringVar(value="Select a kata...")
        self.kata_menu = ctk.CTkOptionMenu(form, variable=self.kata_var, values=["Loading..."], width=400)
        self.kata_menu.pack(anchor="w", pady=(0, 15))

        # Date/Time
        ctk.CTkLabel(form, text="Date & Time:", font=ctk.CTkFont(size=14, weight="bold")).pack(anchor="w", pady=(10, 5))
        self.datetime_entry = ctk.CTkEntry(form, placeholder_text="YYYY-MM-DD HH:MM:SS", width=400)
        self.datetime_entry.insert(0, datetime.now().strftime("%Y-%m-%d %H:%M:%S"))
        self.datetime_entry.pack(anchor="w", pady=(0, 15))

        # In Course checkbox
        self.in_course_var = ctk.BooleanVar(value=False)
        self.in_course_checkbox = ctk.CTkCheckBox(
            form,
            text="Part of structured learning path (In Course)",
            variable=self.in_course_var,
            font=ctk.CTkFont(size=14)
        )
        self.in_course_checkbox.pack(anchor="w", pady=(10, 15))

        # Notes
        ctk.CTkLabel(form, text="Notes (Markdown):", font=ctk.CTkFont(size=14, weight="bold")).pack(anchor="w", pady=(10, 5))
        self.notes_text = ctk.CTkTextbox(form, width=400, height=200)
        self.notes_text.pack(anchor="w", pady=(0, 15))

        # Buttons
        button_frame = ctk.CTkFrame(form, fg_color="transparent")
        button_frame.pack(anchor="w", pady=20)

        self.create_button = ctk.CTkButton(
            button_frame,
            text="Create Session",
            command=self._handle_create,
            width=150
        )
        self.create_button.pack(side="left", padx=5)

        ctk.CTkButton(
            button_frame,
            text="Cancel",
            command=self.parent.show_main_view,
            fg_color="gray",
            width=100
        ).pack(side="left", padx=5)

        # Status label
        self.status_label = ctk.CTkLabel(form, text="", text_color="red")
        self.status_label.pack(pady=10)

        # Load katas
        self._load_katas()

    def _load_katas(self):
        """Load available katas."""
        def load_thread():
            try:
                katas = self.api.list_katas()
                # Sort by level order
                level_order = ['yellow', 'orange', 'green', 'blue', 'brown', 'shodan']
                katas.sort(key=lambda k: level_order.index(k.get('level', 'yellow')))
                self.katas = katas

                # Update dropdown
                kata_names = [f"{k['name']} ({k['level'].upper()})" for k in katas]
                self.after(0, lambda: self.kata_menu.configure(values=kata_names))
                self.after(0, lambda: self.kata_var.set(kata_names[0] if kata_names else "No katas available"))
            except KatanauteAPIError as e:
                self.after(0, lambda: self.status_label.configure(text=f"Error loading katas: {e}"))

        threading.Thread(target=load_thread, daemon=True).start()

    def _handle_create(self):
        """Handle session creation."""
        # Get selected kata
        selected = self.kata_var.get()
        if not selected or selected == "Select a kata..." or selected == "No katas available":
            self.status_label.configure(text="Please select a kata", text_color="red")
            return

        # Find kata by name
        kata_name = selected.split(' (')[0]
        kata = next((k for k in self.katas if k['name'] == kata_name), None)
        if not kata:
            self.status_label.configure(text="Invalid kata selection", text_color="red")
            return

        # Get datetime
        datetime_str = self.datetime_entry.get().strip()
        if not datetime_str:
            self.status_label.configure(text="Please enter a date and time", text_color="red")
            return

        # Validate datetime format
        try:
            # Try to parse and convert to ISO format
            dt = dateparser.parse(datetime_str)
            datetime_iso = dt.isoformat()
        except:
            self.status_label.configure(text="Invalid date format. Use: YYYY-MM-DD HH:MM:SS", text_color="red")
            return

        # Get other fields
        in_course = self.in_course_var.get()
        notes = self.notes_text.get("1.0", "end-1c").strip()

        # Disable button
        self.create_button.configure(state="disabled", text="Creating...")
        self.status_label.configure(text="")

        def create_thread():
            try:
                self.api.create_session(
                    kata_id=kata['id'],
                    practiced_at=datetime_iso,
                    in_course=in_course,
                    notes=notes
                )
                # Success - go back to main view
                self.after(0, self.parent.show_main_view)
            except KatanauteAPIError as e:
                self.after(0, lambda: self.status_label.configure(text=f"Error: {e}", text_color="red"))
            finally:
                self.after(0, lambda: self.create_button.configure(state="normal", text="Create Session"))

        threading.Thread(target=create_thread, daemon=True).start()


class SessionDetailView(ctk.CTkFrame):
    """Session detail view."""

    def __init__(self, parent: PyKataApp, api: APIClient, session: Dict[str, Any]):
        super().__init__(parent)
        self.parent = parent
        self.api = api
        self.session = session

        # Header
        header = ctk.CTkFrame(self, fg_color="transparent")
        header.pack(fill="x", pady=(0, 20), padx=20)

        ctk.CTkLabel(
            header,
            text="Session Details",
            font=ctk.CTkFont(size=20, weight="bold")
        ).pack(side="left")

        ctk.CTkButton(
            header,
            text="← Back to List",
            command=self.parent.show_main_view,
            width=120
        ).pack(side="right")

        # Content frame
        content = ctk.CTkScrollableFrame(self)
        content.pack(fill="both", expand=True, padx=40, pady=20)

        # Load and display kata info
        self._load_and_display(content)

    def _load_and_display(self, content: ctk.CTkScrollableFrame):
        """Load kata info and display session details."""
        def load_thread():
            try:
                kata = self.api.get_kata(self.session['kata_id'])
                self.after(0, lambda: self._render_details(content, kata))
            except KatanauteAPIError as e:
                self.after(0, lambda: ctk.CTkLabel(content, text=f"Error loading kata: {e}").pack())

        threading.Thread(target=load_thread, daemon=True).start()

    def _render_details(self, content: ctk.CTkScrollableFrame, kata: Dict[str, Any]):
        """Render session details."""
        # Kata name and level
        kata_frame = ctk.CTkFrame(content, fg_color="transparent")
        kata_frame.pack(fill="x", pady=10)

        ctk.CTkLabel(
            kata_frame,
            text=kata['name'],
            font=ctk.CTkFont(size=24, weight="bold")
        ).pack(side="left", padx=(0, 10))

        level_color = LEVEL_COLORS.get(kata['level'], '#EAB308')
        ctk.CTkLabel(
            kata_frame,
            text=kata['level'].upper(),
            font=ctk.CTkFont(size=12, weight="bold"),
            text_color="white",
            fg_color=level_color,
            corner_radius=4,
            width=80,
            height=24
        ).pack(side="left")

        if self.session.get('in_course'):
            ctk.CTkLabel(
                kata_frame,
                text="IN COURSE",
                font=ctk.CTkFont(size=12, weight="bold"),
                text_color="white",
                fg_color="#8B5CF6",
                corner_radius=4,
                width=90,
                height=24
            ).pack(side="left", padx=5)

        # Date
        try:
            practiced_dt = dateparser.parse(self.session['practiced_at'])
            date_str = practiced_dt.strftime("%A, %B %d, %Y at %I:%M %p")
        except:
            date_str = self.session.get('practiced_at', 'Unknown date')

        ctk.CTkLabel(
            content,
            text=f"📅 {date_str}",
            font=ctk.CTkFont(size=14),
            text_color="lightgray"
        ).pack(anchor="w", pady=10)

        # Notes section
        if self.session.get('notes'):
            ctk.CTkLabel(
                content,
                text="Notes:",
                font=ctk.CTkFont(size=16, weight="bold")
            ).pack(anchor="w", pady=(20, 10))

            notes_frame = ctk.CTkFrame(content)
            notes_frame.pack(fill="both", expand=True, pady=10)

            notes_text = ctk.CTkTextbox(notes_frame, wrap="word", height=300)
            notes_text.pack(fill="both", expand=True, padx=10, pady=10)
            notes_text.insert("1.0", self.session['notes'])
            notes_text.configure(state="disabled")  # Read-only
        else:
            ctk.CTkLabel(
                content,
                text="No notes for this session.",
                font=ctk.CTkFont(size=14),
                text_color="gray"
            ).pack(anchor="w", pady=20)


def main():
    """Main entry point."""
    app = PyKataApp()
    app.mainloop()


if __name__ == "__main__":
    main()
