import type { ApiResponse, Kata, Session, SessionInput } from '../types'

const API_BASE_URL = import.meta.env.VITE_API_URL || '/api'

async function handleResponse<T>(response: Response): Promise<ApiResponse<T>> {
  if (!response.ok) {
    const error = await response.json().catch(() => ({ message: 'An error occurred' }))
    throw new Error(error.message || `HTTP error! status: ${response.status}`)
  }
  return response.json()
}

export const api = {
  // Sessions
  async getSessions(): Promise<ApiResponse<Session[]>> {
    const response = await fetch(`${API_BASE_URL}/sessions`)
    return handleResponse<Session[]>(response)
  },

  async getSession(id: number): Promise<ApiResponse<Session>> {
    const response = await fetch(`${API_BASE_URL}/sessions/${id}`)
    return handleResponse<Session>(response)
  },

  async createSession(sessionData: SessionInput): Promise<ApiResponse<Session>> {
    const response = await fetch(`${API_BASE_URL}/sessions`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({ session: sessionData }),
    })
    return handleResponse<Session>(response)
  },

  async updateSession(id: number, sessionData: Partial<SessionInput>): Promise<ApiResponse<Session>> {
    const response = await fetch(`${API_BASE_URL}/sessions/${id}`, {
      method: 'PUT',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({ session: sessionData }),
    })
    return handleResponse<Session>(response)
  },

  async deleteSession(id: number): Promise<null> {
    const response = await fetch(`${API_BASE_URL}/sessions/${id}`, {
      method: 'DELETE',
    })
    if (!response.ok) {
      throw new Error(`HTTP error! status: ${response.status}`)
    }
    return null
  },

  // Katas
  async getKatas(): Promise<ApiResponse<Kata[]>> {
    const response = await fetch(`${API_BASE_URL}/katas`)
    return handleResponse<Kata[]>(response)
  },

  async getKata(id: number): Promise<ApiResponse<Kata>> {
    const response = await fetch(`${API_BASE_URL}/katas/${id}`)
    return handleResponse<Kata>(response)
  },
}
