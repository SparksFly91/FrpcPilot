<template>
  <div class="flex h-full flex-col p-6">
    <h2 class="mb-4 text-lg font-semibold">{{ t("menu.download") }}</h2>
    <a-result status="info" :sub-title="t('common.developing')" />
  </div>
</template>

<script setup lang="ts">
import { useI18n } from "vue-i18n"

const { t } = useI18n()
</script>
