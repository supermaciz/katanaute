import { useState, useEffect } from 'react'
import { useParams, useNavigate, Link } from 'react-router-dom'
import ReactMarkdown from 'react-markdown'
import { api } from '../services/api'

const KATA_LEVELS = {
  1: 'Yellow',
  2: 'Orange',
  3: 'Green',
  4: 'Blue',
  5: 'Brown',
  6: 'Shodan',
}

function SessionDetailPage() {
  const { id } = useParams()
  const navigate = useNavigate()
  const [session, setSession] = useState(null)
  const [kata, setKata] = useState(null)
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState(null)

  useEffect(() => {
    loadSession()
  }, [id])

  async function loadSession() {
    try {
      setLoading(true)
      const sessionData = await api.getSession(id)
      setSession(sessionData.data)

      // Load the associated kata
      if (sessionData.data && sessionData.data.kata_id) {
        const kataData = await api.getKata(sessionData.data.kata_id)
        setKata(kataData.data)
      }
    } catch (err) {
      setError(err.message)
    } finally {
      setLoading(false)
    }
  }

  async function handleDelete() {
    if (!window.confirm('Are you sure you want to delete this session?')) {
      return
    }

    try {
      await api.deleteSession(id)
      navigate('/')
    } catch (err) {
      alert('Failed to delete session: ' + err.message)
    }
  }

  function formatDate(dateString) {
    return new Date(dateString).toLocaleString('en-US', {
      year: 'numeric',
      month: 'long',
      day: 'numeric',
      hour: '2-digit',
      minute: '2-digit',
    })
  }

  if (loading) {
    return (
      <div className="flex justify-center items-center h-64">
        <div className="text-gray-500">Loading session...</div>
      </div>
    )
  }

  if (error) {
    return (
      <div className="bg-red-50 border border-red-200 text-red-700 px-4 py-3 rounded">
        Error: {error}
      </div>
    )
  }

  if (!session) {
    return (
      <div className="bg-yellow-50 border border-yellow-200 text-yellow-700 px-4 py-3 rounded">
        Session not found
      </div>
    )
  }

  return (
    <div className="max-w-4xl mx-auto">
      <div className="mb-6">
        <Link to="/" className="text-blue-600 hover:text-blue-800 text-sm">
          ← Back to Sessions
        </Link>
      </div>

      <div className="bg-white shadow-lg rounded-lg overflow-hidden">
        <div className="px-6 py-8">
          <div className="flex justify-between items-start mb-6">
            <div>
              <h1 className="text-3xl font-bold text-gray-900 mb-2">
                {kata?.name || 'Unknown Kata'}
              </h1>
              <div className="flex items-center space-x-4">
                <span className="px-3 py-1 text-sm font-semibold rounded-full bg-blue-100 text-blue-800">
                  {KATA_LEVELS[kata?.level] || 'N/A'}
                </span>
                {session.in_course && (
                  <span className="px-3 py-1 text-sm font-semibold rounded-full bg-green-100 text-green-800">
                    In Course
                  </span>
                )}
              </div>
            </div>
            <button
              onClick={handleDelete}
              className="px-4 py-2 bg-red-600 hover:bg-red-700 text-white rounded-lg transition"
            >
              Delete
            </button>
          </div>

          <div className="border-t border-gray-200 pt-6">
            <dl className="grid grid-cols-1 gap-x-4 gap-y-6 sm:grid-cols-2">
              <div>
                <dt className="text-sm font-medium text-gray-500">Practiced At</dt>
                <dd className="mt-1 text-sm text-gray-900">{formatDate(session.practiced_at)}</dd>
              </div>
              <div>
                <dt className="text-sm font-medium text-gray-500">Session ID</dt>
                <dd className="mt-1 text-sm text-gray-900 font-mono">{session.id}</dd>
              </div>
            </dl>
          </div>

          {session.notes && (
            <div className="mt-8 border-t border-gray-200 pt-6">
              <h2 className="text-lg font-semibold text-gray-900 mb-4">Notes</h2>
              <div className="prose prose-sm max-w-none bg-gray-50 rounded-lg p-6">
                <ReactMarkdown>{session.notes}</ReactMarkdown>
              </div>
            </div>
          )}
        </div>
      </div>
    </div>
  )
}

export default SessionDetailPage
