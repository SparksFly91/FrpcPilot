<template>
  <a-modal :open="store.modalOpen" :title="modalTitle" :closable="!store.installing" :mask-closable="false"
    :keyboard="!store.installing" :width="520" centered wrap-class-name="update-modal-wrap" @cancel="onCancel">
    <!-- 俏皮的问候 + 版本变化 -->
    <div class="update-hero">
      <span class="update-hero-emoji">🎉</span>
      <div class="min-w-0 flex-1">
        <p class="m-0 text-sm font-medium">{{ t("update.hello") }}</p>
        <div class="mt-2 flex flex-wrap items-center gap-1.5 text-xs">
          <a-tag class="!me-0">v{{ info?.currentVersion }}</a-tag>
          <ArrowRightOutlined class="text-[10px] opacity-50" />
          <a-tag color="green" class="!me-0">v{{ info?.version }}</a-tag>
          <span v-if="releaseDate" class="ml-1 opacity-50">{{ releaseDate }}</span>
        </div>
      </div>
    </div>

    <p class="update-tip">{{ t("update.tip") }}</p>

    <!-- 更新日志 -->
    <div class="update-log-header">
      <BulbOutlined />
      <span>{{ t("update.log") }}</span>
    </div>
    <div class="update-log">
      <MarkdownBlock v-if="hasLog" :source="info?.body" />
      <p v-else class="m-0 opacity-80">{{ t("update.emptyLog") }}</p>
    </div>

    <!-- 底部操作：忽略该版本 / 取消 / 确认更新 -->
    <template #footer>
      <div class="update-footer">
        <!-- 下载进度固定在 footer 的按钮操作上方 -->
        <div v-if="store.installing" class="update-progress">
          <a-progress :percent="store.progress" :stroke-color="'#6366f1'" />
          <p class="m-0 text-xs opacity-60">{{ t("update.downloading") }}</p>
        </div>
        <div class="update-footer-actions">
          <a-button v-if="!store.installing" type="text" size="small" @click="onIgnore">
            {{ t("update.ignore") }}
          </a-button>
          <div class="ml-auto flex items-center gap-2">
            <a-button :disabled="store.installing" @click="onCancel">
              {{ t("common.cancel") }}
            </a-button>
            <a-button type="primary" :loading="store.installing" @click="onConfirm">
              {{ t("update.confirm") }}
            </a-button>
          </div>
        </div>
      </div>
    </template>
  </a-modal>
</template>

<script setup lang="ts">
import dayjs from "dayjs"
import { ArrowRightOutlined, BulbOutlined } from "@ant-design/icons-vue"
import { useI18n } from "vue-i18n"
import MarkdownBlock from "@/components/MarkdownBlock.vue"
import { useUpdateStore } from "@/stores/update"

const { t } = useI18n()
const store = useUpdateStore()

const info = computed(() => store.info)

const modalTitle = computed(() => t("update.title", { version: info.value?.version ?? "" }))

const releaseDate = computed(() =>
  info.value?.date ? dayjs(info.value.date).format("YYYY-MM-DD") : "",
)

/** 是否存在可渲染的更新日志内容 */
const hasLog = computed(() => Boolean(info.value?.body?.trim()))

function onCancel() {
  store.cancel()
}

function onIgnore() {
  store.ignoreVersion()
}

function onConfirm() {
  store.install()
}
</script>

<style scoped lang="scss">
.update-hero {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 12px 14px;
  border-radius: 12px;
  background: linear-gradient(135deg, rgba(99, 102, 241, 0.14), rgba(34, 211, 238, 0.14));
}

.dark .update-hero {
  background: linear-gradient(135deg, rgba(99, 102, 241, 0.22), rgba(34, 211, 238, 0.18));
}

.update-hero-emoji {
  font-size: 26px;
  line-height: 1;
}

.update-tip {
  margin: 14px 2px 10px;
  font-size: 12px;
  opacity: 0.65;
}

.update-log-header {
  display: flex;
  align-items: center;
  gap: 6px;
  margin-bottom: 6px;
  font-size: 13px;
  font-weight: 600;
}

.update-log {
  word-break: break-word;
  padding: 12px 14px;
  border-radius: 10px;
  border: 1px solid rgba(15, 23, 42, 0.08);
  background: rgba(148, 163, 184, 0.12);
  opacity: 0.9;
}

.dark .update-log {
  border-color: rgba(255, 255, 255, 0.1);
  background: rgba(148, 163, 184, 0.14);
}

.update-progress {
  width: 100%;
}

.update-footer {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.update-footer-actions {
  display: flex;
  align-items: center;
  gap: 8px;
}
</style>
