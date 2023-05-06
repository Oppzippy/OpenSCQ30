import react from "@vitejs/plugin-react-swc";
import { defineConfig } from "vite";
import { VitePWA } from "vite-plugin-pwa";
import topLevelAwait from "vite-plugin-top-level-await";
import wasm from "vite-plugin-wasm";

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [
    wasm(),
    topLevelAwait(),
    react(),
    VitePWA({
      includeAssets: ["icon.svg", "apple-touch-icon.png", "mask-icon.png"],
      workbox: {
        globPatterns: ["**/*.{js,css,html,wasm}"],
      },
      manifest: {
        name: "OpenSCQ30",
        short_name: "OpenSCQ30",
        description:
          "Cross platform application for controlling settings of Soundcore Q30 headphones. Supports desktop (CLI and GTK4 GUI), Android, and Web Bluetooth.",
        theme_color: "#474e3d",
        scope: ".",
        icons: [
          {
            src: "pwa-192x192.png",
            sizes: "192x192",
            type: "image/png",
          },
          {
            src: "pwa-512x512.png",
            sizes: "512x512",
            type: "image/png",
          },
          {
            src: "mask-icon.png",
            sizes: "640x640",
            type: "image/png",
            purpose: "any maskable",
          },
        ],
      },
    }),
  ],
  assetsInclude: ["../web-wasm/pkg"],
  base: "./", // relative paths for github pages
});
