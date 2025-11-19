# Katanaute React Frontend

A modern React SPA for the Katanaute kata training tracker, served by Phoenix in production and with standalone Vite dev server for development.

## Deployment Modes

### Production (Served by Phoenix)

In production, this React app is built and served by the Phoenix backend:

```bash
# From the katanaute/ directory:
cd ../katanaute
mix react.build        # Build React and copy to priv/static/react/
mix phx.server         # Serve at http://localhost:4000/
```

The React UI is accessible at the root path `/` while the LiveView admin is at `/admin`.

### Development (Standalone Vite Server)

For development with hot reload:

```bash
npm install            # Install dependencies
npm run dev           # Start dev server on http://localhost:3000
```

The dev server includes:
- Hot module replacement (HMR) for instant updates
- API proxy to `http://localhost:4000`
- Full TypeScript type checking

## Features

- User authentication with email/password login
- View all practice sessions in a sortable table
- Create new training sessions with Markdown notes
- View detailed session information with rendered Markdown
- Delete sessions
- Responsive design with Tailwind CSS
- Bearer token authentication with Phoenix backend API
- Served by Phoenix at `/` in production

## Tech Stack

- **React 18** - UI library with TypeScript
- **Vite** - Build tool and dev server
- **React Router** - Client-side routing
- **Tailwind CSS v3** - Utility-first CSS framework
- **React Markdown** - Markdown rendering for session notes
- **Vitest** - Fast unit testing framework
- **React Testing Library** - Component testing utilities

## Prerequisites

- Node.js 18+ or Bun
- Backend Phoenix server running on http://localhost:4000

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

## Building

### For Phoenix Deployment

Use the Mix task from the Phoenix directory:

```bash
cd ../katanaute
mix react.build        # Builds and copies to priv/static/react/
```

This task:
1. Runs `npm install` to ensure dependencies are current
2. Runs `npm run build` (TypeScript compile + Vite build)
3. Copies `dist/*` to `../katanaute/priv/static/react/`

### Local Build (for testing)

```bash
npm run build         # Build to dist/ directory
npm run preview       # Preview production build locally
```

## Configuration

### Vite Base Path

The `base` configuration in `vite.config.ts` is set to `/react/` to match the Phoenix static path:

```typescript
export default defineConfig({
  base: '/react/',  // Assets served from /react/assets/
  // ...
})
```

This ensures that in production, asset references point to `/react/assets/` where Phoenix serves them.

### Environment Variables

Optional `.env` file to customize the API URL:

```
VITE_API_URL=http://localhost:4000/api
```

In development, the Vite proxy handles `/api` requests automatically.

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

**Authentication** (public):
- `POST /api/auth/register` - Register new user
- `POST /api/auth/token` - Login with email/password
- `DELETE /api/auth/token` - Logout
- `GET /api/auth/me` - Get current user

**Sessions** (requires auth):
- `GET /api/sessions` - List all sessions
- `POST /api/sessions` - Create a new session
- `GET /api/sessions/:id` - Get session details
- `DELETE /api/sessions/:id` - Delete a session

**Katas** (public):
- `GET /api/katas` - List all available katas
- `GET /api/katas/:id` - Get kata details

All authenticated requests include the Bearer token in the `Authorization` header.

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
- Tailwind CSS v3 is configured for responsive design
- API proxy is configured in Vite for development (`/api` → `http://localhost:4000`)
- All datetime handling uses ISO 8601 format
- Production build uses `/react/` base path to match Phoenix static serving
- Client-side routing handled by React Router (Phoenix serves index.html for all non-API routes)

## Architecture

### Production Routing

When served by Phoenix:
1. Phoenix serves `index.html` from `priv/static/react/` for all non-API, non-admin routes
2. React Router handles client-side navigation
3. Assets are loaded from `/react/assets/` (served by Phoenix static plug)
4. API calls go to `/api` (handled by Phoenix API routes)

### Development Routing

When using Vite dev server:
1. Vite serves the app on port 3000
2. API calls to `/api` are proxied to `http://localhost:4000`
3. Hot module replacement provides instant feedback
4. No Phoenix static serving involved

## License

Part of the Katanaute project.
