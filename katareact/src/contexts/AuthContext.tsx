import { createContext, useContext, useState, useEffect, useCallback, ReactNode } from 'react'
import type { User, LoginCredentials, RegisterCredentials } from '../types'
import { api } from '../services/api'

interface AuthContextType {
  user: User | null
  token: string | null
  login: (credentials: LoginCredentials) => Promise<void>
  register: (credentials: RegisterCredentials) => Promise<void>
  logout: () => void
  isAuthenticated: boolean
  isLoading: boolean
}

const AuthContext = createContext<AuthContextType | undefined>(undefined)

const TOKEN_KEY = 'katanaute_token'
const USER_KEY = 'katanaute_user'

export function AuthProvider({ children }: { children: ReactNode }) {
  const [user, setUser] = useState<User | null>(null)
  const [token, setToken] = useState<string | null>(null)
  const [isLoading, setIsLoading] = useState(true)

  const persistAuthState = useCallback((nextToken: string, nextUser: User) => {
    setToken(nextToken)
    setUser(nextUser)
    localStorage.setItem(TOKEN_KEY, nextToken)
    localStorage.setItem(USER_KEY, JSON.stringify(nextUser))
  }, [])

  const clearAuthState = useCallback(() => {
    setToken(null)
    setUser(null)
    localStorage.removeItem(TOKEN_KEY)
    localStorage.removeItem(USER_KEY)
  }, [])

  // Load auth state from localStorage on mount
  useEffect(() => {
    let isMounted = true

    const initializeAuth = async () => {
      const storedToken = localStorage.getItem(TOKEN_KEY)
      const storedUser = localStorage.getItem(USER_KEY)

      if (storedToken) {
        setToken(storedToken)

        if (storedUser) {
          try {
            setUser(JSON.parse(storedUser))
          } catch {
            localStorage.removeItem(USER_KEY)
          }
        }

        try {
          const { data } = await api.getCurrentUser()
          if (isMounted) {
            persistAuthState(storedToken, data)
          }
        } catch {
          if (isMounted) {
            clearAuthState()
          }
        }
      } else if (storedUser) {
        localStorage.removeItem(USER_KEY)
      }

      if (isMounted) {
        setIsLoading(false)
      }
    }

    void initializeAuth()

    return () => {
      isMounted = false
    }
  }, [clearAuthState, persistAuthState])

  const login = async (credentials: LoginCredentials) => {
    const response = await api.login(credentials)
    persistAuthState(response.access_token, response.user)
  }

  const register = async (credentials: RegisterCredentials) => {
    const response = await api.register(credentials)
    persistAuthState(response.access_token, response.user)
  }

  const logout = () => {
    // Attempt to revoke the API token but don't block UI
    void api.logout().catch(() => undefined)
    clearAuthState()
  }

  const value = {
    user,
    token,
    login,
    register,
    logout,
    isAuthenticated: !!token,
    isLoading,
  }

  return <AuthContext.Provider value={value}>{children}</AuthContext.Provider>
}

export function useAuth() {
  const context = useContext(AuthContext)
  if (context === undefined) {
    throw new Error('useAuth must be used within an AuthProvider')
  }
  return context
}
