# Katafyne - Simple Desktop App for Kata Training

Track your karate training sessions with a clean, modern desktop interface. Built with Go and Fyne for a native feel on all platforms.

## What Can It Do?

- 📋 **View Your Sessions** - See all your training history in one place
- ➕ **Add New Sessions** - Record what you practiced today
- 🥋 **Browse Katas** - See all available katas with colored belt levels
- 🔐 **Secure Login** - Easy authentication that works offline after first login
- 💾 **Remember You** - Stays logged in between uses
- 🎨 **Clean Interface** - Simple split-screen layout that's easy to navigate

## What It Looks Like

**When you start the app for the first time:**
- You'll see a simple login screen
- Click "Login" and you'll get a code
- Open that website in your browser, enter the code, and you're in!

**After you're logged in:**
- **Left side**: List of all your training sessions (newest first)
- **Right side**: Details of the session you clicked on
- **Top buttons**: Refresh list, Add new session, Logout

## How to Get Started

### Step 1: Install What You Need

**On Linux**, you'll need some system packages first:

```bash
# Ubuntu/Debian
sudo apt-get install gcc libgl1-mesa-dev xorg-dev

# Fedora
sudo dnf install gcc libXcursor-devel libXrandr-devel mesa-libGL-devel libXi-devel

# Arch
sudo pacman -S go gcc libxcursor libxrandr libxinerama libxi mesa
```

**On macOS** or **Windows**, you don't need anything extra!

### Step 2: Build the App

```bash
cd katafyne
go get fyne.io/fyne/v2
go mod tidy
go build
```

### Step 3: Run It

```bash
./katafyne
```

That's it! The app will open up ready to use.

## Connecting to Your Server

**Default**: Katafyne assumes your backend server is running on your computer at `http://localhost:4000/api`.

**Using a different server?** Just set this before running the app:

```bash
export KATANAUTE_API_URL=https://your-server.com/api
./katafyne
```

### Where Your Login Info is Saved

After you log in, Katafyne remembers you by saving a token here:
- **Linux**: `~/.config/katanaute/config.json`
- **macOS**: `~/Library/Application Support/katanaute/config.json`
- **Windows**: `%APPDATA%\katanaute\config.json`

Don't worry - the token is secure and you can delete it anytime by logging out.

## How to Use It

### Your First Time

1. Click the **"Login"** button
2. You'll see a website address and a code (like "ABCD-1234")
3. Open that website in your browser
4. Type in the code
5. Click "Approve"
6. Come back to the app - you're logged in!

(You only need to do this once. After that, the app remembers you.)

### Looking at Your Training History

- All your sessions are on the **left side**, newest first
- **Click on any session** to see the details on the right
- You'll see:
  - Which kata you practiced
  - What belt level it is (shown in color)
  - When you did it
  - Any notes you wrote
  - If it was part of a structured course (look for the 📚 icon)

### Recording a New Training Session

1. Click **"Add Session"** at the top
2. Pick which kata you practiced from the list
3. Type any notes about the session (you can use Markdown formatting if you want)
4. Check the box if this was part of your structured training course
5. Click **"Create"**

Done! Your new session appears in the list.

### Logging Out

Click **"Logout"** at the top. This removes your saved login and takes you back to the login screen.

## Something Not Working?

### The app won't start

**On Linux**: You might be missing some system packages. Scroll up to "Step 1: Install What You Need" and make sure you installed everything.

**On any system**: Try rebuilding:
```bash
go clean
go mod tidy
go build
```

### Can't log in

- Make sure the backend server is running (usually `mix phx.server` in the katanaute folder)
- Check you typed the code correctly (it's case-sensitive!)
- Make sure you clicked "Approve" on the website

### Not seeing your sessions

- Check if the backend server is running
- Try logging out and logging back in
- Make sure you're connecting to the right server (see "Connecting to Your Server" above)

### Still stuck?

Open an issue on GitHub with what went wrong. We're here to help!

## Architecture

Katafyne follows a simple MVC-like architecture:

```
main.go     - Application entry point and UI logic
```

All the heavy lifting (authentication, configuration, API calls) is handled by the shared **katagocore** library.

### Device Flow Sequence

1. **Initiate**: `POST /api/auth/device/code` - Get a user code
2. **Display**: Show the code and website to the user
3. **Poll**: `POST /api/auth/device/token` - Check if user approved (every 5 seconds)
4. **Store**: Save the access token for future use
5. **Use**: Include token in all API requests

## Comparison with Other Clients

| Feature | Katafyne (Go + Fyne) | Katarouille (Rust + Iced) | Katago (Go TUI) |
|---------|---------------------|--------------------------|-----------------|
| Platform | Desktop GUI | Desktop GUI | Terminal |
| Language | Go | Rust | Go |
| Auth | Device Flow | Device Flow | Device Flow |
| View Sessions | ✅ | ✅ | ✅ |
| Create Sessions | ✅ | ✅ | ✅ |
| Edit Sessions | ❌ | ❌ | ❌ |
| Delete Sessions | ❌ | ❌ | ❌ |
| Offline Mode | ❌ | ✅ | ❌ |

**When to use Katafyne**:
- You want a simple, clean GUI
- You prefer Go over Rust
- You don't need offline support
- You want the easiest setup

## For Developers

Want to contribute or customize? Check out:
- **[CLAUDE.md](./CLAUDE.md)** - Development guidelines
- **[katagocore/](../katagocore/)** - Shared Go library documentation

## License

Part of the Katanaute project. Use it however you want - it's a learning project!
