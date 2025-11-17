import { describe, it, expect, beforeEach, vi } from 'vitest'
import { api } from './api'

describe('API Client', () => {
  const seedToken = () => localStorage.setItem('katanaute_token', 'test-token')
  const authHeaders = {
    'Content-Type': 'application/json',
    Authorization: 'Bearer test-token',
  }

  beforeEach(() => {
    global.fetch = vi.fn() as any
    localStorage.clear()
  })

  describe('getSessions', () => {
    it('fetches sessions from the API', async () => {
      const mockData = { data: [{ id: 1, kata_id: 1 }] }
      seedToken()
      ;(global.fetch as any).mockResolvedValueOnce({
        ok: true,
        json: async () => mockData,
      })

      const result = await api.getSessions()

      expect(fetch).toHaveBeenCalledWith('/api/sessions', {
        headers: authHeaders,
      })
      expect(result).toEqual(mockData)
    })

    it('throws error on failed request', async () => {
      ;(global.fetch as any).mockResolvedValueOnce({
        ok: false,
        status: 500,
        json: async () => ({ message: 'Server error' }),
      })

      await expect(api.getSessions()).rejects.toThrow('Server error')
    })
  })

  describe('getSession', () => {
    it('fetches a single session by id', async () => {
      const mockData = { data: { id: 1, kata_id: 1 } }
      seedToken()
      ;(global.fetch as any).mockResolvedValueOnce({
        ok: true,
        json: async () => mockData,
      })

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
      ;(global.fetch as any).mockResolvedValueOnce({
        ok: true,
        json: async () => mockResponse,
      })

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
      ;(global.fetch as any).mockResolvedValueOnce({
        ok: true,
        json: async () => mockResponse,
      })

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
      ;(global.fetch as any).mockResolvedValueOnce({
        ok: true,
        status: 204,
      })

      const result = await api.deleteSession(1)

      expect(fetch).toHaveBeenCalledWith('/api/sessions/1', {
        method: 'DELETE',
        headers: authHeaders,
      })
      expect(result).toBeNull()
    })

    it('throws error when delete fails', async () => {
      ;(global.fetch as any).mockResolvedValueOnce({
        ok: false,
        status: 404,
      })

      await expect(api.deleteSession(999)).rejects.toThrow('HTTP error! status: 404')
    })
  })

  describe('getKatas', () => {
    it('fetches all katas', async () => {
      const mockData = { data: [{ id: 1, name: 'FizzBuzz', level: 'yellow' }] }
      seedToken()
      ;(global.fetch as any).mockResolvedValueOnce({
        ok: true,
        json: async () => mockData,
      })

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
      ;(global.fetch as any).mockResolvedValueOnce({
        ok: true,
        json: async () => mockData,
      })

      const result = await api.getKata(1)

      expect(fetch).toHaveBeenCalledWith('/api/katas/1', {
        headers: authHeaders,
      })
      expect(result).toEqual(mockData)
    })
  })
})
