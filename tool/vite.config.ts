import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";
import xx from './src/assets/wasm_execute.js';
import wasm from "vite-plugin-wasm";
import golangWasm from 'vite-plugin-golang-wasm';
import topLevelAwait from "vite-plugin-top-level-await";



export default defineConfig(async () => ({
  plugins: [react()],

  // Vite options tailored for Tauri development and only applied in `tauri dev` or `tauri build`
  //
  // 1. prevent vite from obscuring rust errors
  clearScreen: false,
  // 2. tauri expects a fixed port, fail if that port is not available
  server: {
    mimeTypes: {
      // Override default MIME types
      '.wasm': 'application/wasm'
    },
    port: 1420,
    strictPort: true,
    watch: {
      // 3. tell vite to ignore watching `src-tauri`
      ignored: ["**/src-tauri/**"],
    },
  },
}));
