import { describe, it, expect, beforeEach, vi } from 'vitest'
import { api } from './api'

const fetchMock = vi.fn<Parameters<typeof fetch>, ReturnType<typeof fetch>>()

const createMockResponse = <T>(body: T, init?: { status?: number }) =>
  ({
    ok: (init?.status ?? 200) < 400,
    status: init?.status ?? 200,
    json: async () => body,
  }) as unknown as Response

describe('API Client', () => {
  const seedToken = () => localStorage.setItem('katanaute_token', 'test-token')
  const authHeaders = {
    'Content-Type': 'application/json',
    Authorization: 'Bearer test-token',
  }

  beforeEach(() => {
    fetchMock.mockReset()
    global.fetch = fetchMock as unknown as typeof fetch
    localStorage.clear()
  })

  describe('getSessions', () => {
    it('fetches sessions from the API', async () => {
      const mockData = { data: [{ id: 1, kata_id: 1 }] }
      seedToken()
      fetchMock.mockResolvedValueOnce(createMockResponse(mockData))

      const result = await api.getSessions()

      expect(fetch).toHaveBeenCalledWith('/api/sessions', {
        headers: authHeaders,
      })
      expect(result).toEqual(mockData)
    })

    it('throws error on failed request', async () => {
      fetchMock.mockResolvedValueOnce(createMockResponse({ message: 'Server error' }, { status: 500 }))

      await expect(api.getSessions()).rejects.toThrow('Server error')
    })
  })

  describe('getSession', () => {
    it('fetches a single session by id', async () => {
      const mockData = { data: { id: 1, kata_id: 1 } }
      seedToken()
      fetchMock.mockResolvedValueOnce(createMockResponse(mockData))

      const result = await api.getSession(1)

      expect(fetch).toHaveBeenCalledWith('/api/sessions/1', {
        headers: authHeaders,
      })
      expect(result).toEqual(mockData)
    })
  })

  describe('createSession', () => {
    it('creates a new session', async () => {
      const sessionData = {
        kata_id: 1,
        practiced_at: '2025-01-10T10:00:00Z',
        in_course: false,
        notes: ''
      }
      const mockResponse = { data: { id: 1, ...sessionData } }

      seedToken()
      fetchMock.mockResolvedValueOnce(createMockResponse(mockResponse))

      const result = await api.createSession(sessionData)

      expect(fetch).toHaveBeenCalledWith('/api/sessions', {
        method: 'POST',
        headers: authHeaders,
        body: JSON.stringify({ session: sessionData }),
      })
      expect(result).toEqual(mockResponse)
    })
  })

  describe('updateSession', () => {
    it('updates an existing session', async () => {
      const sessionData = { notes: 'Updated notes' }
      const mockResponse = { data: { id: 1, ...sessionData } }

      seedToken()
      fetchMock.mockResolvedValueOnce(createMockResponse(mockResponse))

      const result = await api.updateSession(1, sessionData)

      expect(fetch).toHaveBeenCalledWith('/api/sessions/1', {
        method: 'PUT',
        headers: authHeaders,
        body: JSON.stringify({ session: sessionData }),
      })
      expect(result).toEqual(mockResponse)
    })
  })

  describe('deleteSession', () => {
    it('deletes a session', async () => {
      seedToken()
      fetchMock.mockResolvedValueOnce(createMockResponse(null, { status: 204 }))

      const result = await api.deleteSession(1)

      expect(fetch).toHaveBeenCalledWith('/api/sessions/1', {
        method: 'DELETE',
        headers: authHeaders,
      })
      expect(result).toBeNull()
    })

    it('throws error when delete fails', async () => {
      seedToken()
      fetchMock.mockResolvedValueOnce(createMockResponse(null, { status: 404 }))

      await expect(api.deleteSession(999)).rejects.toThrow('HTTP error! status: 404')
    })
  })

  describe('getKatas', () => {
    it('fetches all katas', async () => {
      const mockData = { data: [{ id: 1, name: 'FizzBuzz', level: 'yellow' }] }
      seedToken()
      fetchMock.mockResolvedValueOnce(createMockResponse(mockData))

      const result = await api.getKatas()

      expect(fetch).toHaveBeenCalledWith('/api/katas', {
        headers: authHeaders,
      })
      expect(result).toEqual(mockData)
    })
  })

  describe('getKata', () => {
    it('fetches a single kata by id', async () => {
      const mockData = { data: { id: 1, name: 'FizzBuzz', level: 'yellow' } }
      seedToken()
      fetchMock.mockResolvedValueOnce(createMockResponse(mockData))

      const result = await api.getKata(1)

      expect(fetch).toHaveBeenCalledWith('/api/katas/1', {
        headers: authHeaders,
      })
      expect(result).toEqual(mockData)
    })
  })
})
