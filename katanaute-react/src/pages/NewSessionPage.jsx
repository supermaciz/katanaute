import { useState, useEffect } from 'react'
import { useNavigate } from 'react-router-dom'
import { api } from '../services/api'

const KATA_LEVELS = {
  1: 'Yellow',
  2: 'Orange',
  3: 'Green',
  4: 'Blue',
  5: 'Brown',
  6: 'Shodan',
}

function NewSessionPage() {
  const navigate = useNavigate()
  const [katas, setKatas] = useState([])
  const [loading, setLoading] = useState(true)
  const [submitting, setSubmitting] = useState(false)
  const [error, setError] = useState(null)
  const [formData, setFormData] = useState({
    kata_id: '',
    practiced_at: new Date().toISOString().slice(0, 16),
    in_course: false,
    notes: '',
  })

  useEffect(() => {
    loadKatas()
  }, [])

  async function loadKatas() {
    try {
      const data = await api.getKatas()
      setKatas(data.data)
      // Set first kata as default
      if (data.data.length > 0) {
        setFormData((prev) => ({ ...prev, kata_id: data.data[0].id }))
      }
    } catch (err) {
      setError(err.message)
    } finally {
      setLoading(false)
    }
  }

  function handleChange(e) {
    const { name, value, type, checked } = e.target
    setFormData((prev) => ({
      ...prev,
      [name]: type === 'checkbox' ? checked : value,
    }))
  }

  async function handleSubmit(e) {
    e.preventDefault()
    setSubmitting(true)
    setError(null)

    try {
      // Convert kata_id to integer
      const sessionData = {
        ...formData,
        kata_id: parseInt(formData.kata_id, 10),
      }
      await api.createSession(sessionData)
      navigate('/')
    } catch (err) {
      setError(err.message)
      setSubmitting(false)
    }
  }

  if (loading) {
    return (
      <div className="flex justify-center items-center h-64">
        <div className="text-gray-500">Loading...</div>
      </div>
    )
  }

  return (
    <div className="max-w-2xl mx-auto">
      <div className="px-4 sm:px-0 mb-6">
        <h1 className="text-3xl font-bold text-gray-900">New Training Session</h1>
        <p className="mt-1 text-sm text-gray-600">
          Record a new kata practice session
        </p>
      </div>

      {error && (
        <div className="mb-4 bg-red-50 border border-red-200 text-red-700 px-4 py-3 rounded">
          Error: {error}
        </div>
      )}

      <form onSubmit={handleSubmit} className="bg-white shadow-md rounded-lg p-6">
        <div className="space-y-6">
          <div>
            <label htmlFor="kata_id" className="block text-sm font-medium text-gray-700">
              Kata
            </label>
            <select
              id="kata_id"
              name="kata_id"
              value={formData.kata_id}
              onChange={handleChange}
              required
              className="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500 px-3 py-2 border"
            >
              {katas.map((kata) => (
                <option key={kata.id} value={kata.id}>
                  {kata.name} ({KATA_LEVELS[kata.level]})
                </option>
              ))}
            </select>
          </div>

          <div>
            <label htmlFor="practiced_at" className="block text-sm font-medium text-gray-700">
              Practiced At
            </label>
            <input
              type="datetime-local"
              id="practiced_at"
              name="practiced_at"
              value={formData.practiced_at}
              onChange={handleChange}
              required
              className="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500 px-3 py-2 border"
            />
          </div>

          <div className="flex items-center">
            <input
              type="checkbox"
              id="in_course"
              name="in_course"
              checked={formData.in_course}
              onChange={handleChange}
              className="h-4 w-4 text-blue-600 focus:ring-blue-500 border-gray-300 rounded"
            />
            <label htmlFor="in_course" className="ml-2 block text-sm text-gray-900">
              Part of structured learning path (In Course)
            </label>
          </div>

          <div>
            <label htmlFor="notes" className="block text-sm font-medium text-gray-700">
              Notes (Markdown supported)
            </label>
            <textarea
              id="notes"
              name="notes"
              rows={8}
              value={formData.notes}
              onChange={handleChange}
              placeholder="Enter your notes here... Markdown is supported."
              className="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500 px-3 py-2 border font-mono text-sm"
            />
            <p className="mt-2 text-sm text-gray-500">
              You can use Markdown syntax for formatting (e.g., **bold**, *italic*, `code`, etc.)
            </p>
          </div>

          <div className="flex justify-end space-x-3 pt-4">
            <button
              type="button"
              onClick={() => navigate('/')}
              className="px-4 py-2 border border-gray-300 rounded-md text-sm font-medium text-gray-700 hover:bg-gray-50"
            >
              Cancel
            </button>
            <button
              type="submit"
              disabled={submitting}
              className="px-4 py-2 border border-transparent rounded-md shadow-sm text-sm font-medium text-white bg-blue-600 hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500 disabled:opacity-50"
            >
              {submitting ? 'Creating...' : 'Create Session'}
            </button>
          </div>
        </div>
      </form>
    </div>
  )
}

export default NewSessionPage
