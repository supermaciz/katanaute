import { describe, it, expect, beforeEach, vi } from 'vitest'
import { screen, waitFor } from '@testing-library/react'
import userEvent from '@testing-library/user-event'
import LoginPage from './LoginPage'
import { renderWithRouter } from '../test/utils'
import { useAuth, type AuthContextType } from '../contexts/AuthContext'

const mockNavigate = vi.fn()

vi.mock('react-router-dom', async () => {
  const actual = await vi.importActual<typeof import('react-router-dom')>('react-router-dom')
  return {
    ...actual,
    useNavigate: () => mockNavigate,
  }
})

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

describe('LoginPage', () => {
  beforeEach(() => {
    vi.clearAllMocks()
    mockNavigate.mockReset()
  })

  it('submits credentials and navigates home on success', async () => {
    const user = userEvent.setup()
    const loginMock = vi.fn().mockResolvedValue(undefined)
    mockedUseAuth.mockReturnValue(createAuthValue({ login: loginMock }))

    renderWithRouter(<LoginPage />, { route: '/login' })

    await user.type(screen.getByLabelText(/Email address/i), 'dojo@example.com')
    await user.type(screen.getByLabelText(/Password/i), 'super-secret-password')

    await user.click(screen.getByRole('button', { name: /Sign in/i }))

    await waitFor(() => {
      expect(loginMock).toHaveBeenCalledWith({
        email: 'dojo@example.com',
        password: 'super-secret-password',
      })
      expect(mockNavigate).toHaveBeenCalledWith('/')
    })
  })

  it('shows error message when login fails', async () => {
    const user = userEvent.setup()
    const loginMock = vi.fn().mockRejectedValue(new Error('Invalid credentials'))
    mockedUseAuth.mockReturnValue(createAuthValue({ login: loginMock }))

    renderWithRouter(<LoginPage />, { route: '/login' })

    await user.type(screen.getByLabelText(/Email address/i), 'dojo@example.com')
    await user.type(screen.getByLabelText(/Password/i), 'wrong-password')

    await user.click(screen.getByRole('button', { name: /Sign in/i }))

    await waitFor(() => {
      expect(screen.getByText('Invalid credentials')).toBeInTheDocument()
    })
    expect(mockNavigate).not.toHaveBeenCalled()
  })

  it('disables the submit button while request is in flight', async () => {
    const user = userEvent.setup()
    const loginMock = vi.fn().mockReturnValue(new Promise<never>(() => {}))
    mockedUseAuth.mockReturnValue(createAuthValue({ login: loginMock }))

    renderWithRouter(<LoginPage />, { route: '/login' })

    await user.type(screen.getByLabelText(/Email address/i), 'dojo@example.com')
    await user.type(screen.getByLabelText(/Password/i), 'super-secret-password')

    const submitButton = screen.getByRole('button', { name: /Sign in/i })
    await user.click(submitButton)

    await waitFor(() => {
      expect(submitButton).toBeDisabled()
      expect(submitButton).toHaveTextContent('Signing in...')
    })
  })
})
