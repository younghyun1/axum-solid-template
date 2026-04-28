import { defineConfig } from "vitest/config";
import solidPlugin from "vite-plugin-solid";

export default defineConfig({
  plugins: [solidPlugin()],
  build: {
    assetsDir: "assets",
    emptyOutDir: true,
    outDir: "dist",
    sourcemap: true,
    target: "es2022"
  },
  server: {
    host: "127.0.0.1",
    port: 5173,
    proxy: {
      "/api": {
        changeOrigin: true,
        target: "http://127.0.0.1:3000"
      }
    }
  },
  preview: {
    host: "127.0.0.1",
    port: 4173
  },
  test: {
    environment: "node",
    include: ["src/**/*.test.ts"]
  }
});
