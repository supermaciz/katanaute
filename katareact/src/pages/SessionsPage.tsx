import { useState, useEffect } from 'react'
import { Link } from 'react-router-dom'
import { api } from '../services/api'
import { getKataLevelName, getKataLevelColor } from '../utils/kataLevels'
import type { Session, KataMap } from '../types'

function SessionsPage() {
  const [sessions, setSessions] = useState<Session[]>([])
  const [katas, setKatas] = useState<KataMap>({})
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState<string | null>(null)

  useEffect(() => {
    loadData()
  }, [])

  async function loadData() {
    try {
      setLoading(true)
      const [sessionsData, katasData] = await Promise.all([
        api.getSessions(),
        api.getKatas(),
      ])

      // Create a map of kata IDs to kata objects
      const katasMap: KataMap = {}
      katasData.data.forEach((kata) => {
        katasMap[kata.id] = kata
      })

      // Sort sessions by practice date (descending - newest first)
      const sortedSessions = [...sessionsData.data].sort((a, b) => {
        return new Date(b.practiced_at).getTime() - new Date(a.practiced_at).getTime()
      })

      setSessions(sortedSessions)
      setKatas(katasMap)
    } catch (err) {
      setError(err instanceof Error ? err.message : 'An error occurred')
    } finally {
      setLoading(false)
    }
  }

  async function handleDelete(id: number) {
    if (!window.confirm('Are you sure you want to delete this session?')) {
      return
    }

    try {
      await api.deleteSession(id)
      setSessions(sessions.filter((s) => s.id !== id))
    } catch (err) {
      alert('Failed to delete session: ' + (err instanceof Error ? err.message : 'Unknown error'))
    }
  }

  function formatDate(dateString: string) {
    return new Date(dateString).toLocaleString('en-US', {
      year: 'numeric',
      month: 'short',
      day: 'numeric',
      hour: '2-digit',
      minute: '2-digit',
    })
  }

  if (loading) {
    return (
      <div className="flex justify-center items-center h-64">
        <div className="text-gray-500">Loading sessions...</div>
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

  return (
    <div>
      <div className="px-4 sm:px-0 flex justify-between items-center mb-6">
        <div>
          <h1 className="text-3xl font-bold text-gray-900">Training Sessions</h1>
          <p className="mt-1 text-sm text-gray-600">
            Track your kata practice sessions
          </p>
        </div>
        <Link
          to="/sessions/new"
          className="bg-blue-600 hover:bg-blue-700 text-white font-medium py-2 px-4 rounded-lg transition"
        >
          New Session
        </Link>
      </div>

      {sessions.length === 0 ? (
        <div className="text-center py-12 bg-white rounded-lg shadow">
          <p className="text-gray-500 mb-4">No sessions yet</p>
          <Link
            to="/sessions/new"
            className="text-blue-600 hover:text-blue-700 font-medium"
          >
            Create your first session
          </Link>
        </div>
      ) : (
        <div className="bg-white shadow-md rounded-lg overflow-hidden">
          <table className="min-w-full divide-y divide-gray-200">
            <thead className="bg-gray-50">
              <tr>
                <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                  Kata
                </th>
                <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                  Level
                </th>
                <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                  Practiced At
                </th>
                <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                  In Course
                </th>
                <th className="px-6 py-3 text-right text-xs font-medium text-gray-500 uppercase tracking-wider">
                  Actions
                </th>
              </tr>
            </thead>
            <tbody className="bg-white divide-y divide-gray-200">
              {sessions.map((session) => {
                const kata = katas[session.kata_id]
                return (
                  <tr key={session.id} className="hover:bg-gray-50">
                    <td className="px-6 py-4 whitespace-nowrap">
                      <Link
                        to={`/sessions/${session.id}`}
                        className="text-blue-600 hover:text-blue-800 font-medium"
                      >
                        {kata?.name || 'Unknown Kata'}
                      </Link>
                    </td>
                    <td className="px-6 py-4 whitespace-nowrap">
                      <span className={`px-2 inline-flex text-xs leading-5 font-semibold rounded-full ${getKataLevelColor(kata?.level)}`}>
                        {getKataLevelName(kata?.level)}
                      </span>
                    </td>
                    <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-900">
                      {formatDate(session.practiced_at)}
                    </td>
                    <td className="px-6 py-4 whitespace-nowrap">
                      {session.in_course ? (
                        <span className="px-2 inline-flex text-xs leading-5 font-semibold rounded-full bg-green-100 text-green-800">
                          Yes
                        </span>
                      ) : (
                        <span className="px-2 inline-flex text-xs leading-5 font-semibold rounded-full bg-gray-100 text-gray-800">
                          No
                        </span>
                      )}
                    </td>
                    <td className="px-6 py-4 whitespace-nowrap text-right text-sm font-medium">
                      <button
                        onClick={() => handleDelete(session.id)}
                        className="text-red-600 hover:text-red-900 ml-4"
                      >
                        Delete
                      </button>
                    </td>
                  </tr>
                )
              })}
            </tbody>
          </table>
        </div>
      )}
    </div>
  )
}

export default SessionsPage
