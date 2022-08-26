import { defineConfig } from 'windicss/helpers'

export default defineConfig({
  extract: {
    include: ['src/**/*.{js,jsx,ts,tsx}'],
  },
  theme: {
    colors: {
      'dark-grey': '#23272A',
      grey: '#36393F',
      blurple: '#7289DA',
      'dark-blurple': '#4E5D94',
      'dark-orange': '#7B4A04',
      white: '#FFFFFF',
      orange: '#F8A532',
    },
  },
  plugins: [],
})
