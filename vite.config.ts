import { defineConfig } from "vite"
import vue from "@vitejs/plugin-vue"
import process from "node:process"
const host = process.env.TAURI_DEV_HOST
import tailwindcss from "@tailwindcss/vite" // 引入插件
import path from "path"
import AutoImport from "unplugin-auto-import/vite"
import Components from "unplugin-vue-components/vite"
import { AntDesignVueResolver } from "unplugin-vue-components/resolvers"

// https://vite.dev/config/
export default defineConfig(() => ({
  plugins: [
    vue(),
    tailwindcss(),
    AutoImport({
      imports: ["vue", "vue-router", "pinia", "@vueuse/core"],
      dts: "src/types/auto-imports.d.ts",
      resolvers: [
        AntDesignVueResolver({
          importStyle: false, // Ant Design Vue 4.x 推荐关闭，使用 CSS-in-JS
        }),
      ],
    }),
    Components({
      resolvers: [
        AntDesignVueResolver({
          importStyle: false, // Ant Design Vue 4.x 推荐关闭，使用 CSS-in-JS
          resolveIcons: true, // 自动导入图标组件（如 <CheckOutlined />）
        }),
      ],
      dts: "src/types/components.d.ts",
    }),
  ],

  // Vite options tailored for Tauri development and only applied in `tauri dev` or `tauri build`
  //
  // 1. prevent Vite from obscuring rust errors
  clearScreen: false,
  // 2. tauri expects a fixed port, fail if that port is not available
  server: {
    port: 1420,
    strictPort: true,
    host: host || false,
    hmr: host
      ? {
          protocol: "ws",
          host,
          port: 1421,
        }
      : undefined,
    watch: {
      // 3. tell Vite to ignore watching `src-tauri`
      ignored: ["**/src-tauri/**"],
    },
  },
  resolve: {
    alias: {
      "@": path.resolve(__dirname, "src"),
    },
  },
}))
