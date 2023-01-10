import { resolve } from 'path'
import { defineConfig } from 'vite'
import { svelte } from '@sveltejs/vite-plugin-svelte'
import { watch } from 'fs'

const root = resolve(__dirname, 'src/routes')

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [svelte()],
  server: {
    host: true
  },
  build: {
    outDir: "../backend/dist",
    emptyOutDir: true,
    rollupOptions: {
      input: {
        main: resolve(root, 'home', "index.html"),
        blog: resolve(root, 'blog', "index.html")
      }
    },
    watch : {}
  }
})
