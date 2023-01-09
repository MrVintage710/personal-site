import { resolve } from 'path'
import { defineConfig } from 'vite'
import { svelte } from '@sveltejs/vite-plugin-svelte'

const root = resolve(__dirname, 'src')

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
        main: resolve(root, 'routes', 'home', "index.html"),
        blog: resolve(root, 'routes', 'blog', "index.html")
      }
    }
  }
})
