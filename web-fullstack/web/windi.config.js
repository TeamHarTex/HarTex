import { defineConfig } from 'windicss/helpers'

export default defineConfig({
  extract: {
    include: ['src/**/*.{js,jsx,ts,tsx}'],
  },
  theme: {
    colors: {
      blurple: '#5866EF',
      'dark-blurple': '#4455EE',
      'dark-grey': '#23272A',
      'dark-orange': '#7B4A04',
      'darker-grey': '#171A1C',
      grey: '#36393F',
      orange: '#F8A532',
      white: '#FFFFFF',
    },
  },
  plugins: [],
})
