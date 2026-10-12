import { createApp } from "vue"
import App from "./App.vue"
import "@/assets/main.css"
// 引入 Ant Design Vue 组件库
import Antd from "ant-design-vue"
import "ant-design-vue/dist/reset.css"
// 引入路由
import router from "@/router/router"

const app = createApp(App)
app.use(Antd)
app.use(router)
app.mount("#app")
