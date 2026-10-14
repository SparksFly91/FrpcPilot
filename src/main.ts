import { createApp } from "vue"
import { createPinia } from "pinia"
import App from "./App.vue"
import "@/assets/main.css"
// 引入 Ant Design Vue 组件库
import Antd from "ant-design-vue"
import "ant-design-vue/dist/reset.css"
// 引入路由
import router from "@/router/router"
// 引入国际化
import i18n from "@/i18n"
import { detectPlatform } from "@/composables/usePlatform"

/*
 * macOS 的无边框窗口系统不会自动裁圆角，需要配合 transparent 窗口，
 * 由 #app 自身裁出 8px 圆角（见 assets/main.css）。这里在挂载前打上平台标记，
 * 避免首帧出现方角闪烁。
 */
if (detectPlatform() === "macos") {
  document.documentElement.classList.add("is-macos")
}

const app = createApp(App)
app.use(createPinia())
app.use(Antd)
app.use(i18n)
app.use(router)
app.mount("#app")
