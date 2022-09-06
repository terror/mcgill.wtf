import { defineConfig } from 'vite';
import react from '@vitejs/plugin-react';

export default defineConfig({
  root: './client',
  plugins: [react()],
  server: {
    proxy: {
      '/search': 'http://localhost:7500',
    },
  },
});
