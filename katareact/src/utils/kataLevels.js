// Backend returns kata levels as atom strings (yellow, orange, etc.)
export const KATA_LEVELS = {
  yellow: 'Yellow',
  orange: 'Orange',
  green: 'Green',
  blue: 'Blue',
  brown: 'Brown',
  shodan: 'Shodan',
}

// Color mappings for kata level badges
export const KATA_LEVEL_COLORS = {
  yellow: 'bg-yellow-100 text-yellow-800',
  orange: 'bg-orange-100 text-orange-800',
  green: 'bg-green-100 text-green-800',
  blue: 'bg-blue-100 text-blue-800',
  brown: 'bg-amber-100 text-amber-800',
  shodan: 'bg-gray-900 text-white',
}

// Helper function to get level display name
export function getKataLevelName(level) {
  return KATA_LEVELS[level] || 'N/A'
}

// Helper function to get level color classes
export function getKataLevelColor(level) {
  return KATA_LEVEL_COLORS[level] || 'bg-gray-100 text-gray-800'
}
