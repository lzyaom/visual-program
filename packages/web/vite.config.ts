import { fileURLToPath, URL } from 'node:url'

import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'
import vueJsx from '@vitejs/plugin-vue-jsx'
import VueDevTools from 'vite-plugin-vue-devtools'
import taiwindcss from 'tailwindcss'
import autoprefixer from 'autoprefixer'

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [vue(), vueJsx(), VueDevTools()],
  resolve: {
    alias: {
      '@': fileURLToPath(new URL('./src', import.meta.url))
    }
  },
  server: {
    host: '0.0.0.0',
    hmr: true,
    proxy: {
      '^/api': {
        target: 'http://localhost:3000/api',
        changeOrigin: true,
        rewrite(path) {
          return path.replace(/^\/api/, '')
        }
      }
    }
  },
  css: {
    postcss: {
      plugins: [taiwindcss(), autoprefixer()]
    }
  }
})
