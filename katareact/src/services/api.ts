import type { ApiResponse, Kata, Session, SessionInput, LoginCredentials, RegisterCredentials, AuthResponse, User } from '../types'

const API_BASE_URL = import.meta.env.VITE_API_URL || '/api'
const TOKEN_KEY = 'katanaute_token'

function getAuthHeaders(): HeadersInit {
  const token = localStorage.getItem(TOKEN_KEY)
  const headers: HeadersInit = {
    'Content-Type': 'application/json',
  }

  if (token) {
    headers['Authorization'] = `Bearer ${token}`
  }

  return headers
}

async function handleResponse<T>(response: Response): Promise<ApiResponse<T>> {
  if (!response.ok) {
    const error = await response.json().catch(() => ({ message: 'An error occurred' }))
    throw new Error(error.message || `HTTP error! status: ${response.status}`)
  }
  return response.json()
}

async function handleAuthResponse(response: Response): Promise<AuthResponse> {
  if (!response.ok) {
    const error = await response.json().catch(() => ({ message: 'An error occurred' }))
    throw new Error(error.message || `HTTP error! status: ${response.status}`)
  }
  const data = await response.json()
  return data.data
}

export const api = {
  // Auth
  async login(credentials: LoginCredentials): Promise<AuthResponse> {
    const response = await fetch(`${API_BASE_URL}/auth/token`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify(credentials),
    })
    return handleAuthResponse(response)
  },

  async register(credentials: RegisterCredentials): Promise<AuthResponse> {
    const response = await fetch(`${API_BASE_URL}/auth/register`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify(credentials),
    })
    return handleAuthResponse(response)
  },

  async getCurrentUser(): Promise<ApiResponse<User>> {
    const response = await fetch(`${API_BASE_URL}/auth/me`, {
      headers: getAuthHeaders(),
    })
    return handleResponse<User>(response)
  },

  async logout(): Promise<void> {
    const response = await fetch(`${API_BASE_URL}/auth/token`, {
      method: 'DELETE',
      headers: getAuthHeaders(),
    })
    if (!response.ok) {
      throw new Error(`HTTP error! status: ${response.status}`)
    }
  },

  // Sessions
  async getSessions(): Promise<ApiResponse<Session[]>> {
    const response = await fetch(`${API_BASE_URL}/sessions`, {
      headers: getAuthHeaders(),
    })
    return handleResponse<Session[]>(response)
  },

  async getSession(id: number): Promise<ApiResponse<Session | null>> {
    const response = await fetch(`${API_BASE_URL}/sessions/${id}`, {
      headers: getAuthHeaders(),
    })
    return handleResponse<Session | null>(response)
  },

  async createSession(sessionData: SessionInput): Promise<ApiResponse<Session>> {
    const response = await fetch(`${API_BASE_URL}/sessions`, {
      method: 'POST',
      headers: getAuthHeaders(),
      body: JSON.stringify({ session: sessionData }),
    })
    return handleResponse<Session>(response)
  },

  async updateSession(id: number, sessionData: Partial<SessionInput>): Promise<ApiResponse<Session>> {
    const response = await fetch(`${API_BASE_URL}/sessions/${id}`, {
      method: 'PUT',
      headers: getAuthHeaders(),
      body: JSON.stringify({ session: sessionData }),
    })
    return handleResponse<Session>(response)
  },

  async deleteSession(id: number): Promise<null> {
    const response = await fetch(`${API_BASE_URL}/sessions/${id}`, {
      method: 'DELETE',
      headers: getAuthHeaders(),
    })
    if (!response.ok) {
      throw new Error(`HTTP error! status: ${response.status}`)
    }
    return null
  },

  // Katas
  async getKatas(): Promise<ApiResponse<Kata[]>> {
    const response = await fetch(`${API_BASE_URL}/katas`, {
      headers: getAuthHeaders(),
    })
    return handleResponse<Kata[]>(response)
  },

  async getKata(id: number): Promise<ApiResponse<Kata>> {
    const response = await fetch(`${API_BASE_URL}/katas/${id}`, {
      headers: getAuthHeaders(),
    })
    return handleResponse<Kata>(response)
  },
}
