import { describe, it, expect, beforeEach, vi } from 'vitest'
import { screen, waitFor } from '@testing-library/react'
import userEvent from '@testing-library/user-event'
import { renderWithRouter, mockKatas } from '../test/utils'
import NewSessionPage from './NewSessionPage'
import { api } from '../services/api'

// Mock the useNavigate hook
const mockNavigate = vi.fn()
vi.mock('react-router-dom', async () => {
  const actual = await vi.importActual('react-router-dom')
  return {
    ...actual,
    useNavigate: () => mockNavigate,
  }
})

vi.mock('../services/api')

describe('NewSessionPage', () => {
  beforeEach(() => {
    vi.clearAllMocks()
    mockNavigate.mockClear()
  })

  it('renders loading state initially', () => {
    ;(api.getKatas as any).mockReturnValue(new Promise(() => {}))

    renderWithRouter(<NewSessionPage />)

    expect(screen.getByText('Loading...')).toBeInTheDocument()
  })

  it('displays the form after loading katas', async () => {
    ;(api.getKatas as any).mockResolvedValue({ data: mockKatas })

    renderWithRouter(<NewSessionPage />)

    await waitFor(() => {
      expect(screen.getByText('New Training Session')).toBeInTheDocument()
    })

    expect(screen.getByLabelText('Kata')).toBeInTheDocument()
    expect(screen.getByLabelText('Practiced At')).toBeInTheDocument()
    expect(screen.getByLabelText(/Part of structured learning path/i)).toBeInTheDocument()
    expect(screen.getByLabelText(/Notes/i)).toBeInTheDocument()
  })

  it('displays error message when API fails', async () => {
    ;(api.getKatas as any).mockRejectedValue(new Error('Failed to load katas'))

    renderWithRouter(<NewSessionPage />)

    await waitFor(() => {
      expect(screen.getByText(/Error: Failed to load katas/i)).toBeInTheDocument()
    })
  })

  it('populates kata dropdown with available katas', async () => {
    ;(api.getKatas as any).mockResolvedValue({ data: mockKatas })

    renderWithRouter(<NewSessionPage />)

    await waitFor(() => {
      expect(screen.getByText('FizzBuzz (Yellow)')).toBeInTheDocument()
    })

    expect(screen.getByText('Roman Numerals (Orange)')).toBeInTheDocument()
    expect(screen.getByText('Bowling Game (Blue)')).toBeInTheDocument()
  })

  it('submits form with correct data', async () => {
    const user = userEvent.setup()
    ;(api.getKatas as any).mockResolvedValue({ data: mockKatas })
    ;(api.createSession as any).mockResolvedValue({ data: { id: 1 } })

    renderWithRouter(<NewSessionPage />)

    await waitFor(() => {
      expect(screen.getByLabelText('Kata')).toBeInTheDocument()
    })

    // Fill in the form
    const kataSelect = screen.getByLabelText('Kata')
    await user.selectOptions(kataSelect, '2')

    const notesTextarea = screen.getByLabelText(/Notes/i)
    await user.type(notesTextarea, 'Test notes')

    const inCourseCheckbox = screen.getByLabelText(/Part of structured learning path/i)
    await user.click(inCourseCheckbox)

    // Submit the form
    const submitButton = screen.getByText('Create Session')
    await user.click(submitButton)

    // Wait for both API call and navigation to complete
    await waitFor(() => {
      expect(api.createSession).toHaveBeenCalledWith(
        expect.objectContaining({
          kata_id: 2,
          notes: 'Test notes',
          in_course: true,
        })
      )
      expect(mockNavigate).toHaveBeenCalledWith('/')
    })
  })

  it('navigates back when cancel button is clicked', async () => {
    const user = userEvent.setup()
    ;(api.getKatas as any).mockResolvedValue({ data: mockKatas })

    renderWithRouter(<NewSessionPage />)

    await waitFor(() => {
      expect(screen.getByText('Cancel')).toBeInTheDocument()
    })

    const cancelButton = screen.getByText('Cancel')
    await user.click(cancelButton)

    expect(mockNavigate).toHaveBeenCalledWith('/')
  })

  it('disables submit button while submitting', async () => {
    const user = userEvent.setup()
    ;(api.getKatas as any).mockResolvedValue({ data: mockKatas })
    ;(api.createSession as any).mockReturnValue(new Promise(() => {})) // Never resolves

    renderWithRouter(<NewSessionPage />)

    await waitFor(() => {
      expect(screen.getByLabelText('Kata')).toBeInTheDocument()
    })

    const submitButton = screen.getByText('Create Session')
    await user.click(submitButton)

    await waitFor(() => {
      expect(screen.getByText('Creating...')).toBeInTheDocument()
      expect(screen.getByText('Creating...')).toBeDisabled()
    })
  })

  it('shows error message when submission fails', async () => {
    const user = userEvent.setup()
    ;(api.getKatas as any).mockResolvedValue({ data: mockKatas })
    ;(api.createSession as any).mockRejectedValue(new Error('Failed to create session'))

    renderWithRouter(<NewSessionPage />)

    await waitFor(() => {
      expect(screen.getByLabelText('Kata')).toBeInTheDocument()
    })

    const submitButton = screen.getByText('Create Session')
    await user.click(submitButton)

    await waitFor(() => {
      expect(screen.getByText(/Error: Failed to create session/i)).toBeInTheDocument()
    })

    // Should not navigate on error
    expect(mockNavigate).not.toHaveBeenCalled()
  })

  it('has default datetime value set to current time', async () => {
    ;(api.getKatas as any).mockResolvedValue({ data: mockKatas })

    renderWithRouter(<NewSessionPage />)

    await waitFor(() => {
      const datetimeInput = screen.getByLabelText('Practiced At') as HTMLInputElement
      expect(datetimeInput).toHaveValue()
      // Value should be in ISO format (yyyy-MM-ddThh:mm)
      expect(datetimeInput.value).toMatch(/^\d{4}-\d{2}-\d{2}T\d{2}:\d{2}$/)
    })
  })

  it('unchecks "in course" checkbox by default', async () => {
    ;(api.getKatas as any).mockResolvedValue({ data: mockKatas })

    renderWithRouter(<NewSessionPage />)

    await waitFor(() => {
      const checkbox = screen.getByLabelText(/Part of structured learning path/i)
      expect(checkbox).not.toBeChecked()
    })
  })
})
