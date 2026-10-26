<template>
  <div class="max-w-2xl">
    <h2 class="mb-1 text-lg font-semibold">{{ t("settings.general.title") }}</h2>
    <p class="mb-5 text-xs opacity-60">{{ t("settings.general.subtitle") }}</p>

    <!-- 重新开始引导：与引导弹窗一致，可在此再次触发 -->
    <section class="flex items-center justify-between gap-4">
      <div>
        <div class="text-sm font-medium">{{ t('onboarding.title') }}</div>
        <p class="mt-1 text-xs opacity-60">{{ t('onboarding.replayDescription') }}</p>
      </div>
      <a-button class="shrink-0" @click="onboarding.start">{{ t('onboarding.replay') }}</a-button>
    </section>

    <a-divider class="general-divider" />

    <!-- 语言：与顶部栏语言切换共用同一状态 -->
    <section class="flex items-center justify-between gap-4">
      <div class="min-w-0">
        <div class="text-sm font-medium">{{ t("settings.general.language") }}</div>
        <p class="mt-1 text-xs opacity-60">{{ t("settings.general.languageTip") }}</p>
      </div>
      <a-select
        v-model:value="appStore.language"
        :options="languageSelectOptions"
        :aria-label="t('settings.general.language')"
        class="w-36 shrink-0"
      />
    </section>
    
    <a-divider class="general-divider" />

    <!-- 开机自启：开关直接写入系统启动项，状态由系统决定 -->
    <section>
      <div class="flex items-center justify-between gap-4">
        <div class="min-w-0">
          <div class="text-sm font-medium">{{ t("settings.general.autostart") }}</div>
          <p class="mt-1 text-xs opacity-60">{{ autostartTip }}</p>
        </div>
        <a-switch
          class="shrink-0"
          :checked="autostartStore.enabled"
          :loading="autostartStore.loading"
          :disabled="!autostartStore.supported"
          @change="onAutostartChange"
        />
      </div>
    </section>

    <a-divider class="general-divider" />

    <!-- 关闭行为：与关闭弹窗内容保持一致，可在此再次修改 -->
    <section>
      <div class="mb-1 text-sm font-medium">{{ t("settings.general.closeBehavior") }}</div>
      <p class="mb-2 text-xs opacity-60">{{ t("settings.general.closeBehaviorTip") }}</p>
      <CardRadioGroup v-model="windowStore.closeBehavior" :options="behaviorOptions" />

      <!-- 关闭方式：与关闭弹窗一致，可随时修改；选择「保持以上选择」后关闭时直接生效 -->
      <div class="mt-3 mb-1 text-sm font-medium">{{ t("settings.general.closeAction") }}</div>
      <p class="mb-2 text-xs opacity-60">{{ t("settings.general.closeActionTip") }}</p>
      <CardRadioGroup v-model="windowStore.closeChoice" :options="choiceOptions" />
    </section>

  </div>
</template>

<script setup lang="ts">
import { useI18n } from "vue-i18n"
import { languageOptions } from "@/i18n"
import { useAppStore } from "@/stores/app"
import { useAutostartStore } from "@/stores/autostart"
import { useWindowStore } from "@/stores/window"
import { useOnboardingStore } from "@/stores/onboarding"

const { t } = useI18n()
const appStore = useAppStore()
const autostartStore = useAutostartStore()
const windowStore = useWindowStore()
const onboarding = useOnboardingStore()

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
const languageSelectOptions = languageOptions.map((item) => ({ label: item.label, value: item.key }))

/** 无法访问系统启动项时（非桌面环境）给出说明，避免开关无故置灰 */
const autostartTip = computed(() =>
  autostartStore.requiresRelease
    ? t("settings.general.autostartRequiresRelease")
    : autostartStore.supported
      ? t("settings.general.autostartTip")
      : t("settings.general.autostartUnsupported"),
)

/** 切换开机自启：失败时由 store 回滚开关状态并提示原因 */
async function onAutostartChange(checked: boolean | string | number) {
  await autostartStore.toggle(Boolean(checked))
}

// 每次进入设置页都读一次系统启动项，用户可能在系统设置里改过
onMounted(() => {
  void autostartStore.refresh()
})
</script>

<style scoped>
/* 各组设置之间留白收紧，避免分隔线两侧过于松散 */
.general-divider {
  margin: 16px 0;
}
</style>
