import resolveConfig from 'tailwindcss/resolveConfig'

// Is there a type-safe way to import the tailwind config?
// @ts-ignore
import partialTailwindConfig from '../tailwind.config.js'

const tailwindConfig = resolveConfig(partialTailwindConfig)

// hex strings
export const accentColors: string[] = [
  tailwindConfig.theme.colors.accent4,
  tailwindConfig.theme.colors.accent3,
  tailwindConfig.theme.colors.accent2,
  tailwindConfig.theme.colors.accent1,
]
