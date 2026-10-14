import { createRouter, createWebHistory } from "vue-router";
import { RouteRecordRaw } from "vue-router";

const routes: RouteRecordRaw[] = [
    {
        path: "/",
        name: "Layout",
        component: () => import("@/layout/Layout.vue"),
        redirect: "/dashboard",
        children: [
            {
                path: "dashboard",
                name: "Dashboard",
                component: () => import("@/views/Dashboard.vue"),
                meta: { title: "menu.dashboard" },
            },
            {
                path: "servers",
                name: "Servers",
                component: () => import("@/views/Servers.vue"),
                meta: { title: "menu.servers" },
            },
            {
                path: "tunnels",
                name: "Tunnels",
                component: () => import("@/views/Tunnels.vue"),
                meta: { title: "menu.tunnels" },
            },
            {
                path: "download",
                name: "Download",
                component: () => import("@/views/Download.vue"),
                meta: { title: "menu.download" },
            },
            {
                path: "setting",
                name: "Setting",
                component: () => import("@/views/Setting.vue"),
                redirect: "/setting/appearance",
                meta: { title: "settings.title" },
                children: [
                    {
                        path: "appearance",
                        name: "Appearance",
                        component: () => import("@/views/setting/Appearance.vue"),
                        meta: { title: "settings.menu.appearance" },
                    },
                    {
                        path: "about",
                        name: "About",
                        component: () => import("@/views/setting/About.vue"),
                        meta: { title: "settings.menu.about" },
                    },
                ],
            },
        ],
    },
]

const router = createRouter({
  history: createWebHistory(),
  routes,
})

export default router
