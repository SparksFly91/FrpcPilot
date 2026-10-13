<template>
  <div class="flex h-full flex-col gap-3">
    <!-- 顶部操作栏（独立磨砂卡片） -->
    <div class="glass-panel glass-panel-flat flex shrink-0 items-center justify-between gap-3 px-5 py-3">
      <h2 class="text-lg font-semibold">{{ t("menu.download") }}</h2>
    </div>

    <!-- 内容区（独立磨砂卡片） -->
    <div class="glass-panel glass-panel-flat min-h-0 flex-1 overflow-y-auto p-5">
      <a-result status="info" :sub-title="t('common.developing')" />
    </div>
  </div>
</template>

<script setup lang="ts">
import { useI18n } from "vue-i18n"

const { t } = useI18n()
</script>
