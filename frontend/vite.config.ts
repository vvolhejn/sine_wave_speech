import path from 'path'
import { defineConfig } from 'vite'

import vue from '@vitejs/plugin-vue'

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [vue()],
  build: {
    rollupOptions: {
      input: {
        explanation: './explanation/index.html',
        live: './live/index.html',
        // The redirection should be handled by Vercel in production, but for local
        // testing it's also useful to have a HTML-based redirect.
        root: './index.html',
      },
    },
    minify: true,
  },
  resolve: {
    alias: {
      '@': path.resolve(__dirname, './src/'),
    },
  },
  define: {
    __VUE_PROD_HYDRATION_MISMATCH_DETAILS__: false,
  },
})
