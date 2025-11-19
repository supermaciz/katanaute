import { describe, it, expect, beforeEach, vi } from 'vitest'
import { screen, waitFor } from '@testing-library/react'
import userEvent from '@testing-library/user-event'
import RegisterPage from './RegisterPage'
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

describe('RegisterPage', () => {
  beforeEach(() => {
    vi.clearAllMocks()
    mockNavigate.mockReset()
  })

  it('validates password confirmation before calling API', async () => {
    const user = userEvent.setup()
    const registerMock = vi.fn()
    mockedUseAuth.mockReturnValue(createAuthValue({ register: registerMock }))

    renderWithRouter(<RegisterPage />, { route: '/register' })

    await user.type(screen.getByLabelText(/Email address/i), 'dojo@example.com')
    await user.type(screen.getByLabelText('Password'), 'super-secure-password')
    await user.type(screen.getByLabelText(/Confirm Password/i), 'different-password')

    await user.click(screen.getByRole('button', { name: /Create account/i }))

    expect(screen.getByText('Passwords do not match')).toBeInTheDocument()
    expect(registerMock).not.toHaveBeenCalled()
  })

  it('requires strong passwords', async () => {
    const user = userEvent.setup()
    const registerMock = vi.fn()
    mockedUseAuth.mockReturnValue(createAuthValue({ register: registerMock }))

    renderWithRouter(<RegisterPage />, { route: '/register' })

    await user.type(screen.getByLabelText(/Email address/i), 'dojo@example.com')
    await user.type(screen.getByLabelText('Password'), 'short')
    await user.type(screen.getByLabelText(/Confirm Password/i), 'short')

    await user.click(screen.getByRole('button', { name: /Create account/i }))

    expect(screen.getByText('Password must be at least 12 characters')).toBeInTheDocument()
    expect(registerMock).not.toHaveBeenCalled()
  })

  it('submits registration data and navigates home on success', async () => {
    const user = userEvent.setup()
    const registerMock = vi.fn().mockResolvedValue(undefined)
    mockedUseAuth.mockReturnValue(createAuthValue({ register: registerMock }))

    renderWithRouter(<RegisterPage />, { route: '/register' })

    await user.type(screen.getByLabelText(/Email address/i), 'dojo@example.com')
    await user.type(screen.getByLabelText('Password'), 'super-secure-password')
    await user.type(screen.getByLabelText(/Confirm Password/i), 'super-secure-password')

    await user.click(screen.getByRole('button', { name: /Create account/i }))

    await waitFor(() => {
      expect(registerMock).toHaveBeenCalledWith({
        email: 'dojo@example.com',
        password: 'super-secure-password',
      })
      expect(mockNavigate).toHaveBeenCalledWith('/')
    })
  })

  it('surfaces API errors when registration fails', async () => {
    const user = userEvent.setup()
    const registerMock = vi.fn().mockRejectedValue(new Error('Email already taken'))
    mockedUseAuth.mockReturnValue(createAuthValue({ register: registerMock }))

    renderWithRouter(<RegisterPage />, { route: '/register' })

    await user.type(screen.getByLabelText(/Email address/i), 'dojo@example.com')
    await user.type(screen.getByLabelText('Password'), 'super-secure-password')
    await user.type(screen.getByLabelText(/Confirm Password/i), 'super-secure-password')

    await user.click(screen.getByRole('button', { name: /Create account/i }))

    await waitFor(() => {
      expect(screen.getByText('Email already taken')).toBeInTheDocument()
    })
    expect(mockNavigate).not.toHaveBeenCalled()
  })

  it('disables submit button while request is pending', async () => {
    const user = userEvent.setup()
    const registerMock = vi.fn().mockReturnValue(new Promise<never>(() => {}))
    mockedUseAuth.mockReturnValue(createAuthValue({ register: registerMock }))

    renderWithRouter(<RegisterPage />, { route: '/register' })

    await user.type(screen.getByLabelText(/Email address/i), 'dojo@example.com')
    await user.type(screen.getByLabelText('Password'), 'super-secure-password')
    await user.type(screen.getByLabelText(/Confirm Password/i), 'super-secure-password')

    const submitButton = screen.getByRole('button', { name: /Create account/i })
    await user.click(submitButton)

    await waitFor(() => {
      expect(submitButton).toBeDisabled()
      expect(submitButton).toHaveTextContent('Creating account...')
    })
  })
})
