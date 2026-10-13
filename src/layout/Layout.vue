<template>
  <!-- 应用整体：背景由 <html> 统一绘制，此处只负责布局 -->
  <div class="app-shell flex h-full w-full flex-col overflow-hidden">
    <!-- 顶部自定义 TitleBar -->
    <Header @settings="goToSetting" />

    <!-- 主体：侧边栏 + 内容区 -->
    <div class="flex min-h-0 flex-1 gap-3 px-3 pb-3">
      <Aside />

      <main class="glass-panel min-w-0 flex-1 overflow-y-auto">
        <router-view />
      </main>
    </div>
  </div>
</template>

<script setup lang="ts">
import Header from "./header/Header.vue"
import Aside from "./aside/Aside.vue"

const router = useRouter()

/** 点击标题栏“设置”按钮 → 跳转设置页 */
function goToSetting() {
  router.push("/setting")
}
</script>
