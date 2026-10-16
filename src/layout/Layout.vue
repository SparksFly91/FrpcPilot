<template>
  <!-- 应用整体：背景由 <html> 统一绘制，此处只负责布局 -->
  <div class="app-shell flex h-full w-full flex-col overflow-hidden">
    <!-- 顶部自定义 TitleBar -->
    <Header @settings="goToSetting" />

    <!-- 主体：侧边栏 + 内容区 -->
    <div class="flex min-h-0 flex-1 gap-3 px-3 pb-3">
      <Aside />

      <main class="min-w-0 flex-1 overflow-hidden">
        <!-- 按路由 meta.isCache 决定是否缓存页面组件（keep-alive 不持久化，仅在应用运行时生效） -->
        <router-view v-slot="{ Component }">
          <keep-alive :include="cachedViews">
            <component :is="Component" />
          </keep-alive>
        </router-view>
      </main>
    </div>

    <!-- 全局更新弹窗 -->
    <UpdateModal />

    <!-- 关闭确认对话框（最小化到系统托盘 / 直接退出） -->
    <CloseConfirmModal />
  </div>
</template>

<script setup lang="ts">
import Header from "./header/Header.vue"
import Aside from "./aside/Aside.vue"
import UpdateModal from "@/components/UpdateModal.vue"
import CloseConfirmModal from "@/components/CloseConfirmModal.vue"
import { useUpdateStore } from "@/stores/update"

const router = useRouter()
const updateStore = useUpdateStore()

/**
 * 需要 keep-alive 缓存的页面组件名列表（由路由 meta.isCache 决定）。
 *
 * keep-alive 通过组件名匹配缓存，故约定：缓存页面的组件名需与路由 name 一致
 * （页面通过 defineOptions({ name }) 显式声明）。
 */
const cachedViews = computed(() =>
  router
    .getRoutes()
    .filter((record) => record.meta.isCache && record.name)
    .map((record) => String(record.name)),
)

/** 点击标题栏“设置”按钮 → 跳转设置页 */
function goToSetting() {
  router.push("/setting")
}

// 应用启动后静默检查一次更新（已忽略的版本不会打扰用户）
onMounted(() => {
  updateStore.checkForUpdate(true)
})
</script>
