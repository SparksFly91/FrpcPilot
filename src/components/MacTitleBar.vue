<template>
  <!-- macOS 自定义标题栏 -->
  <div
    class="titlebar flex h-12 shrink-0 items-center justify-between pl-4 pr-2 select-none"
    data-tauri-drag-region
  >
    <!-- 左侧：红绿灯 -->
    <div class="group/lights flex items-center gap-2" data-tauri-drag-region>
      <button
        class="traffic-light bg-[#ff5f57]"
        :class="{ 'traffic-light--idle': !focused }"
        :title="t('titlebar.close')"
        @click="closeWindow()"
      >
        <svg viewBox="0 0 12 12" class="light-icon opacity-0 group-hover/lights:opacity-100">
          <path d="M3.5 3.5l5 5M8.5 3.5l-5 5" />
        </svg>
      </button>
      <button
        class="traffic-light bg-[#febc2e]"
        :class="{ 'traffic-light--idle': !focused }"
        :title="t('titlebar.minimize')"
        @click="minimizeWindow()"
      >
        <svg viewBox="0 0 12 12" class="light-icon opacity-0 group-hover/lights:opacity-100">
          <path d="M3 6h6" />
        </svg>
      </button>
      <button
        class="traffic-light bg-[#28c840]"
        :class="{ 'traffic-light--idle': !focused }"
        :title="t('titlebar.maximize')"
        @click="toggleMaximizeWindow()"
      >
        <svg viewBox="0 0 12 12" class="light-icon opacity-0 group-hover/lights:opacity-100">
          <path d="M3.5 3.5h5v5h-5z" />
        </svg>
      </button>
    </div>

    <!-- 右侧：语言 / 主题 / 设置 / 最小化 / 关闭 -->
    <TitleBarActions @settings="emit('settings')" />
  </div>
</template>

<script setup lang="ts">
import { useI18n } from "vue-i18n"
import TitleBarActions from "./TitleBarActions.vue"
import { closeWindow, minimizeWindow, toggleMaximizeWindow } from "@/utils/window"

const emit = defineEmits<{ settings: [] }>()

const { t } = useI18n()
const focused = useWindowFocus()
</script>

<style scoped lang="scss">
.traffic-light {
  display: inline-flex;
  height: 12px;
  width: 12px;
  align-items: center;
  justify-content: center;
  border: none;
  border-radius: 9999px;
  padding: 0;
  cursor: pointer;
  outline: none;
  transition: filter 0.15s ease;
  filter: saturate(0.85);

  &:active {
    filter: brightness(0.85);
  }
}

/* 窗口失焦时红绿灯置灰（模拟 macOS 行为） */
.traffic-light--idle {
  background-color: #cbd5e1 !important;
}

.light-icon {
  height: 8px;
  width: 8px;
  stroke: rgba(0, 0, 0, 0.6);
  stroke-width: 1.4;
  stroke-linecap: round;
  stroke-linejoin: round;
  fill: none;
  transition: opacity 0.15s ease;
}
</style>
