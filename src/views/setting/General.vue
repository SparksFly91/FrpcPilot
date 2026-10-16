<template>
  <div class="max-w-2xl">
    <h2 class="mb-1 text-lg font-semibold">{{ t("settings.general.title") }}</h2>
    <p class="mb-6 text-xs opacity-60">{{ t("settings.general.subtitle") }}</p>

    <!-- 关闭行为：与关闭弹窗内容保持一致，可在此再次修改 -->
    <section class="mb-6">
      <div class="mb-1 text-sm font-medium">{{ t("settings.general.closeBehavior") }}</div>
      <p class="mb-3 text-xs opacity-60">{{ t("settings.general.closeBehaviorTip") }}</p>
      <CardRadioGroup v-model="windowStore.closeBehavior" :options="behaviorOptions" />

      <!-- 关闭方式：与关闭弹窗一致，可随时修改；选择「保持以上选择」后关闭时直接生效 -->
      <div class="mt-4 mb-1 text-sm font-medium">{{ t("settings.general.closeAction") }}</div>
      <p class="mb-3 text-xs opacity-60">{{ t("settings.general.closeActionTip") }}</p>
      <CardRadioGroup v-model="windowStore.closeChoice" :options="choiceOptions" />
    </section>

    <a-divider />

    <!-- 语言：与顶部栏语言切换共用同一状态 -->
    <section>
      <div class="mb-1 text-sm font-medium">{{ t("settings.general.language") }}</div>
      <p class="mb-3 text-xs opacity-60">{{ t("settings.general.languageTip") }}</p>
      <a-radio-group
        v-model:value="appStore.language"
        :options="languageRadios"
        option-type="button"
        button-style="solid"
      />
    </section>
  </div>
</template>

<script setup lang="ts">
import { useI18n } from "vue-i18n"
import { languageOptions } from "@/i18n"
import { useAppStore } from "@/stores/app"
import { useWindowStore } from "@/stores/window"

const { t } = useI18n()
const appStore = useAppStore()
const windowStore = useWindowStore()

const behaviorOptions = computed(() => [
  { value: "ask" as const, label: t("closeDialog.ask"), description: t("closeDialog.askDesc") },
  {
    value: "remember" as const,
    label: t("closeDialog.remember"),
    description: t("closeDialog.rememberDesc"),
  },
])

const choiceOptions = computed(() => [
  { value: "tray" as const, label: t("closeDialog.tray"), description: t("closeDialog.trayDesc") },
  { value: "exit" as const, label: t("closeDialog.exit"), description: t("closeDialog.exitDesc") },
])

/** 语言选项与顶部栏共用 languageOptions，保证两端一致 */
const languageRadios = languageOptions.map((item) => ({ label: item.label, value: item.key }))
</script>
