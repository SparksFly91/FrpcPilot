<template>
  <!-- 应用整体：背景渐变 + 悬浮面板 -->
  <div class="app-shell flex h-screen w-screen flex-col overflow-hidden">
    <!-- 顶部自定义 TitleBar -->
    <Header @settings="settingsOpen = true" />

    <!-- 主体：侧边栏 + 内容区 -->
    <div class="flex min-h-0 flex-1 gap-3 px-3 pb-3">
      <Aside />

      <main class="glass-panel min-w-0 flex-1 overflow-y-auto">
        <router-view />
      </main>
    </div>

    <!-- 设置（占位） -->
    <a-modal
      v-model:open="settingsOpen"
      :title="t('settings.title')"
      :footer="null"
      centered
    >
      <a-result status="info" :sub-title="t('settings.placeholder')" />
    </a-modal>
  </div>
</template>

<script setup lang="ts">
import { useI18n } from "vue-i18n"
import Header from "./header/Header.vue"
import Aside from "./aside/Aside.vue"

const { t } = useI18n()
const settingsOpen = ref(false)
</script>

<style scoped lang="scss">
.app-shell {
  background:
    radial-gradient(1200px 600px at 10% -10%, rgba(99, 102, 241, 0.28), transparent 60%),
    radial-gradient(1000px 500px at 110% 10%, rgba(34, 211, 238, 0.28), transparent 55%),
    linear-gradient(160deg, #eef2ff 0%, #f8fafc 45%, #eef7ff 100%);
}

.dark .app-shell {
  background:
    radial-gradient(1200px 600px at 10% -10%, rgba(99, 102, 241, 0.35), transparent 60%),
    radial-gradient(1000px 500px at 110% 10%, rgba(14, 165, 233, 0.28), transparent 55%),
    linear-gradient(160deg, #0b1220 0%, #0f172a 45%, #111827 100%);
}
</style>
