import { createRouter, createWebHistory } from "vue-router";
import { RouteRecordRaw } from "vue-router";

const routes: RouteRecordRaw[] = [
    {
        path: "/",
        name: "Layout",
        component: () => import("@/layout/Layout.vue"),
    }
]

const router = createRouter({
  history: createWebHistory(),
  routes,
})

export default router