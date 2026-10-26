<template>
  <!-- 侧边栏：磨砂玻璃悬浮面板 -->
  <aside
    id="app-sidebar"
    class="app-sidebar glass-panel flex shrink-0 flex-col overflow-hidden"
    :class="{ 'is-collapsed': appStore.sidebarCollapsed }"
    :aria-label="t('sidebar.navigation')"
  >
    <!-- 菜单区 -->
    <div class="flex-1 overflow-y-auto p-2">
      <a-menu class="aside-menu" mode="inline" :inline-collapsed="appStore.sidebarCollapsed" :selected-keys="selectedKeys" :items="menuItems" @click="onMenuClick" />
    </div>

    <!-- 底部系统信息区（与菜单之间有间隔） -->
    <div class="system-panel mt-2 border-t px-2 pb-2 pt-3">
      <div v-if="!appStore.sidebarCollapsed" class="px-1">
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

      <a-tooltip v-else placement="right" :title="`${t('system.cpu')}: ${cpu}% · ${t('system.memory')}: ${memory}% · ${statusText}`">
        <div class="flex h-10 items-center justify-center" tabindex="0" :aria-label="`${t('system.cpu')}: ${cpu}% · ${t('system.memory')}: ${memory}% · ${statusText}`">
          <span class="h-2 w-2 rounded-full" :class="statusDotClass" />
        </div>
      </a-tooltip>

      <!-- 设置入口（位于运行状态下方） -->
      <div class="system-divider mt-3 border-t pt-2">
        <a-menu
          class="aside-menu"
          mode="inline"
          :inline-collapsed="appStore.sidebarCollapsed"
          :selected-keys="settingSelectedKeys"
          :items="settingItems"
          @click="onMenuClick"
        />
        <a-tooltip placement="right" :title="collapseLabel">
          <a-button
            class="sidebar-toggle"
            type="text"
            :aria-label="collapseLabel"
            :aria-expanded="!appStore.sidebarCollapsed"
            aria-controls="app-sidebar"
            @click="appStore.sidebarCollapsed = !appStore.sidebarCollapsed"
          >
            <template #icon>
              <MenuUnfoldOutlined v-if="appStore.sidebarCollapsed" />
              <MenuFoldOutlined v-else />
            </template>
            <span v-if="!appStore.sidebarCollapsed">{{ collapseLabel }}</span>
          </a-button>
        </a-tooltip>
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
  MenuFoldOutlined,
  MenuUnfoldOutlined,
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
const collapseLabel = computed(() => t(appStore.sidebarCollapsed ? 'sidebar.expand' : 'sidebar.collapse'))

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
.app-sidebar {
  width: 224px;
  transition: width 200ms ease;

  &.is-collapsed {
    width: 72px;
  }
}

.sidebar-toggle {
  width: 100%;
  height: 44px;
  border: 0;
  border-radius: 10px;
}

/* 展开时沿用菜单项的左内边距，让折叠操作与“设置”入口排成同一列。 */
.app-sidebar:not(.is-collapsed) .sidebar-toggle {
  justify-content: flex-start;
  gap: 10px;
  padding-inline: 24px;

  :deep(.anticon + span) {
    margin-inline-start: 0;
  }
}

/* 折叠时隐藏文字后，按钮和菜单图标都回到侧栏中心。 */
.app-sidebar.is-collapsed .sidebar-toggle {
  justify-content: center;
  padding-inline: 0;
}

.sidebar-toggle:focus-visible {
  outline: 2px solid #6366f1;
  outline-offset: -2px;
}

.is-collapsed .aside-menu {
  width: 100%;
  border-inline-end: none;

  :deep(.ant-menu-item) {
    display: flex;
    align-items: center;
    justify-content: center;
    padding-inline: 0 !important;
    height: 44px;
    line-height: 44px;
  }

  :deep(.ant-menu-item-icon) {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    flex: 0 0 16px;
    margin: 0;
    font-size: 16px;
    line-height: 1;
  }

  :deep(.ant-menu-title-content) {
    display: none;
  }
}

@media (prefers-reduced-motion: reduce) {
  .app-sidebar {
    transition: none;
  }
}

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
