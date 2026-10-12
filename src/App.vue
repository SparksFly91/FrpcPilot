<template>
  <a-config-provider :locale="appStore.antdLocale" :theme="themeConfig">
    <router-view />
  </a-config-provider>
</template>

<script setup lang="ts">
import { theme as antdTheme } from "ant-design-vue"
import { useAppStore } from "@/stores/app"

const appStore = useAppStore()

/** 跟随明暗主题切换 Ant Design Vue 的算法 */
const themeConfig = computed(() => ({
  algorithm: appStore.isDark ? antdTheme.darkAlgorithm : antdTheme.defaultAlgorithm,
  token: {
    colorPrimary: "#6366f1",
    borderRadius: 8,
  },
}))
</script>

<style lang="scss" scoped></style>
