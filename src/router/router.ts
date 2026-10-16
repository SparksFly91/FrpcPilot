import { createRouter, createWebHistory } from "vue-router";
import { RouteRecordRaw } from "vue-router";

/** 路由元信息类型扩展 */
declare module "vue-router" {
    interface RouteMeta {
        /** 页面标题（i18n key） */
        title?: string;
        /** 是否使用 keep-alive 缓存该页面组件（默认不缓存） */
        isCache?: boolean;
    }
}

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
                // 缓存该页面：避免每次进入都请求 GitHub 触发限流，需刷新时由用户手动点击刷新按钮
                meta: { title: "menu.download", isCache: true },
            },
            {
                path: "setting",
                name: "Setting",
                component: () => import("@/views/Setting.vue"),
                redirect: "/setting/general",
                meta: { title: "settings.title" },
                children: [
                    {
                        path: "general",
                        name: "General",
                        component: () => import("@/views/setting/General.vue"),
                        meta: { title: "settings.menu.general" },
                    },
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
