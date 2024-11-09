/** @type {import('tailwindcss').Config} */
export default {
  content: ['./src/**/*.{ts,vue}', './index.html'],
  theme: {
    extend: {
      colors: {
        background: '#403F4C', // #0C6291
        primary: '#F2F2F2',
        accent1: '#FCBF49',
        accent2: '#AE7C7C',
        accent3: '#6CBBB3',
        accent4: '#EFE784',
        white: '#FFFFFF',
      },
    },
    fontFamily: {
      header: ['"Playfair"'],
      body: ['"Playfair Display"'],
    },
  },
  plugins: [],
}
