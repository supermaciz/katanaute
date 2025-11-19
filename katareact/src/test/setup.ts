import { afterEach, beforeAll } from 'vitest'
import { cleanup } from '@testing-library/react'
import '@testing-library/jest-dom/vitest'

// Suppress act() warnings for user-event interactions
// These are expected when using @testing-library/user-event with React 18
// See: https://github.com/testing-library/user-event/issues/1115
const originalError = console.error
beforeAll(() => {
  console.error = (...args: any[]) => {
    if (
      typeof args[0] === 'string' &&
      args[0].includes('Warning: An update to') &&
      args[0].includes('inside a test was not wrapped in act')
    ) {
      return
    }
    originalError.call(console, ...args)
  }
})

// Cleanup after each test
afterEach(() => {
  cleanup()
})
