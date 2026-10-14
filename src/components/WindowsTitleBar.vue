<template>
  <!-- Windows 自定义标题栏 -->
  <div
    class="titlebar flex h-12 shrink-0 items-center justify-between pl-4 pr-2 select-none"
    data-tauri-drag-region
  >
    <!-- 左侧：标题 -->
    <div class="flex items-center gap-2" data-tauri-drag-region>
      <span class="brand-logo">F</span>
      <span class="text-sm font-semibold tracking-wide">Frpilot</span>
    </div>

    <!-- 右侧：语言 / 主题 / 设置 / 最小化 / 关闭（macOS 下隐藏最小化与关闭） -->
    <TitleBarActions @settings="emit('settings')" />
  </div>
</template>

<script setup lang="ts">
import TitleBarActions from "./TitleBarActions.vue"

const emit = defineEmits<{ settings: [] }>()
</script>

<style scoped lang="scss">
.brand-logo {
  display: inline-flex;
  height: 22px;
  width: 22px;
  align-items: center;
  justify-content: center;
  border-radius: 6px;
  background: linear-gradient(135deg, #6366f1, #22d3ee);
  color: #fff;
  font-size: 12px;
  font-weight: 700;
}
</style>
