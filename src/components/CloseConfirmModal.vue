<template>
  <a-modal
    :open="store.closeConfirmOpen"
    :title="t('closeDialog.title')"
    :width="400"
    centered
    :mask-closable="false"
    :closable="!store.quitting"
    :keyboard="!store.quitting"
    @cancel="store.cancelClose()"
  >
    <!-- 关闭方式：最小化到系统托盘 / 直接退出 -->
    <CardRadioGroup v-model="draftChoice" :options="choiceOptions" :disabled="store.quitting" />

    <a-divider class="!my-3" />

    <!-- 关闭行为：每次询问 / 保持以上选择 -->
    <div class="mb-2 text-xs opacity-60">{{ t("closeDialog.behavior") }}</div>
    <CardRadioGroup v-model="draftBehavior" :options="behaviorOptions" :disabled="store.quitting" />

    <template #footer>
      <a-button :disabled="store.quitting" @click="store.cancelClose()">
        {{ t("common.cancel") }}
      </a-button>
      <a-button type="primary" :loading="store.quitting" @click="onConfirm">
        {{ t("common.confirm") }}
      </a-button>
    </template>
  </a-modal>
</template>

<script setup lang="ts">
import { useI18n } from "vue-i18n"
import { useWindowStore, type CloseBehavior, type CloseChoice } from "@/stores/window"

const { t } = useI18n()
const store = useWindowStore()

/** 弹窗内的临时选择，点击「确定」后才写回设置（避免误触单选框又取消时被改掉） */
const draftChoice = ref<CloseChoice>("tray")
const draftBehavior = ref<CloseBehavior>("ask")

const choiceOptions = computed(() => [
  { value: "tray" as const, label: t("closeDialog.tray"), description: t("closeDialog.trayDesc") },
  { value: "exit" as const, label: t("closeDialog.exit"), description: t("closeDialog.exitDesc") },
])

const behaviorOptions = computed(() => [
  { value: "ask" as const, label: t("closeDialog.ask"), description: t("closeDialog.askDesc") },
  {
    value: "remember" as const,
    label: t("closeDialog.remember"),
    description: t("closeDialog.rememberDesc"),
  },
])

// 每次打开弹窗时，用当前设置初始化临时选择
watch(
  () => store.closeConfirmOpen,
  (open) => {
    if (!open) return
    draftChoice.value = store.closeChoice
    draftBehavior.value = store.closeBehavior
  },
)

function onConfirm() {
  void store.confirmClose(draftChoice.value, draftBehavior.value)
}
</script>
