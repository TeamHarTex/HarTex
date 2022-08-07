import { defineConfig } from 'windicss/helpers'

export default defineConfig({
  extract: {
    include: ['src/**/*.{js,jsx,ts,tsx}'],
  },
  theme: {
    colors: {
      grey: '#37393E',
      blurple: '#5866EF',
      'dark-blurple': '#4454C4',
      white: '#FFFFFF',
      orange: '#F66B0E',
    },
  },
  plugins: [],
})
