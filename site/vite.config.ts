import { defineConfig } from 'vite';
import react from '@vitejs/plugin-react';
import { vanillaExtractPlugin } from '@vanilla-extract/vite-plugin';
import path from 'path';
import { cargoDefines } from './cargo-meta';

export default defineConfig({
  plugins: [react(), vanillaExtractPlugin()],
  define: cargoDefines,
  resolve: {
    alias: {
      '@': path.resolve(__dirname, 'src'),
    },
  },
});
