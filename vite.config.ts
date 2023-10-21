import { defineConfig } from "vite";
import vue from "@vitejs/plugin-vue";
import path from "path";


// https://vitejs.dev/config/
export default defineConfig(async () => ({
  pluginOptions: {
    quasar: {
      importStrategy: 'kebab',
      rtlSupport: false
    }
  },
  transpileDependencies: [
    'quasar'
  ],
  plugins: [
    vue(),
  ],
  extras: [
    'material-icons',
    'mdi-v6',
    'ionicons-v4', // 最后一个webfont在v4.6.3中可用。
    'eva-icons',
    'fontawesome-v6',
    'themify',
    'line-awesome',
    'bootstrap-icons'
  ],
  resolve: {
    alias: {
      "@": path.resolve(__dirname, "./src"),
    },
  },
  // Vite options tailored for Tauri development and only applied in `tauri dev` or `tauri build`
  //
  // 1. prevent vite from obscuring rust errors
  clearScreen: false,
  // 2. tauri expects a fixed port, fail if that port is not available
  server: {
    port: 1420,
    // strictPort: true,
    proxy: {
      "/api": {
        target: "http://127.0.0.1:10081/",
        changeOrigin: true,
      },
    },
    host: "0.0.0.0",
    hmr: true,
  },
  // 3. to make use of `TAURI_DEBUG` and other env variables
  // https://tauri.app/v1/api/config#buildconfig.beforedevcommand
  envPrefix: ["VITE_", "TAURI_"],
  define:{
    'process.env':{
      VUE_ROUTER_MODE:'hash',
      VUE_ROUTER_BASE:'\\',
    }
  }
}));
