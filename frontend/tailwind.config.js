/** @type {import('tailwindcss').Config} */
export default {
  content: ['./src/**/*.{ts,vue}', './explanation/index.html', './live/index.html'],
  theme: {
    extend: {
      colors: {
        background: '#403F4C', // #0C6291
        primary: '#F2F2F2',
        secondary: {
          50: '#f3faf8',
          100: '#d8efeb',
          200: '#b1ded8',
          300: '#6cbbb3',
          400: '#59a8a2',
          500: '#3f8d89',
          600: '#30716f',
          700: '#2a5b5a',
          800: '#254a4a',
          900: '#223f3f',
          950: '#0f2424',
        },
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
