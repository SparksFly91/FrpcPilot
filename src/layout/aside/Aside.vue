<template>
  <!-- 侧边栏：磨砂玻璃悬浮面板 -->
  <aside class="glass-panel flex w-56 shrink-0 flex-col overflow-hidden">
    <!-- 菜单区 -->
    <div class="flex-1 overflow-y-auto p-2">
      <a-menu class="aside-menu" mode="inline" :selected-keys="selectedKeys" :items="menuItems" @click="onMenuClick" />
    </div>

    <!-- 底部系统信息区（与菜单之间有间隔） -->
    <div class="system-panel mt-2 border-t px-2 pb-2 pt-3">
      <div class="px-1">
        <!-- CPU -->
        <div class="mb-2">
          <div class="mb-1 flex items-center justify-between text-xs">
            <span class="opacity-70">{{ t("system.cpu") }}</span>
            <span class="font-medium tabular-nums">{{ cpu }}%</span>
          </div>
          <a-progress :percent="cpu" :show-info="false" :stroke-color="barColor(cpu)" size="small" />
        </div>

        <!-- 内存 -->
        <div class="mb-3">
          <div class="mb-1 flex items-center justify-between text-xs">
            <span class="opacity-70">{{ t("system.memory") }}</span>
            <span class="font-medium tabular-nums">{{ memory }}%</span>
          </div>
          <a-progress :percent="memory" :show-info="false" :stroke-color="barColor(memory)" size="small" />
        </div>

        <!-- 运行状态 -->
        <div class="flex items-center gap-2 text-xs">
          <span class="h-2 w-2 shrink-0 rounded-full" :class="statusDotClass" />
          <span class="opacity-70">{{ t("system.status") }}</span>
          <span class="font-medium">{{ statusText }}</span>
        </div>
      </div>

      <!-- 设置入口（位于运行状态下方） -->
      <div class="system-divider mt-3 border-t pt-2">
        <a-menu
          class="aside-menu"
          mode="inline"
          :selected-keys="settingSelectedKeys"
          :items="settingItems"
          @click="onMenuClick"
        />
      </div>
    </div>
  </aside>
</template>

<script setup lang="ts">
import { h } from "vue"
import {
  ApiOutlined,
  CloudServerOutlined,
  DashboardOutlined,
  DownloadOutlined,
  SettingOutlined,
} from "@ant-design/icons-vue"
import { useI18n } from "vue-i18n"
import type { MenuProps } from "ant-design-vue"
import { useAppStore } from "@/stores/app"
import { useSystemStats } from "@/composables/useSystemStats"

const { t } = useI18n()
const route = useRoute()
const router = useRouter()
const appStore = useAppStore()

const { cpu, memory } = useSystemStats()

const selectedKeys = computed(() => [route.path])

/** 设置项在运行状态下方独立渲染，子路由（如 /setting/appearance）同样高亮 */
const settingSelectedKeys = computed(() => (route.path.startsWith("/setting") ? ["/setting"] : []))

const menuItems = computed<MenuProps["items"]>(() => [
  { key: "/dashboard", icon: () => h(DashboardOutlined), label: t("menu.dashboard") },
  { key: "/servers", icon: () => h(CloudServerOutlined), label: t("menu.servers") },
  { key: "/tunnels", icon: () => h(ApiOutlined), label: t("menu.tunnels") },
  { key: "/download", icon: () => h(DownloadOutlined), label: t("menu.download") },
])

const settingItems = computed<MenuProps["items"]>(() => [
  { key: "/setting", icon: () => h(SettingOutlined), label: t("menu.setting") },
])

const onMenuClick: MenuProps["onClick"] = ({ key }) => {
  router.push(String(key))
}

/** 进度条颜色：低=绿，中=蓝，高=橙/红 */
function barColor(value: number) {
  if (value >= 85) return "#ef4444"
  if (value >= 60) return "#f59e0b"
  return "#6366f1"
}

const statusDotClass = computed(() => {
  switch (appStore.runStatus) {
    case "running":
      return "bg-green-500 shadow-[0_0_6px_#22c55e]"
    case "error":
      return "bg-red-500 shadow-[0_0_6px_#ef4444]"
    default:
      return "bg-yellow-400"
  }
})

const statusText = computed(() => {
  switch (appStore.runStatus) {
    case "running":
      return t("system.statusRunning")
    case "error":
      return t("system.statusError")
    default:
      return t("system.statusStopped")
  }
})
</script>

<style scoped lang="scss">
.system-panel {
  border-color: rgba(15, 23, 42, 0.08);
}

.dark .system-panel {
  border-color: rgba(255, 255, 255, 0.08);
}

/* 系统信息与设置入口之间的分隔线 */
.system-divider {
  border-color: rgba(15, 23, 42, 0.06);
}

.dark .system-divider {
  border-color: rgba(255, 255, 255, 0.06);
}

/* 让 Antd 菜单透明化，融入磨砂玻璃面板 */
.aside-menu {
  background: transparent;

  :deep(.ant-menu-item) {
    border-radius: 10px;
    margin-inline: 0;
    width: 100%;
  }

  &.ant-menu-inline {
    border-inline-end: none;
  }
}
</style>
