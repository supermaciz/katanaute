# PyKata - Development Guidelines

Development guidelines and conventions for the PyKata Python GUI client.

## Overview

PyKata is a desktop GUI client for the Katanaute Kata Training Tracker, built with:
- **Language**: Python 3.8+
- **GUI Framework**: CustomTkinter (modern Tkinter wrapper)
- **HTTP Client**: Requests library
- **Architecture**: Multi-view GUI application with API client

## Table of Contents

- [Technology Stack](#technology-stack)
- [Project Structure](#project-structure)
- [Development Setup](#development-setup)
- [Code Conventions](#code-conventions)
- [GUI Architecture](#gui-architecture)
- [Authentication](#authentication)
- [API Client](#api-client)
- [Common Development Tasks](#common-development-tasks)
- [Testing](#testing)
- [Debugging](#debugging)
- [TODO](#todo)

## Technology Stack

### CustomTkinter
- **Why chosen**: Modern UI, lightweight, cross-platform, no external system dependencies
- **Docs**: https://customtkinter.tomschimansky.com/
- **Key features**:
  - Modern widgets with dark/light themes
  - Built on standard Tkinter (included with Python)
  - Good form handling and layout management
  - Active development and community

### Requests
- Industry-standard HTTP client for Python
- Simple API for REST requests
- Good error handling and session management

### Python-dateutil
- Robust date/time parsing
- Handles various datetime formats
- Compatible with ISO 8601

## Project Structure

```
pykata/
├── pykata.py           # Main application and GUI views
├── api_client.py       # API client for backend communication
├── requirements.txt    # Python dependencies
├── .gitignore         # Git ignore patterns
├── README.md          # User documentation
└── CLAUDE.md          # This file - development guidelines
```

### File Responsibilities

**pykata.py**:
- Main application window (`PyKataApp`)
- View management and navigation
- GUI views:
  - `LoginView`: Authentication (email/password, device flow, registration)
  - `MainView`: Session list with refresh, create, delete
  - `CreateSessionView`: Session creation form
  - `SessionDetailView`: Session details display
- CustomTkinter widget composition
- Threading for background API calls

**api_client.py**:
- `APIClient` class for all backend communication
- Authentication methods (login, register, logout, device flow)
- Session CRUD operations
- Kata retrieval
- Token persistence (file-based)
- Error handling with custom exceptions

## Development Setup

### Initial Setup

```bash
# Navigate to pykata directory
cd pykata

# Create virtual environment
python3 -m venv venv
source venv/bin/activate  # Windows: venv\Scripts\activate

# Install dependencies
pip install -r requirements.txt

# Ensure backend is running
cd ../katanaute
mix phx.server
```

### Running the Application

```bash
cd pykata
source venv/bin/activate  # If not already activated
python pykata.py
```

### Development Workflow

1. Activate virtual environment
2. Make code changes
3. Run application to test
4. Use print statements or logging for debugging (see [Debugging](#debugging))
5. Test with backend running

## Code Conventions

### Python Style

Follow **PEP 8** conventions:
- Use 4 spaces for indentation
- Maximum line length: 100 characters (flexible for readability)
- Use snake_case for functions and variables
- Use PascalCase for classes
- Use UPPER_CASE for constants

### Type Hints

Use type hints for function signatures:

```python
def create_session(self, kata_id: int, practiced_at: str,
                  in_course: bool, notes: str = "") -> Dict[str, Any]:
    """Create a new session."""
    # Implementation...
```

### Docstrings

Use docstrings for all classes and public methods:

```python
def poll_device_token(self, device_code: str, interval: int = 5, timeout: int = 300) -> bool:
    """
    Poll for device authorization completion.
    Returns True if successful, False if denied or timed out.
    """
    # Implementation...
```

### Error Handling

Use custom exceptions for API errors:

```python
class KatanauteAPIError(Exception):
    """Custom exception for API errors."""
    pass

# Usage
raise KatanauteAPIError(f"Failed to create session: {error_msg}")
```

Catch and handle errors appropriately:

```python
try:
    response = requests.post(url, json=data, headers=self._headers())
    response.raise_for_status()
    return response.json()
except requests.exceptions.RequestException as e:
    raise KatanauteAPIError(f"Login failed: {e}")
```

### Constants

Define constants at module level:

```python
# Kata level colors (matching React frontend)
LEVEL_COLORS = {
    'yellow': '#EAB308',
    'orange': '#F97316',
    'green': '#22C55E',
    'blue': '#3B82F6',
    'brown': '#92400E',
    'shodan': '#1F2937'
}
```

## GUI Architecture

### CustomTkinter Basics

CustomTkinter uses a widget hierarchy similar to Tkinter:

```python
# Create a frame
frame = ctk.CTkFrame(parent)
frame.pack(fill="both", expand=True)

# Add widgets to frame
label = ctk.CTkLabel(frame, text="Hello")
label.pack(pady=10)

button = ctk.CTkButton(frame, text="Click", command=self.handle_click)
button.pack(pady=5)
```

### View Pattern

Each view is a `CTkFrame` subclass:

```python
class MyView(ctk.CTkFrame):
    """Description of the view."""

    def __init__(self, parent: PyKataApp, api: APIClient):
        super().__init__(parent)
        self.parent = parent
        self.api = api

        # Build UI
        self._create_widgets()

    def _create_widgets(self):
        """Create and layout widgets."""
        # Widget creation...
```

### Navigation

The main app handles view switching:

```python
class PyKataApp(ctk.CTk):
    def show_main_view(self):
        """Show the main session list view."""
        self.clear_frame()
        self.current_frame = MainView(self, self.api)
        self.current_frame.pack(fill="both", expand=True)

    def clear_frame(self):
        """Clear the current frame."""
        if self.current_frame:
            self.current_frame.destroy()
            self.current_frame = None
```

### Threading for API Calls

**IMPORTANT**: Never block the UI thread with API calls. Use threading:

```python
def load_thread():
    try:
        sessions = self.api.list_sessions()
        # Update UI using self.after()
        self.after(0, lambda: self._render_sessions(sessions))
    except KatanauteAPIError as e:
        self.after(0, lambda: self._show_error(str(e)))

threading.Thread(target=load_thread, daemon=True).start()
```

**Rules**:
- All API calls must run in background threads
- Use `self.after(0, callback)` to update UI from thread
- Mark threads as daemon (`daemon=True`)
- Show loading states while threads are running

### Layout Management

Use `.pack()` for most layouts:

```python
# Vertical stacking
widget1.pack(pady=10)
widget2.pack(pady=10)

# Horizontal layout
frame = ctk.CTkFrame(parent)
frame.pack(fill="x")
left_widget.pack(side="left", padx=5)
right_widget.pack(side="right", padx=5)

# Fill and expand
scrollable = ctk.CTkScrollableFrame(parent)
scrollable.pack(fill="both", expand=True)
```

### Common Widgets

**Label**:
```python
ctk.CTkLabel(parent, text="Text", font=ctk.CTkFont(size=14, weight="bold"))
```

**Button**:
```python
ctk.CTkButton(parent, text="Click", command=callback, width=100)
```

**Entry** (text input):
```python
entry = ctk.CTkEntry(parent, placeholder_text="Enter text", width=300)
value = entry.get()  # Get value
```

**Textbox** (multi-line):
```python
textbox = ctk.CTkTextbox(parent, width=400, height=200)
textbox.insert("1.0", "Initial text")
content = textbox.get("1.0", "end-1c")  # Get content
```

**Checkbox**:
```python
var = ctk.BooleanVar(value=False)
checkbox = ctk.CTkCheckBox(parent, text="Label", variable=var)
is_checked = var.get()  # Get value
```

**OptionMenu** (dropdown):
```python
var = ctk.StringVar(value="Option 1")
menu = ctk.CTkOptionMenu(parent, variable=var, values=["Option 1", "Option 2"])
selected = var.get()  # Get selection
```

**ScrollableFrame**:
```python
scrollable = ctk.CTkScrollableFrame(parent, label_text="Title")
scrollable.pack(fill="both", expand=True)
# Add widgets to scrollable
```

## Authentication

### Token Storage

Tokens are stored in `~/.pykata_token` as JSON:

```json
{
  "token": "access_token_here",
  "user": {"id": 1, "email": "user@example.com"}
}
```

File permissions are set to 0600 (Unix only) for security.

### Authentication Methods

**Email/Password**:
```python
self.api.login(email, password)
# Sets self.api.token and self.api.user
```

**Device Flow**:
```python
# 1. Start flow
flow_data = self.api.start_device_flow()
# Returns: device_code, user_code, verification_uri_complete, interval

# 2. Open browser to verification URI
webbrowser.open(flow_data['verification_uri_complete'])

# 3. Poll for completion
success = self.api.poll_device_token(
    device_code=flow_data['device_code'],
    interval=flow_data['interval']
)
# Returns True when approved, False if denied/timeout
```

**Registration**:
```python
self.api.register(email, password)
# Sets self.api.token and self.api.user
```

**Logout**:
```python
self.api.logout()
# Revokes token on server and deletes local token file
```

### Checking Authentication

```python
if self.api.is_authenticated():
    # User is logged in
    print(f"Logged in as: {self.api.user['email']}")
```

## API Client

### Making API Calls

All API calls use the `APIClient` class:

```python
# List sessions
sessions = self.api.list_sessions()

# Create session
session = self.api.create_session(
    kata_id=1,
    practiced_at="2025-01-15T10:30:00",
    in_course=True,
    notes="Great practice session!"
)

# Delete session
self.api.delete_session(session_id=5)
```

### Error Handling

API methods raise `KatanauteAPIError` on failure:

```python
try:
    sessions = self.api.list_sessions()
except KatanauteAPIError as e:
    # Handle error (show message to user)
    print(f"Error: {e}")
```

### Available API Methods

**Authentication**:
- `login(email, password)` → Dict
- `register(email, password)` → Dict
- `logout()` → None
- `get_current_user()` → Dict
- `start_device_flow()` → Dict
- `poll_device_token(device_code, interval, timeout)` → bool

**Sessions**:
- `list_sessions()` → List[Dict]
- `get_session(session_id)` → Dict
- `create_session(kata_id, practiced_at, in_course, notes)` → Dict
- `update_session(session_id, kata_id, practiced_at, in_course, notes)` → Dict
- `delete_session(session_id)` → None

**Katas**:
- `list_katas()` → List[Dict]
- `get_kata(kata_id)` → Dict

## Common Development Tasks

### Adding a New View

1. Create a new class inheriting from `ctk.CTkFrame`:
   ```python
   class NewView(ctk.CTkFrame):
       def __init__(self, parent: PyKataApp, api: APIClient):
           super().__init__(parent)
           self.parent = parent
           self.api = api
           # Build UI...
   ```

2. Add a method to `PyKataApp` to show the view:
   ```python
   def show_new_view(self):
       self.clear_frame()
       self.current_frame = NewView(self, self.api)
       self.current_frame.pack(fill="both", expand=True)
   ```

3. Call the method from buttons/menus:
   ```python
   button = ctk.CTkButton(frame, text="Go to New View", command=self.parent.show_new_view)
   ```

### Adding a New API Method

1. Add method to `APIClient` in `api_client.py`:
   ```python
   def my_new_method(self, param: str) -> Dict[str, Any]:
       """Description of what this does."""
       url = f"{self.base_url}/my/endpoint"
       try:
           response = requests.get(url, headers=self._headers())
           response.raise_for_status()
           return response.json()['data']
       except requests.exceptions.RequestException as e:
           raise KatanauteAPIError(f"Operation failed: {e}")
   ```

2. Use from GUI in a background thread:
   ```python
   def load_thread():
       try:
           result = self.api.my_new_method("param")
           self.after(0, lambda: self._handle_result(result))
       except KatanauteAPIError as e:
           self.after(0, lambda: self._handle_error(str(e)))

   threading.Thread(target=load_thread, daemon=True).start()
   ```

### Updating Dependencies

```bash
# Update requirements.txt after adding new packages
pip freeze > requirements.txt

# Or manually add to requirements.txt:
# new-package>=1.0.0
```

### Changing Theme/Appearance

```python
# In pykata.py, modify:
ctk.set_appearance_mode("dark")  # "dark", "light", or "system"
ctk.set_default_color_theme("blue")  # "blue", "green", "dark-blue"
```

## Testing

### Current Status

PyKata does not yet have automated tests.

### Planned Testing Strategy

**Unit Tests** (using pytest):
- Test `APIClient` methods with mocked requests
- Test utility functions
- Test data validation

**Integration Tests**:
- Test API client against running backend
- Verify token persistence
- Test error handling

**GUI Tests** (challenging with CustomTkinter):
- May require manual testing
- Focus on logic separation for testability

### Manual Testing Checklist

- [ ] Login with valid credentials
- [ ] Login with invalid credentials (verify error)
- [ ] Device flow authentication
- [ ] Registration
- [ ] Session list loading
- [ ] Session creation
- [ ] Session deletion
- [ ] Session detail view
- [ ] Logout
- [ ] Token persistence (close and reopen app)
- [ ] Refresh session list
- [ ] Error handling (disconnect backend, test error messages)

## Debugging

### Print Debugging

Use `print()` statements for quick debugging:

```python
print(f"Session data: {session}")
print(f"API response: {response.json()}")
```

### Logging

For more structured debugging, use Python's logging module:

```python
import logging

logging.basicConfig(level=logging.DEBUG, filename='pykata_debug.log')
logger = logging.getLogger(__name__)

logger.debug(f"Loading sessions: {sessions}")
logger.error(f"API error: {e}")
```

### Network Debugging

Use requests session with logging:

```python
import logging
import http.client as http_client

http_client.HTTPConnection.debuglevel = 1
logging.basicConfig(level=logging.DEBUG)
```

### GUI Debugging

**Check widget hierarchy**:
```python
# Print all children of a widget
for widget in frame.winfo_children():
    print(widget)
```

**Check widget state**:
```python
print(f"Button state: {button.cget('state')}")
print(f"Entry value: {entry.get()}")
```

### Common Issues

**Threading errors**:
- Make sure UI updates use `self.after(0, callback)`
- Ensure threads are marked as daemon

**API errors**:
- Check backend is running on correct port
- Verify authentication token is valid
- Check request/response format

**Layout issues**:
- Verify `pack()` or `grid()` is called on all widgets
- Check `fill` and `expand` parameters
- Use `pack_configure()` to modify existing layout

## TODO

### High Priority

- [ ] **Session editing**: Add ability to edit existing sessions
- [ ] **Unit tests**: Add pytest tests for `api_client.py`
- [ ] **Error recovery**: Better error messages and recovery flows
- [ ] **Input validation**: Client-side validation before API calls

### Medium Priority

- [ ] **Search/filter**: Add session search and filtering
- [ ] **Statistics**: Show training statistics and progress
- [ ] **Export**: Export sessions to Markdown/PDF
- [ ] **Preferences**: User preferences (theme, API URL) stored in config file
- [ ] **Loading indicators**: Better loading states with spinners

### Low Priority

- [ ] **Offline mode**: Cache sessions for offline viewing
- [ ] **Multi-language**: i18n support
- [ ] **Custom themes**: User-customizable color schemes
- [ ] **Keyboard shortcuts**: Add keyboard shortcuts for common actions
- [ ] **Session templates**: Save and reuse session templates

### Technical Debt

- [ ] Separate views into individual files (pykata.py is getting large)
- [ ] Add configuration file for settings
- [ ] Improve error handling with more specific exceptions
- [ ] Add logging throughout the application
- [ ] Consider using `asyncio` instead of threading for API calls

## Best Practices

### Do's

✅ **Use background threads for all API calls**
✅ **Update UI with `self.after(0, callback)` from threads**
✅ **Show loading states during API operations**
✅ **Handle errors gracefully with user-friendly messages**
✅ **Use type hints for better code clarity**
✅ **Follow PEP 8 style guidelines**
✅ **Add docstrings to all public methods**
✅ **Validate user input before making API calls**

### Don'ts

❌ **Don't block the UI thread with API calls**
❌ **Don't update UI directly from background threads**
❌ **Don't ignore exceptions (always handle or propagate)**
❌ **Don't hard-code URLs or configuration**
❌ **Don't use global variables for state**
❌ **Don't create circular dependencies between modules**

## Resources

- [CustomTkinter Documentation](https://customtkinter.tomschimansky.com/)
- [Requests Documentation](https://requests.readthedocs.io/)
- [Python Threading](https://docs.python.org/3/library/threading.html)
- [PEP 8 Style Guide](https://pep8.org/)
- [Python Type Hints](https://docs.python.org/3/library/typing.html)
- [Katanaute API Documentation](../CLAUDE.md)
