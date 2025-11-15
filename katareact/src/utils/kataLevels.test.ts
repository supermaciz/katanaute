import { describe, it, expect } from 'vitest'
import { getKataLevelName, getKataLevelColor, KATA_LEVELS, KATA_LEVEL_COLORS } from './kataLevels'

describe('kataLevels utility', () => {
  describe('KATA_LEVELS', () => {
    it('has all expected level mappings', () => {
      expect(KATA_LEVELS).toEqual({
        yellow: 'Yellow',
        orange: 'Orange',
        green: 'Green',
        blue: 'Blue',
        brown: 'Brown',
        shodan: 'Shodan',
      })
    })
  })

  describe('KATA_LEVEL_COLORS', () => {
    it('has color classes for all levels', () => {
      expect(KATA_LEVEL_COLORS).toEqual({
        yellow: 'bg-yellow-100 text-yellow-800',
        orange: 'bg-orange-100 text-orange-800',
        green: 'bg-green-100 text-green-800',
        blue: 'bg-blue-100 text-blue-800',
        brown: 'bg-amber-100 text-amber-800',
        shodan: 'bg-gray-900 text-white',
      })
    })
  })

  describe('getKataLevelName', () => {
    it('returns correct name for yellow level', () => {
      expect(getKataLevelName('yellow')).toBe('Yellow')
    })

    it('returns correct name for orange level', () => {
      expect(getKataLevelName('orange')).toBe('Orange')
    })

    it('returns correct name for green level', () => {
      expect(getKataLevelName('green')).toBe('Green')
    })

    it('returns correct name for blue level', () => {
      expect(getKataLevelName('blue')).toBe('Blue')
    })

    it('returns correct name for brown level', () => {
      expect(getKataLevelName('brown')).toBe('Brown')
    })

    it('returns correct name for shodan level', () => {
      expect(getKataLevelName('shodan')).toBe('Shodan')
    })

    it('returns N/A for undefined level', () => {
      expect(getKataLevelName(undefined)).toBe('N/A')
    })
  })

  describe('getKataLevelColor', () => {
    it('returns yellow colors for yellow level', () => {
      expect(getKataLevelColor('yellow')).toBe('bg-yellow-100 text-yellow-800')
    })

    it('returns orange colors for orange level', () => {
      expect(getKataLevelColor('orange')).toBe('bg-orange-100 text-orange-800')
    })

    it('returns green colors for green level', () => {
      expect(getKataLevelColor('green')).toBe('bg-green-100 text-green-800')
    })

    it('returns blue colors for blue level', () => {
      expect(getKataLevelColor('blue')).toBe('bg-blue-100 text-blue-800')
    })

    it('returns amber colors for brown level', () => {
      expect(getKataLevelColor('brown')).toBe('bg-amber-100 text-amber-800')
    })

    it('returns black/white colors for shodan level', () => {
      expect(getKataLevelColor('shodan')).toBe('bg-gray-900 text-white')
    })

    it('returns default gray colors for undefined level', () => {
      expect(getKataLevelColor(undefined)).toBe('bg-gray-100 text-gray-800')
    })
  })
})
