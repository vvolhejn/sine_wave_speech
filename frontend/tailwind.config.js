/** @type {import('tailwindcss').Config} */
export default {
  content: ['./src/**/*.{ts,vue}', './index.html'],
  theme: {
    extend: {},
    colors: {
      background: '#403F4C', // #0C6291
      primary: '#F2F2F2',
      accent1: '#AE7C7C',
      accent2: '#6CBBB3',
      accent3: '#FCBF49',
      accent4: '#EFE784',
    },
    fontFamily: {
      header: ['"Playfair"'],
      body: ['"Playfair Display"'],
    },
  },
  plugins: [],
}
