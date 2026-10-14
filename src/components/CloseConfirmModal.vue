<template>
  <a-modal
    :open="store.closeConfirmOpen"
    :title="t('closeDialog.title')"
    :width="380"
    centered
    :mask-closable="false"
    :closable="!store.quitting"
    :keyboard="!store.quitting"
    @cancel="store.cancelClose()"
  >
    <a-radio-group v-model:value="store.closeChoice" class="close-choice">
      <a-radio value="tray" class="close-option">
        <span class="close-option-title">{{ t("closeDialog.tray") }}</span>
        <span class="close-option-desc">{{ t("closeDialog.trayDesc") }}</span>
      </a-radio>
      <a-radio value="exit" class="close-option">
        <span class="close-option-title">{{ t("closeDialog.exit") }}</span>
        <span class="close-option-desc">{{ t("closeDialog.exitDesc") }}</span>
      </a-radio>
    </a-radio-group>

    <template #footer>
      <a-button :disabled="store.quitting" @click="store.cancelClose()">
        {{ t("common.cancel") }}
      </a-button>
      <a-button type="primary" :loading="store.quitting" @click="store.confirmClose()">
        {{ t("common.confirm") }}
      </a-button>
    </template>
  </a-modal>
</template>

<script setup lang="ts">
import { useI18n } from "vue-i18n"
import { useWindowStore } from "@/stores/window"

const { t } = useI18n()
const store = useWindowStore()
</script>

<style scoped lang="scss">
.close-choice {
  display: flex;
  flex-direction: column;
  gap: 10px;
  width: 100%;
}

.close-option {
  display: flex;
  align-items: flex-start;
  width: 100%;
  margin-inline-end: 0;
  padding: 10px 12px;
  border-radius: 10px;
  border: 1px solid rgba(15, 23, 42, 0.1);
  transition: border-color 0.2s ease, background-color 0.2s ease;

  &:hover {
    border-color: rgba(99, 102, 241, 0.5);
  }

  // 选中态高亮
  &.ant-radio-wrapper-checked {
    border-color: #6366f1;
    background-color: rgba(99, 102, 241, 0.08);
  }

  :deep(.ant-radio) {
    margin-top: 2px;
  }

  :deep(> span:last-child) {
    display: flex;
    flex-direction: column;
    gap: 2px;
    padding-inline-end: 0;
  }
}

.dark .close-option {
  border-color: rgba(255, 255, 255, 0.12);

  &.ant-radio-wrapper-checked {
    border-color: #6366f1;
    background-color: rgba(99, 102, 241, 0.16);
  }
}

.close-option-title {
  font-weight: 500;
}

.close-option-desc {
  font-size: 12px;
  line-height: 1.4;
  opacity: 0.6;
}
</style>
