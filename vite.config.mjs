import { defineConfig } from 'vite';
import vue from '@vitejs/plugin-vue';
import { tauri } from 'vite-plugin-tauri';

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [vue(), tauri()],
  build: {
    outDir: 'dist',
  },
});
