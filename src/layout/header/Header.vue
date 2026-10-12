<template>
  <!-- 顶部栏：按操作系统类型加载对应的自定义 TitleBar -->
  <header class="shrink-0">
    <WindowsTitleBar v-if="isWindows" @settings="emit('settings')" />
    <MacTitleBar v-else @settings="emit('settings')" />
  </header>
</template>

<script setup lang="ts">
import WindowsTitleBar from "@/components/WindowsTitleBar.vue"
import MacTitleBar from "@/components/MacTitleBar.vue"
import { usePlatform } from "@/composables/usePlatform"

const emit = defineEmits<{ settings: [] }>()

const { isWindows } = usePlatform()
</script>
