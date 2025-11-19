import { ReactElement } from 'react'
import { render, RenderResult } from '@testing-library/react'
import { BrowserRouter } from 'react-router-dom'
import type { Session, Kata } from '../types'

interface RenderOptions {
  route?: string
}

export function renderWithRouter(ui: ReactElement, { route = '/' }: RenderOptions = {}): RenderResult {
  window.history.pushState({}, 'Test page', route)

  // Add future flags to silence React Router warnings
  const RouterWrapper = ({ children }: { children: React.ReactNode }) => (
    <BrowserRouter future={{ v7_startTransition: true, v7_relativeSplatPath: true }}>
      {children}
    </BrowserRouter>
  )

  return render(ui, { wrapper: RouterWrapper })
}

export const mockSessions: Session[] = [
  {
    id: 1,
    kata_id: 1,
    practiced_at: '2025-01-10T10:00:00Z',
    in_course: true,
    notes: 'First session notes',
    inserted_at: '2025-01-10T10:00:00Z',
    updated_at: '2025-01-10T10:00:00Z',
  },
  {
    id: 2,
    kata_id: 2,
    practiced_at: '2025-01-11T14:30:00Z',
    in_course: false,
    notes: 'Second session notes',
    inserted_at: '2025-01-11T14:30:00Z',
    updated_at: '2025-01-11T14:30:00Z',
  },
]

// Backend returns kata levels as atom strings, not integers
export const mockKatas: Kata[] = [
  {
    id: 1,
    name: 'FizzBuzz',
    level: 'yellow',
    inserted_at: '2025-01-01T00:00:00Z',
    updated_at: '2025-01-01T00:00:00Z',
  },
  {
    id: 2,
    name: 'Roman Numerals',
    level: 'orange',
    inserted_at: '2025-01-01T00:00:00Z',
    updated_at: '2025-01-01T00:00:00Z',
  },
  {
    id: 3,
    name: 'Bowling Game',
    level: 'blue',
    inserted_at: '2025-01-01T00:00:00Z',
    updated_at: '2025-01-01T00:00:00Z',
  },
]
