// Kata levels based on martial arts belt progression
export type KataLevel = 'yellow' | 'orange' | 'green' | 'blue' | 'brown' | 'shodan'

// Kata entity
export interface Kata {
  id: number
  name: string
  level: KataLevel
  inserted_at: string
  updated_at: string
}

// Session entity
export interface Session {
  id: number
  kata_id: number
  practiced_at: string
  in_course: boolean
  notes: string
  inserted_at: string
  updated_at: string
  kata?: Kata  // Preloaded kata data (optional)
}

// API response wrapper
export interface ApiResponse<T> {
  data: T
}

// Session creation/update data
export interface SessionInput {
  kata_id: number
  practiced_at: string
  in_course: boolean
  notes: string
}

// Kata map for quick lookups
export type KataMap = Record<number, Kata>

// User entity
export interface User {
  id: number
  email: string
  confirmed_at?: string | null
}

// Auth types
export interface LoginCredentials {
  email: string
  password: string
}

export interface RegisterCredentials {
  email: string
  password: string
}

export interface AuthResponse {
  access_token: string
  token_type: string
  user: User
}
