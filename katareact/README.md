# Katanaute React Frontend

A modern React frontend for the Katanaute kata training tracker application.

## Features

- View all practice sessions in a sortable table
- Create new training sessions with Markdown notes
- View detailed session information with rendered Markdown
- Delete sessions
- Responsive design with Tailwind CSS
- Integration with Phoenix backend API

## Tech Stack

- **React 18** - UI library
- **Vite** - Build tool and dev server
- **React Router** - Client-side routing
- **Tailwind CSS** - Utility-first CSS framework
- **React Markdown** - Markdown rendering for session notes
- **Vitest** - Fast unit testing framework
- **React Testing Library** - Component testing utilities

## Prerequisites

- Node.js 18+ or Bun
- Backend Phoenix server running on http://localhost:4000

## Installation

```bash
# Install dependencies
npm install
# or
bun install
```

## Development

```bash
# Start the development server
npm run dev
# or
bun run dev
```

The application will be available at http://localhost:3000

The dev server is configured to proxy API requests to the Phoenix backend at http://localhost:4000.

## Testing

The project includes a comprehensive test suite using Vitest and React Testing Library.

```bash
# Run tests in watch mode
npm test
# or
bun test

# Run tests with UI
npm run test:ui
# or
bun run test:ui

# Run tests with coverage
npm run test:coverage
# or
bun run test:coverage
```

### Test Coverage

- **API Client** - Full coverage of all API methods
- **SessionsPage** - Loading states, data display, delete functionality
- **NewSessionPage** - Form validation, submission, navigation
- **SessionDetailPage** - Data display, Markdown rendering, delete functionality

All tests use mocked API responses and follow React Testing Library best practices.

## Building for Production

```bash
# Build for production
npm run build
# or
bun run build

# Preview production build
npm run preview
# or
bun run preview
```

## Environment Variables

Create a `.env` file in the root directory to customize the API URL:

```
VITE_API_URL=http://localhost:4000/api
```

## Project Structure

```
katareact/
├── src/
│   ├── components/        # Reusable React components
│   ├── pages/            # Page components
│   │   ├── SessionsPage.jsx          # List all sessions
│   │   ├── SessionsPage.test.jsx     # Tests for sessions list
│   │   ├── NewSessionPage.jsx        # Create new session
│   │   ├── NewSessionPage.test.jsx   # Tests for session form
│   │   ├── SessionDetailPage.jsx     # View session details
│   │   └── SessionDetailPage.test.jsx # Tests for session detail
│   ├── services/         # API client and utilities
│   │   ├── api.js        # Backend API client
│   │   └── api.test.js   # API client tests
│   ├── test/             # Test utilities
│   │   ├── setup.js      # Vitest setup
│   │   └── utils.jsx     # Test helpers and mocks
│   ├── App.jsx           # Main app component with routing
│   ├── main.jsx          # Application entry point
│   └── index.css         # Global styles and Tailwind imports
├── public/               # Static assets
├── index.html           # HTML template
├── vite.config.js       # Vite configuration
├── tailwind.config.js   # Tailwind CSS configuration
└── package.json         # Project dependencies
```

## API Integration

The frontend consumes the following Phoenix API endpoints:

- `GET /api/sessions` - List all sessions
- `POST /api/sessions` - Create a new session
- `GET /api/sessions/:id` - Get session details
- `DELETE /api/sessions/:id` - Delete a session
- `GET /api/katas` - List all available katas
- `GET /api/katas/:id` - Get kata details

## Features in Detail

### Sessions List
- Displays all training sessions in a table format
- Shows kata name, level (Yellow to Shodan), practice date, and course status
- Click on a session to view details
- Delete sessions directly from the list

### New Session Form
- Select from available katas
- Set practice date and time
- Mark as part of structured learning path (In Course)
- Add notes with Markdown support
- Real-time form validation

### Session Detail View
- View full session information
- Rendered Markdown notes with proper formatting
- Kata level badges
- Delete session option

## Development Notes

- The project uses ESLint for code quality
- Tailwind CSS is configured for responsive design
- API proxy is configured in Vite for development
- All datetime handling uses ISO 8601 format

## License

Part of the Katanaute project.
