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

const app = createApp(App)
app.use(createPinia())
app.use(Antd)
app.use(i18n)
app.use(router)
app.mount("#app")
