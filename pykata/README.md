# PyKata - Python GUI Client

A modern desktop GUI client for the Katanaute Kata Training Tracker, built with Python and CustomTkinter.

## Features

- 🎨 **Modern UI**: Clean, modern interface with dark theme using CustomTkinter
- 🔐 **Multiple Auth Methods**:
  - Email/password login
  - Device flow authentication (browser-based, ideal for remote systems)
  - User registration
- 📊 **Session Management**:
  - View all training sessions in a scrollable list
  - Color-coded kata level badges (Yellow → Shodan)
  - Create new sessions with Markdown notes
  - View detailed session information
  - Delete sessions
- 🔄 **Real-time Updates**: Refresh session list on demand
- 💾 **Token Persistence**: Stays logged in between sessions

## Prerequisites

- Python 3.8 or higher
- Katanaute backend running on `http://localhost:4000` (or configure custom URL)

## Installation

1. **Navigate to the pykata directory**:
   ```bash
   cd pykata
   ```

2. **Create a virtual environment (recommended)**:
   ```bash
   python3 -m venv venv
   source venv/bin/activate  # On Windows: venv\Scripts\activate
   ```

3. **Install dependencies**:
   ```bash
   pip install -r requirements.txt
   ```

## Usage

### Starting PyKata

```bash
python pykata.py
```

Or make it executable and run directly:
```bash
chmod +x pykata.py
./pykata.py
```

### First Time Setup

1. **Launch the application**
2. **Choose an authentication method**:
   - **Email/Password**: Enter your credentials and click "Login"
   - **Device Flow**: Click "Start Device Flow" and follow the browser prompt
   - **Register**: Create a new account with email and password

### Managing Sessions

**View Sessions**:
- Sessions are displayed in reverse chronological order (newest first)
- Each session shows:
  - Kata name and level (color-coded badge)
  - Date and time of practice
  - Notes preview (if available)
  - "In Course" indicator (if applicable)

**Create a Session**:
1. Click "+ New Session" button
2. Select a kata from the dropdown
3. Enter date/time (defaults to current time)
4. Check "In Course" if part of structured learning
5. Add optional Markdown notes
6. Click "Create Session"

**View Session Details**:
- Click "View Details" on any session
- See full kata information, date, and complete notes

**Delete a Session**:
- Click "Delete" button on a session
- Confirm the deletion in the dialog

**Logout**:
- Click the "Logout" button in the top-right corner
- This revokes your access token

## Configuration

### Custom API URL

Set the `KATANAUTE_API_URL` environment variable to point to a different backend:

```bash
export KATANAUTE_API_URL=http://your-server:4000/api
python pykata.py
```

Or modify the `base_url` in `api_client.py`.

### Token Storage

Authentication tokens are stored in `~/.pykata_token` with restricted permissions (Unix systems only).

## Architecture

### Components

**api_client.py**:
- API client for Katanaute backend
- Handles authentication (email/password, device flow, registration)
- Manages sessions and katas
- Token persistence

**pykata.py**:
- Main application and GUI
- CustomTkinter-based interface
- Multiple views:
  - `LoginView`: Authentication screen
  - `MainView`: Session list
  - `CreateSessionView`: Session creation form
  - `SessionDetailView`: Session details

### Authentication Flow

**Email/Password**:
1. User enters email and password
2. POST to `/api/auth/token`
3. Receive and store access token
4. Include token in subsequent requests

**Device Flow**:
1. POST to `/api/auth/device/code` to get device code and user code
2. Open browser to verification URI
3. User logs in and approves device
4. Poll `/api/auth/device/token` until approved
5. Receive and store access token

**Registration**:
1. User enters email and password
2. POST to `/api/auth/register`
3. Receive and store access token
4. Automatically logged in

### GUI Library: CustomTkinter

PyKata uses **CustomTkinter** for its GUI, offering:
- Modern, clean widgets with theming support
- Built on standard Python Tkinter (no external system dependencies)
- Cross-platform compatibility (Windows, macOS, Linux)
- Dark/light theme support
- Easy-to-use API for forms and layouts

## Development

### Project Structure

```
pykata/
├── pykata.py           # Main application
├── api_client.py       # API client
├── requirements.txt    # Python dependencies
├── .gitignore         # Git ignore patterns
├── README.md          # This file
└── CLAUDE.md          # Development guidelines
```

### Dependencies

- **customtkinter**: Modern UI framework
- **requests**: HTTP client for API calls
- **python-dateutil**: Date/time parsing and formatting

### Running from Source

```bash
# Install dependencies
pip install -r requirements.txt

# Run application
python pykata.py
```

### Building a Standalone Executable (Optional)

You can use PyInstaller to create a standalone executable:

```bash
pip install pyinstaller
pyinstaller --onefile --windowed --name PyKata pykata.py
```

The executable will be in the `dist/` directory.

## Troubleshooting

### Can't connect to backend
- Ensure the Katanaute Phoenix backend is running on `http://localhost:4000`
- Check if `KATANAUTE_API_URL` environment variable is set correctly
- Verify network connectivity

### Authentication fails
- Check email and password are correct
- For device flow, ensure you complete authorization in the browser
- Check that the backend is running and accessible

### Sessions not loading
- Ensure you're authenticated (token is valid)
- Try clicking "Refresh" to reload
- Check backend logs for errors

### Token issues
- Delete `~/.pykata_token` and re-authenticate
- Check file permissions on the token file

## Comparison with Other Clients

| Feature | PyKata (GUI) | React Web | Go TUI | Phoenix LiveView |
|---------|-------------|-----------|--------|------------------|
| Interface | Desktop GUI | Web Browser | Terminal | Web Browser |
| Auth Methods | Email/Password, Device Flow, Register | Email/Password | Device Flow | Email/Password |
| Create Sessions | ✅ | ✅ | ✅ | ✅ |
| View Sessions | ✅ | ✅ | ✅ | ✅ |
| Edit Sessions | ❌ | ❌ | ❌ | ✅ |
| Delete Sessions | ✅ | ✅ | ❌ | ✅ |
| Offline Access | No | No | No | No |
| Platform | Desktop | Any (web) | Terminal | Any (web) |

## Future Enhancements

- Session editing functionality
- Search and filter sessions
- Statistics and progress tracking
- Export sessions to PDF/Markdown
- Offline mode with sync
- Multi-language support
- Custom themes

## License

Part of the Katanaute project. See main repository for license information.

## Resources

- [CustomTkinter Documentation](https://customtkinter.tomschimansky.com/)
- [Katanaute API Documentation](../CLAUDE.md)
- [Python Requests Library](https://requests.readthedocs.io/)
