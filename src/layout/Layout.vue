<template>
  <!-- 应用整体：背景（渐变 / 图片） + 悬浮面板 -->
  <div class="app-shell flex h-screen w-screen flex-col overflow-hidden" :style="shellStyle">
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
import { useAppStore } from "@/stores/app"

const router = useRouter()
const appStore = useAppStore()

/** 点击标题栏“设置”按钮 → 跳转设置页 */
function goToSetting() {
  router.push("/setting")
}

/**
 * 选中背景图片时覆盖默认渐变背景。
 * `cover` 让图片按原始比例缩放铺满窗口，不会变形拉伸。
 */
const shellStyle = computed(() =>
  appStore.backgroundUrl
    ? {
        backgroundImage: `url('${appStore.backgroundUrl}')`,
        backgroundSize: "cover",
        backgroundPosition: "center",
        backgroundRepeat: "no-repeat",
      }
    : {},
)
</script>

<style scoped lang="scss">
.app-shell {
  background-color: #eef2ff;
  background-image:
    radial-gradient(1200px 600px at 10% -10%, rgba(99, 102, 241, 0.28), transparent 60%),
    radial-gradient(1000px 500px at 110% 10%, rgba(34, 211, 238, 0.28), transparent 55%),
    linear-gradient(160deg, #eef2ff 0%, #f8fafc 45%, #eef7ff 100%);
}

.dark .app-shell {
  background-color: #0b1220;
  background-image:
    radial-gradient(1200px 600px at 10% -10%, rgba(99, 102, 241, 0.35), transparent 60%),
    radial-gradient(1000px 500px at 110% 10%, rgba(14, 165, 233, 0.28), transparent 55%),
    linear-gradient(160deg, #0b1220 0%, #0f172a 45%, #111827 100%);
}
</style>
