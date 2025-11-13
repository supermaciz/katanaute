const API_BASE_URL = import.meta.env.VITE_API_URL || '/api'

async function handleResponse(response) {
  if (!response.ok) {
    const error = await response.json().catch(() => ({ message: 'An error occurred' }))
    throw new Error(error.message || `HTTP error! status: ${response.status}`)
  }
  return response.json()
}

export const api = {
  // Sessions
  async getSessions() {
    const response = await fetch(`${API_BASE_URL}/sessions`)
    return handleResponse(response)
  },

  async getSession(id) {
    const response = await fetch(`${API_BASE_URL}/sessions/${id}`)
    return handleResponse(response)
  },

  async createSession(sessionData) {
    const response = await fetch(`${API_BASE_URL}/sessions`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({ session: sessionData }),
    })
    return handleResponse(response)
  },

  async updateSession(id, sessionData) {
    const response = await fetch(`${API_BASE_URL}/sessions/${id}`, {
      method: 'PUT',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({ session: sessionData }),
    })
    return handleResponse(response)
  },

  async deleteSession(id) {
    const response = await fetch(`${API_BASE_URL}/sessions/${id}`, {
      method: 'DELETE',
    })
    if (!response.ok) {
      throw new Error(`HTTP error! status: ${response.status}`)
    }
    return response.status === 204 ? null : response.json()
  },

  // Katas
  async getKatas() {
    const response = await fetch(`${API_BASE_URL}/katas`)
    return handleResponse(response)
  },

  async getKata(id) {
    const response = await fetch(`${API_BASE_URL}/katas/${id}`)
    return handleResponse(response)
  },
}
