import { BrowserRouter as Router, Routes, Route, Link } from 'react-router-dom'
import SessionsPage from './pages/SessionsPage'
import SessionDetailPage from './pages/SessionDetailPage'
import NewSessionPage from './pages/NewSessionPage'

function App() {
  return (
    <Router>
      <div className="min-h-screen bg-gray-50">
        <nav className="bg-white shadow-lg">
          <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
            <div className="flex justify-between h-16">
              <div className="flex">
                <Link to="/" className="flex items-center px-2 py-2 text-gray-900 text-xl font-bold">
                  Katanaute
                </Link>
                <div className="hidden sm:ml-6 sm:flex sm:space-x-8">
                  <Link
                    to="/"
                    className="inline-flex items-center px-1 pt-1 text-sm font-medium text-gray-900 hover:text-gray-700"
                  >
                    Sessions
                  </Link>
                  <Link
                    to="/sessions/new"
                    className="inline-flex items-center px-1 pt-1 text-sm font-medium text-gray-500 hover:text-gray-700"
                  >
                    New Session
                  </Link>
                </div>
              </div>
            </div>
          </div>
        </nav>

        <main className="max-w-7xl mx-auto py-6 sm:px-6 lg:px-8">
          <Routes>
            <Route path="/" element={<SessionsPage />} />
            <Route path="/sessions/:id" element={<SessionDetailPage />} />
            <Route path="/sessions/new" element={<NewSessionPage />} />
          </Routes>
        </main>
      </div>
    </Router>
  )
}

export default App
