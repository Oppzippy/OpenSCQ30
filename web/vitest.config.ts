/// <reference types="vitest" />

import react from "@vitejs/plugin-react-swc";
import { defineConfig } from "vite";
import { VitePWA } from "vite-plugin-pwa";
import topLevelAwait from "vite-plugin-top-level-await";
import wasm from "vite-plugin-wasm";

export default defineConfig({
  plugins: [wasm(), topLevelAwait(), react(), VitePWA()],
  test: {
    dir: "tests",
    environment: "jsdom",
    setupFiles: ["./tests/setupTests.ts"],
  },
});
