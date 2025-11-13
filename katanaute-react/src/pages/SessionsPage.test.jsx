import { describe, it, expect, beforeEach, vi } from 'vitest'
import { screen, waitFor } from '@testing-library/react'
import userEvent from '@testing-library/user-event'
import { renderWithRouter, mockSessions, mockKatas } from '../test/utils'
import SessionsPage from './SessionsPage'
import { api } from '../services/api'

vi.mock('../services/api')

describe('SessionsPage', () => {
  beforeEach(() => {
    vi.clearAllMocks()
  })

  it('renders loading state initially', () => {
    api.getSessions.mockReturnValue(new Promise(() => {}))
    api.getKatas.mockReturnValue(new Promise(() => {}))

    renderWithRouter(<SessionsPage />)

    expect(screen.getByText('Loading sessions...')).toBeInTheDocument()
  })

  it('displays sessions in a table after loading', async () => {
    api.getSessions.mockResolvedValue({ data: mockSessions })
    api.getKatas.mockResolvedValue({ data: mockKatas })

    renderWithRouter(<SessionsPage />)

    await waitFor(() => {
      expect(screen.getByText('FizzBuzz')).toBeInTheDocument()
      expect(screen.getByText('Roman Numerals')).toBeInTheDocument()
    })

    expect(screen.getByText('Yellow')).toBeInTheDocument()
    expect(screen.getByText('Orange')).toBeInTheDocument()
  })

  it('shows empty state when no sessions exist', async () => {
    api.getSessions.mockResolvedValue({ data: [] })
    api.getKatas.mockResolvedValue({ data: mockKatas })

    renderWithRouter(<SessionsPage />)

    await waitFor(() => {
      expect(screen.getByText('No sessions yet')).toBeInTheDocument()
    })

    expect(screen.getByText('Create your first session')).toBeInTheDocument()
  })

  it('displays error message when API fails', async () => {
    api.getSessions.mockRejectedValue(new Error('Network error'))
    api.getKatas.mockResolvedValue({ data: mockKatas })

    renderWithRouter(<SessionsPage />)

    await waitFor(() => {
      expect(screen.getByText(/Error: Network error/i)).toBeInTheDocument()
    })
  })

  it('displays kata levels correctly with proper names', async () => {
    api.getSessions.mockResolvedValue({ data: mockSessions })
    api.getKatas.mockResolvedValue({ data: mockKatas })

    renderWithRouter(<SessionsPage />)

    await waitFor(() => {
      expect(screen.getByText('Yellow')).toBeInTheDocument()
      expect(screen.getByText('Orange')).toBeInTheDocument()
    })

    // Should NOT show N/A for valid kata levels
    expect(screen.queryByText('N/A')).not.toBeInTheDocument()
  })

  it('applies correct color classes to kata level badges', async () => {
    api.getSessions.mockResolvedValue({ data: mockSessions })
    api.getKatas.mockResolvedValue({ data: mockKatas })

    renderWithRouter(<SessionsPage />)

    await waitFor(() => {
      const yellowBadge = screen.getByText('Yellow')
      expect(yellowBadge).toHaveClass('bg-yellow-100', 'text-yellow-800')

      const orangeBadge = screen.getByText('Orange')
      expect(orangeBadge).toHaveClass('bg-orange-100', 'text-orange-800')
    })
  })

  it('shows "In Course" badge for in-course sessions', async () => {
    api.getSessions.mockResolvedValue({ data: mockSessions })
    api.getKatas.mockResolvedValue({ data: mockKatas })

    renderWithRouter(<SessionsPage />)

    await waitFor(() => {
      const yesBadges = screen.getAllByText('Yes')
      expect(yesBadges.length).toBeGreaterThan(0)
    })
  })

  it('sorts sessions by practice date in descending order (newest first)', async () => {
    const unsortedSessions = [
      {
        id: 1,
        kata_id: 1,
        practiced_at: '2025-01-10T10:00:00Z',
        in_course: true,
        notes: 'Oldest session',
      },
      {
        id: 3,
        kata_id: 2,
        practiced_at: '2025-01-15T14:30:00Z',
        in_course: false,
        notes: 'Newest session',
      },
      {
        id: 2,
        kata_id: 1,
        practiced_at: '2025-01-12T11:00:00Z',
        in_course: false,
        notes: 'Middle session',
      },
    ]

    api.getSessions.mockResolvedValue({ data: unsortedSessions })
    api.getKatas.mockResolvedValue({ data: mockKatas })

    renderWithRouter(<SessionsPage />)

    await waitFor(() => {
      expect(screen.getByText('Roman Numerals')).toBeInTheDocument()
    })

    // Get all kata name links in order they appear in the table
    const kataLinks = screen.getAllByRole('link', { name: /FizzBuzz|Roman Numerals/i })

    // First row should be the newest session (id: 3, kata_id: 2 = Roman Numerals)
    // Second row should be middle session (id: 2, kata_id: 1 = FizzBuzz)
    // Third row should be oldest session (id: 1, kata_id: 1 = FizzBuzz)
    expect(kataLinks[0]).toHaveTextContent('Roman Numerals')
    expect(kataLinks[1]).toHaveTextContent('FizzBuzz')
    expect(kataLinks[2]).toHaveTextContent('FizzBuzz')
  })

  it('deletes a session when delete button is clicked and confirmed', async () => {
    const user = userEvent.setup()
    api.getSessions.mockResolvedValue({ data: mockSessions })
    api.getKatas.mockResolvedValue({ data: mockKatas })
    api.deleteSession.mockResolvedValue(null)

    // Mock window.confirm
    window.confirm = vi.fn(() => true)

    renderWithRouter(<SessionsPage />)

    await waitFor(() => {
      expect(screen.getByText('Roman Numerals')).toBeInTheDocument()
    })

    const deleteButtons = screen.getAllByText('Delete')
    await user.click(deleteButtons[0])

    expect(window.confirm).toHaveBeenCalledWith('Are you sure you want to delete this session?')
    // After sorting by date desc, session 2 (newest) is first in the list
    expect(api.deleteSession).toHaveBeenCalledWith(2)
  })

  it('does not delete session when user cancels', async () => {
    const user = userEvent.setup()
    api.getSessions.mockResolvedValue({ data: mockSessions })
    api.getKatas.mockResolvedValue({ data: mockKatas })

    window.confirm = vi.fn(() => false)

    renderWithRouter(<SessionsPage />)

    await waitFor(() => {
      expect(screen.getByText('FizzBuzz')).toBeInTheDocument()
    })

    const deleteButtons = screen.getAllByText('Delete')
    await user.click(deleteButtons[0])

    expect(api.deleteSession).not.toHaveBeenCalled()
  })

  it('renders links to session detail pages', async () => {
    api.getSessions.mockResolvedValue({ data: mockSessions })
    api.getKatas.mockResolvedValue({ data: mockKatas })

    renderWithRouter(<SessionsPage />)

    await waitFor(() => {
      const link = screen.getByRole('link', { name: 'FizzBuzz' })
      expect(link).toHaveAttribute('href', '/sessions/1')
    })
  })

  it('renders "New Session" button', async () => {
    api.getSessions.mockResolvedValue({ data: mockSessions })
    api.getKatas.mockResolvedValue({ data: mockKatas })

    renderWithRouter(<SessionsPage />)

    await waitFor(() => {
      const newSessionButtons = screen.getAllByText('New Session')
      expect(newSessionButtons.length).toBeGreaterThan(0)
    })
  })
})
