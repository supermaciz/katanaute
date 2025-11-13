import { render } from '@testing-library/react'
import { BrowserRouter } from 'react-router-dom'

export function renderWithRouter(ui, { route = '/' } = {}) {
  window.history.pushState({}, 'Test page', route)

  return {
    ...render(ui, { wrapper: BrowserRouter }),
  }
}

export const mockSessions = [
  {
    id: 1,
    kata_id: 1,
    practiced_at: '2025-01-10T10:00:00Z',
    in_course: true,
    notes: 'First session notes',
  },
  {
    id: 2,
    kata_id: 2,
    practiced_at: '2025-01-11T14:30:00Z',
    in_course: false,
    notes: 'Second session notes',
  },
]

export const mockKatas = [
  {
    id: 1,
    name: 'FizzBuzz',
    level: 1,
  },
  {
    id: 2,
    name: 'Roman Numerals',
    level: 2,
  },
  {
    id: 3,
    name: 'Bowling Game',
    level: 4,
  },
]
