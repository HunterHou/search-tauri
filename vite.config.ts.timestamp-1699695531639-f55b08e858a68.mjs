// vite.config.ts
import { defineConfig } from "file:///D:/code/hd/tauri-app/node_modules/vite/dist/node/index.js";
import vue from "file:///D:/code/hd/tauri-app/node_modules/@vitejs/plugin-vue/dist/index.mjs";
import path from "path";
var __vite_injected_original_dirname = "D:\\code\\hd\\tauri-app";
var vite_config_default = defineConfig(async () => ({
  pluginOptions: {
    quasar: {
      importStrategy: "kebab",
      rtlSupport: false
    }
  },
  transpileDependencies: [
    "quasar"
  ],
  plugins: [
    vue()
  ],
  resolve: {
    alias: {
      "@": path.resolve(__vite_injected_original_dirname, "./src")
    }
  },
  // Vite options tailored for Tauri development and only applied in `tauri dev` or `tauri build`
  //
  // 1. prevent vite from obscuring rust errors
  clearScreen: false,
  // 2. tauri expects a fixed port, fail if that port is not available
  server: {
    port: 1420,
    // strictPort: true,
    // proxy: {
    //   "/api": {
    //     target: "http://127.0.0.1:10081/",
    //     changeOrigin: true,
    //   },
    // },
    // host: "0.0.0.0",
    hmr: true
  },
  // 3. to make use of `TAURI_DEBUG` and other env variables
  // https://tauri.app/v1/api/config#buildconfig.beforedevcommand
  envPrefix: ["VITE_", "TAURI_"],
  define: {
    "process.env": {
      VUE_ROUTER_MODE: "hash",
      VUE_ROUTER_BASE: "\\"
    }
  }
}));
export {
  vite_config_default as default
};
//# sourceMappingURL=data:application/json;base64,ewogICJ2ZXJzaW9uIjogMywKICAic291cmNlcyI6IFsidml0ZS5jb25maWcudHMiXSwKICAic291cmNlc0NvbnRlbnQiOiBbImNvbnN0IF9fdml0ZV9pbmplY3RlZF9vcmlnaW5hbF9kaXJuYW1lID0gXCJEOlxcXFxjb2RlXFxcXGhkXFxcXHRhdXJpLWFwcFwiO2NvbnN0IF9fdml0ZV9pbmplY3RlZF9vcmlnaW5hbF9maWxlbmFtZSA9IFwiRDpcXFxcY29kZVxcXFxoZFxcXFx0YXVyaS1hcHBcXFxcdml0ZS5jb25maWcudHNcIjtjb25zdCBfX3ZpdGVfaW5qZWN0ZWRfb3JpZ2luYWxfaW1wb3J0X21ldGFfdXJsID0gXCJmaWxlOi8vL0Q6L2NvZGUvaGQvdGF1cmktYXBwL3ZpdGUuY29uZmlnLnRzXCI7aW1wb3J0IHsgZGVmaW5lQ29uZmlnIH0gZnJvbSBcInZpdGVcIjtcclxuaW1wb3J0IHZ1ZSBmcm9tIFwiQHZpdGVqcy9wbHVnaW4tdnVlXCI7XHJcbmltcG9ydCBwYXRoIGZyb20gXCJwYXRoXCI7XHJcblxyXG5cclxuLy8gaHR0cHM6Ly92aXRlanMuZGV2L2NvbmZpZy9cclxuZXhwb3J0IGRlZmF1bHQgZGVmaW5lQ29uZmlnKGFzeW5jICgpID0+ICh7XHJcbiAgcGx1Z2luT3B0aW9uczoge1xyXG4gICAgcXVhc2FyOiB7XHJcbiAgICAgIGltcG9ydFN0cmF0ZWd5OiAna2ViYWInLFxyXG4gICAgICBydGxTdXBwb3J0OiBmYWxzZVxyXG4gICAgfVxyXG4gIH0sXHJcbiAgdHJhbnNwaWxlRGVwZW5kZW5jaWVzOiBbXHJcbiAgICAncXVhc2FyJ1xyXG4gIF0sXHJcbiAgcGx1Z2luczogW1xyXG4gICAgdnVlKCksXHJcbiAgXSxcclxuIFxyXG4gIHJlc29sdmU6IHtcclxuICAgIGFsaWFzOiB7XHJcbiAgICAgIFwiQFwiOiBwYXRoLnJlc29sdmUoX19kaXJuYW1lLCBcIi4vc3JjXCIpLFxyXG4gICAgfSxcclxuICB9LFxyXG4gIC8vIFZpdGUgb3B0aW9ucyB0YWlsb3JlZCBmb3IgVGF1cmkgZGV2ZWxvcG1lbnQgYW5kIG9ubHkgYXBwbGllZCBpbiBgdGF1cmkgZGV2YCBvciBgdGF1cmkgYnVpbGRgXHJcbiAgLy9cclxuICAvLyAxLiBwcmV2ZW50IHZpdGUgZnJvbSBvYnNjdXJpbmcgcnVzdCBlcnJvcnNcclxuICBjbGVhclNjcmVlbjogZmFsc2UsXHJcbiAgLy8gMi4gdGF1cmkgZXhwZWN0cyBhIGZpeGVkIHBvcnQsIGZhaWwgaWYgdGhhdCBwb3J0IGlzIG5vdCBhdmFpbGFibGVcclxuICBzZXJ2ZXI6IHtcclxuICAgIHBvcnQ6IDE0MjAsXHJcbiAgICAvLyBzdHJpY3RQb3J0OiB0cnVlLFxyXG4gICAgLy8gcHJveHk6IHtcclxuICAgIC8vICAgXCIvYXBpXCI6IHtcclxuICAgIC8vICAgICB0YXJnZXQ6IFwiaHR0cDovLzEyNy4wLjAuMToxMDA4MS9cIixcclxuICAgIC8vICAgICBjaGFuZ2VPcmlnaW46IHRydWUsXHJcbiAgICAvLyAgIH0sXHJcbiAgICAvLyB9LFxyXG4gICAgLy8gaG9zdDogXCIwLjAuMC4wXCIsXHJcbiAgICBobXI6IHRydWUsXHJcbiAgfSxcclxuICAvLyAzLiB0byBtYWtlIHVzZSBvZiBgVEFVUklfREVCVUdgIGFuZCBvdGhlciBlbnYgdmFyaWFibGVzXHJcbiAgLy8gaHR0cHM6Ly90YXVyaS5hcHAvdjEvYXBpL2NvbmZpZyNidWlsZGNvbmZpZy5iZWZvcmVkZXZjb21tYW5kXHJcbiAgZW52UHJlZml4OiBbXCJWSVRFX1wiLCBcIlRBVVJJX1wiXSxcclxuICBkZWZpbmU6e1xyXG4gICAgJ3Byb2Nlc3MuZW52Jzp7XHJcbiAgICAgIFZVRV9ST1VURVJfTU9ERTonaGFzaCcsXHJcbiAgICAgIFZVRV9ST1VURVJfQkFTRTonXFxcXCcsXHJcbiAgICB9XHJcbiAgfVxyXG59KSk7XHJcbiJdLAogICJtYXBwaW5ncyI6ICI7QUFBc1AsU0FBUyxvQkFBb0I7QUFDblIsT0FBTyxTQUFTO0FBQ2hCLE9BQU8sVUFBVTtBQUZqQixJQUFNLG1DQUFtQztBQU16QyxJQUFPLHNCQUFRLGFBQWEsYUFBYTtBQUFBLEVBQ3ZDLGVBQWU7QUFBQSxJQUNiLFFBQVE7QUFBQSxNQUNOLGdCQUFnQjtBQUFBLE1BQ2hCLFlBQVk7QUFBQSxJQUNkO0FBQUEsRUFDRjtBQUFBLEVBQ0EsdUJBQXVCO0FBQUEsSUFDckI7QUFBQSxFQUNGO0FBQUEsRUFDQSxTQUFTO0FBQUEsSUFDUCxJQUFJO0FBQUEsRUFDTjtBQUFBLEVBRUEsU0FBUztBQUFBLElBQ1AsT0FBTztBQUFBLE1BQ0wsS0FBSyxLQUFLLFFBQVEsa0NBQVcsT0FBTztBQUFBLElBQ3RDO0FBQUEsRUFDRjtBQUFBO0FBQUE7QUFBQTtBQUFBLEVBSUEsYUFBYTtBQUFBO0FBQUEsRUFFYixRQUFRO0FBQUEsSUFDTixNQUFNO0FBQUE7QUFBQTtBQUFBO0FBQUE7QUFBQTtBQUFBO0FBQUE7QUFBQTtBQUFBLElBU04sS0FBSztBQUFBLEVBQ1A7QUFBQTtBQUFBO0FBQUEsRUFHQSxXQUFXLENBQUMsU0FBUyxRQUFRO0FBQUEsRUFDN0IsUUFBTztBQUFBLElBQ0wsZUFBYztBQUFBLE1BQ1osaUJBQWdCO0FBQUEsTUFDaEIsaUJBQWdCO0FBQUEsSUFDbEI7QUFBQSxFQUNGO0FBQ0YsRUFBRTsiLAogICJuYW1lcyI6IFtdCn0K
