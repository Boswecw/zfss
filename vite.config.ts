import { defineConfig } from "vite";

export default defineConfig({
  clearScreen: false,
  root: "src",
  publicDir: "../public",
  server: {
    port: 5173,
    strictPort: true,
  },
  envPrefix: ["VITE_", "TAURI_"],
  build: {
    outDir: "../dist",
    emptyOutDir: true,
    target: ["es2021", "chrome100", "safari13"],
    minify: !process.env.TAURI_DEBUG ? "esbuild" : false,
    sourcemap: !!process.env.TAURI_DEBUG,
  },
});
