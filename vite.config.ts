import { defineConfig } from 'vite';
import { svelte } from '@sveltejs/vite-plugin-svelte';

export default defineConfig({
  root: './client',
  plugins: [svelte()],
  server: {
    proxy: {
      '/search': 'http://localhost:7500',
    },
  },
});
