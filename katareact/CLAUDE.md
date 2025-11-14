This is a React single-page application built with Vite.

## Project guidelines

- Use conventional commits with `react` scope for all commits (e.g., `feat(react):`, `fix(react):`, `test(react):`)
- Run `npm test` before committing to ensure all tests pass
- Use existing utility functions from `src/utils/` when available
- All components should have corresponding test files

### React & Vite guidelines

- This project uses **React 18** with functional components and hooks
- **Always** use React hooks (useState, useEffect, etc.) for state management
- **Never** use class components
- Vite is the build tool - configuration is in `vite.config.js`
- API proxy is configured in Vite to forward `/api` requests to `http://localhost:4000`

### Testing guidelines

- This project uses **Vitest** and **React Testing Library**
- Test setup is in `src/test/setup.js`
- Shared test utilities and mocks are in `src/test/utils.jsx`
- **Always** write tests for new components and features
- Use `renderWithRouter` helper from `src/test/utils.jsx` for components using React Router
- Mock API calls using `vi.mock('../services/api')`
- Test user interactions with `@testing-library/user-event`
- **Always** use `waitFor` for async assertions
- Follow React Testing Library best practices: test behavior, not implementation

### Styling guidelines

- This project uses **Tailwind CSS v3**
- **Always** use Tailwind utility classes for styling
- Tailwind config is in `tailwind.config.js`
- Global styles are in `src/index.css` with Tailwind imports
- **Never** write inline styles unless absolutely necessary
- Use responsive classes (`sm:`, `md:`, `lg:`, etc.) for mobile-first design
- Color-coded kata levels use specific Tailwind classes (see `src/utils/kataLevels.js`)

### API integration guidelines

- All API calls go through `src/services/api.js`
- The backend is a Phoenix application running on `http://localhost:4000`
- API responses follow the format: `{ data: [...] }`
- **Always** handle loading states while fetching data
- **Always** handle error states and display error messages to users
- Use `try/catch` blocks for all API calls
- Kata levels from backend are atom strings (`"yellow"`, `"orange"`, etc.), not integers

### Code organization

- **Pages** (`src/pages/`): Top-level route components
- **Components** (`src/components/`): Reusable UI components
- **Services** (`src/services/`): API clients and external service integrations
- **Utils** (`src/utils/`): Utility functions and constants
- **Test utilities** (`src/test/`): Shared test helpers and setup

### Component guidelines

- Use descriptive component names (e.g., `SessionsPage`, not `Sessions`)
- Export components as default exports
- Keep components focused on a single responsibility
- Extract reusable logic into custom hooks if needed
- Use PropTypes or TypeScript for type checking if needed

### React Router guidelines

- This project uses **React Router v6**
- **Always** use `Link` component for navigation, never `<a>` tags for internal routes
- Use `useNavigate` hook for programmatic navigation
- Use `useParams` hook to access URL parameters
- Route definitions are in `src/App.jsx`

### Form handling guidelines

- Use controlled components with `useState` for form inputs
- **Always** use `onChange` handlers to update state
- Use `onSubmit` on forms, prevent default with `e.preventDefault()`
- Validate form data before submission
- Show loading state while form is submitting
- Display error messages when submission fails

### Data handling guidelines

- Sessions are sorted by `practiced_at` date in descending order (newest first)
- Use JavaScript `Date` objects for date comparisons
- Format dates using `toLocaleString()` or similar for display
- Kata levels have specific color mappings in `src/utils/kataLevels.js`
- Use helper functions `getKataLevelName()` and `getKataLevelColor()` for consistent display

### Best practices

- Keep components small and focused
- Avoid prop drilling - consider React Context for deeply nested state
- Use semantic HTML elements
- Ensure accessibility (ARIA labels, keyboard navigation, etc.)
- Handle edge cases (empty states, loading states, error states)
- Clean up side effects in `useEffect` cleanup functions
- Avoid unnecessary re-renders by using `useMemo` and `useCallback` when appropriate
