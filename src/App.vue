<template>
  <a-config-provider :locale="appStore.antdLocale" :theme="themeConfig">
    <router-view />
  </a-config-provider>
</template>

<script setup lang="ts">
import { theme as antdTheme } from "ant-design-vue"
import { useAppStore } from "@/stores/app"
import { useFrpcStore } from "@/stores/frpc"

const appStore = useAppStore()
const frpcStore = useFrpcStore()

/** 跟随明暗主题切换 Ant Design Vue 的算法 */
const themeConfig = computed(() => ({
  algorithm: appStore.isDark ? antdTheme.darkAlgorithm : antdTheme.defaultAlgorithm,
  token: {
    colorPrimary: "#6366f1",
    borderRadius: 8,
  },
}))

// 应用根组件初始化一次：订阅 frpc 日志/退出事件，离开仪表盘后仍持续收集
onMounted(() => {
  frpcStore.init()
})
</script>

<style lang="scss" scoped></style>
