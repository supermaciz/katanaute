import { describe, it, expect, beforeEach, vi } from 'vitest'
import { render, screen } from '@testing-library/react'
import { MemoryRouter, Routes, Route } from 'react-router-dom'
import ProtectedRoute from './ProtectedRoute'
import { useAuth, type AuthContextType } from '../contexts/AuthContext'

vi.mock('../contexts/AuthContext', async () => {
  const actual = await vi.importActual<typeof import('../contexts/AuthContext')>('../contexts/AuthContext')
  return {
    ...actual,
    useAuth: vi.fn(),
  }
})

const mockedUseAuth = vi.mocked(useAuth)

const createAuthValue = (overrides: Partial<AuthContextType> = {}): AuthContextType => ({
  user: null,
  token: null,
  login: vi.fn(),
  register: vi.fn(),
  logout: vi.fn(),
  isAuthenticated: false,
  isLoading: false,
  ...overrides,
})

const renderProtectedRoute = () =>
  render(
    <MemoryRouter
      initialEntries={['/sessions']}
      future={{ v7_startTransition: true, v7_relativeSplatPath: true }}
    >
      <Routes>
        <Route path="/login" element={<div>Login Page</div>} />
        <Route
          path="/sessions"
          element={
            <ProtectedRoute>
              <div>Private Content</div>
            </ProtectedRoute>
          }
        />
      </Routes>
    </MemoryRouter>
  )

describe('ProtectedRoute', () => {
  beforeEach(() => {
    vi.clearAllMocks()
  })

  it('renders loading indicator while auth is resolving', () => {
    mockedUseAuth.mockReturnValue(createAuthValue({ isLoading: true }))

    renderProtectedRoute()

    expect(screen.getByText('Loading...')).toBeInTheDocument()
  })

  it('redirects unauthenticated users to the login page', () => {
    mockedUseAuth.mockReturnValue(createAuthValue({ isLoading: false, isAuthenticated: false }))

    renderProtectedRoute()

    expect(screen.getByText('Login Page')).toBeInTheDocument()
  })

  it('renders children when user is authenticated', () => {
    mockedUseAuth.mockReturnValue(createAuthValue({ isLoading: false, isAuthenticated: true }))

    renderProtectedRoute()

    expect(screen.getByText('Private Content')).toBeInTheDocument()
  })
})
