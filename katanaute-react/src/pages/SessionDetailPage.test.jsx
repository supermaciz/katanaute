import { describe, it, expect, beforeEach, vi } from 'vitest'
import { screen, waitFor } from '@testing-library/react'
import userEvent from '@testing-library/user-event'
import { renderWithRouter } from '../test/utils'
import SessionDetailPage from './SessionDetailPage'
import { api } from '../services/api'

const mockNavigate = vi.fn()
vi.mock('react-router-dom', async () => {
  const actual = await vi.importActual('react-router-dom')
  return {
    ...actual,
    useNavigate: () => mockNavigate,
    useParams: () => ({ id: '1' }),
  }
})

vi.mock('../services/api')

describe('SessionDetailPage', () => {
  const mockSession = {
    id: 1,
    kata_id: 1,
    practiced_at: '2025-01-10T10:00:00Z',
    in_course: true,
    notes: '# Test Notes\n\nThis is a **test** session with `code`.',
  }

  const mockKata = {
    id: 1,
    name: 'FizzBuzz',
    level: 1,
  }

  beforeEach(() => {
    vi.clearAllMocks()
    mockNavigate.mockClear()
  })

  it('renders loading state initially', () => {
    api.getSession.mockReturnValue(new Promise(() => {}))

    renderWithRouter(<SessionDetailPage />)

    expect(screen.getByText('Loading session...')).toBeInTheDocument()
  })

  it('displays session details after loading', async () => {
    api.getSession.mockResolvedValue({ data: mockSession })
    api.getKata.mockResolvedValue({ data: mockKata })

    renderWithRouter(<SessionDetailPage />)

    await waitFor(() => {
      expect(screen.getByText('FizzBuzz')).toBeInTheDocument()
    })

    expect(screen.getByText('Yellow')).toBeInTheDocument()
    expect(screen.getByText('In Course')).toBeInTheDocument()
    expect(screen.getByText('Notes')).toBeInTheDocument()
  })

  it('renders markdown notes correctly', async () => {
    api.getSession.mockResolvedValue({ data: mockSession })
    api.getKata.mockResolvedValue({ data: mockKata })

    renderWithRouter(<SessionDetailPage />)

    await waitFor(() => {
      expect(screen.getByText('Test Notes')).toBeInTheDocument()
    })

    // ReactMarkdown should render the markdown
    const boldText = screen.getByText('test')
    expect(boldText.tagName).toBe('STRONG')

    const codeText = screen.getByText('code')
    expect(codeText.tagName).toBe('CODE')
  })

  it('does not show "In Course" badge for non-course sessions', async () => {
    const nonCourseSession = { ...mockSession, in_course: false }
    api.getSession.mockResolvedValue({ data: nonCourseSession })
    api.getKata.mockResolvedValue({ data: mockKata })

    renderWithRouter(<SessionDetailPage />)

    await waitFor(() => {
      expect(screen.getByText('FizzBuzz')).toBeInTheDocument()
    })

    expect(screen.queryByText('In Course')).not.toBeInTheDocument()
  })

  it('displays error message when API fails', async () => {
    api.getSession.mockRejectedValue(new Error('Session not found'))

    renderWithRouter(<SessionDetailPage />)

    await waitFor(() => {
      expect(screen.getByText(/Error: Session not found/i)).toBeInTheDocument()
    })
  })

  it('shows session not found message when session is null', async () => {
    api.getSession.mockResolvedValue({ data: null })

    renderWithRouter(<SessionDetailPage />)

    await waitFor(() => {
      expect(screen.getByText('Session not found')).toBeInTheDocument()
    })
  })

  it('deletes session and navigates when delete button is clicked', async () => {
    const user = userEvent.setup()
    api.getSession.mockResolvedValue({ data: mockSession })
    api.getKata.mockResolvedValue({ data: mockKata })
    api.deleteSession.mockResolvedValue(null)

    window.confirm = vi.fn(() => true)

    renderWithRouter(<SessionDetailPage />)

    await waitFor(() => {
      expect(screen.getByText('FizzBuzz')).toBeInTheDocument()
    })

    const deleteButton = screen.getByText('Delete')
    await user.click(deleteButton)

    expect(window.confirm).toHaveBeenCalledWith('Are you sure you want to delete this session?')
    expect(api.deleteSession).toHaveBeenCalledWith('1')
    expect(mockNavigate).toHaveBeenCalledWith('/')
  })

  it('does not delete when user cancels confirmation', async () => {
    const user = userEvent.setup()
    api.getSession.mockResolvedValue({ data: mockSession })
    api.getKata.mockResolvedValue({ data: mockKata })

    window.confirm = vi.fn(() => false)

    renderWithRouter(<SessionDetailPage />)

    await waitFor(() => {
      expect(screen.getByText('FizzBuzz')).toBeInTheDocument()
    })

    const deleteButton = screen.getByText('Delete')
    await user.click(deleteButton)

    expect(api.deleteSession).not.toHaveBeenCalled()
    expect(mockNavigate).not.toHaveBeenCalled()
  })

  it('shows alert when delete fails', async () => {
    const user = userEvent.setup()
    api.getSession.mockResolvedValue({ data: mockSession })
    api.getKata.mockResolvedValue({ data: mockKata })
    api.deleteSession.mockRejectedValue(new Error('Delete failed'))

    window.confirm = vi.fn(() => true)
    window.alert = vi.fn()

    renderWithRouter(<SessionDetailPage />)

    await waitFor(() => {
      expect(screen.getByText('FizzBuzz')).toBeInTheDocument()
    })

    const deleteButton = screen.getByText('Delete')
    await user.click(deleteButton)

    await waitFor(() => {
      expect(window.alert).toHaveBeenCalledWith('Failed to delete session: Delete failed')
    })

    expect(mockNavigate).not.toHaveBeenCalled()
  })

  it('renders back to sessions link', async () => {
    api.getSession.mockResolvedValue({ data: mockSession })
    api.getKata.mockResolvedValue({ data: mockKata })

    renderWithRouter(<SessionDetailPage />)

    await waitFor(() => {
      const link = screen.getByText('← Back to Sessions')
      expect(link).toHaveAttribute('href', '/')
    })
  })

  it('displays session ID', async () => {
    api.getSession.mockResolvedValue({ data: mockSession })
    api.getKata.mockResolvedValue({ data: mockKata })

    renderWithRouter(<SessionDetailPage />)

    await waitFor(() => {
      expect(screen.getByText('1')).toBeInTheDocument()
    })
  })

  it('does not render notes section when notes are empty', async () => {
    const sessionWithoutNotes = { ...mockSession, notes: '' }
    api.getSession.mockResolvedValue({ data: sessionWithoutNotes })
    api.getKata.mockResolvedValue({ data: mockKata })

    renderWithRouter(<SessionDetailPage />)

    await waitFor(() => {
      expect(screen.getByText('FizzBuzz')).toBeInTheDocument()
    })

    expect(screen.queryByText('Notes')).not.toBeInTheDocument()
  })

  it('formats the practiced_at date correctly', async () => {
    api.getSession.mockResolvedValue({ data: mockSession })
    api.getKata.mockResolvedValue({ data: mockKata })

    renderWithRouter(<SessionDetailPage />)

    await waitFor(() => {
      // The date should be formatted as a readable string
      expect(screen.getByText(/January 10, 2025/i)).toBeInTheDocument()
    })
  })
})
