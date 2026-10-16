<template>
  <!-- Windows 自定义标题栏 -->
  <div class="titlebar flex h-12 shrink-0 items-center justify-between pl-4 pr-2 select-none" data-tauri-drag-region>
    <!-- 左侧：图标 + 标题 -->
    <div class="flex items-center gap-2" data-tauri-drag-region>
      <img
        class="h-[22px] w-[22px] shrink-0 object-contain"
        :src="appIcon"
        alt=""
        draggable="false"
      />
      <span class="text-sm font-semibold tracking-wide">FrpcPilot</span>
    </div>

    <!-- 右侧：语言 / 主题 / 设置 / 最小化 / 关闭（macOS 下隐藏最小化与关闭） -->
    <TitleBarActions @settings="emit('settings')" />
  </div>
</template>

<script setup lang="ts">
import TitleBarActions from "./TitleBarActions.vue"
import appIcon from "@/assets/images/app-icon.webp"

const emit = defineEmits<{ settings: [] }>()
</script>
