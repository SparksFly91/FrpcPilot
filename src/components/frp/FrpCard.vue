<template>
  <a-card class="glass-card" :body-style="{ padding: '16px' }" hoverable>
    <!-- 头部：版本 + 状态标签 -->
    <div class="flex items-start justify-between gap-3">
      <div class="min-w-0 flex-1">
        <div class="flex flex-wrap items-center gap-2">
          <span class="truncate text-base font-semibold">{{ release.version }}</span>
          <a-tag v-if="release.downloaded" color="green" class="!me-0">{{ t("frp.downloaded") }}</a-tag>
          <a-tag v-if="release.local_only" color="purple" class="!me-0">{{ t("frp.local") }}</a-tag>
        </div>
        <div class="mt-1 flex flex-wrap items-center gap-2">
          <a-tag color="blue" class="!me-0">{{ release.platform }}</a-tag>
          <a-tag class="!me-0">frpc</a-tag>
          <a-tag v-if="showFrps" color="cyan" class="!me-0">frps</a-tag>
        </div>
      </div>
    </div>

    <!-- 元信息：大小 / 下载次数 / 发布日期 -->
    <div class="mt-3 grid grid-cols-3 gap-2 text-center text-xs">
      <div class="flex flex-col gap-1">
        <span class="opacity-60">{{ t("frp.size") }}</span>
        <span class="font-medium tabular-nums">{{ formatSize(release.size) }}</span>
      </div>
      <div class="flex flex-col gap-1">
        <span class="opacity-60">{{ t("frp.downloadCount") }}</span>
        <span class="font-medium tabular-nums">{{ formatCount(release.download_count) }}</span>
      </div>
      <div class="flex flex-col gap-1">
        <span class="opacity-60">{{ t("frp.publishedAt") }}</span>
        <span class="font-medium">{{ formatDate(release.published_at) }}</span>
      </div>
    </div>

    <a-divider class="!my-3" />

    <!-- 操作按钮 -->
    <div class="flex items-center justify-end gap-2">
      <a-button
        v-if="!release.local_only"
        type="primary"
        size="small"
        :icon="h(DownloadOutlined)"
        :loading="downloading"
        :disabled="release.downloaded"
        @click="emit('download')"
      >
        {{ release.downloaded ? t("frp.downloaded") : t("frp.download") }}
      </a-button>

      <a-tooltip v-if="!release.local_only" :title="t('frp.copyLink')">
        <a-button size="small" :icon="h(LinkOutlined)" @click="emit('copy')" />
      </a-tooltip>

      <a-popconfirm
        v-if="release.downloaded"
        :title="t('frp.deleteConfirm', { version: release.version, platform: release.platform })"
        :ok-text="t('common.confirm')"
        :cancel-text="t('common.cancel')"
        @confirm="emit('delete')"
      >
        <a-button size="small" danger :icon="h(DeleteOutlined)" />
      </a-popconfirm>
    </div>

    <!-- 下载进度（卡片下方） -->
    <div v-if="progress" class="mt-3">
      <div class="mb-1 flex items-center justify-between text-xs">
        <span :class="statusClass">{{ statusText }}</span>
        <span class="tabular-nums opacity-70">{{ progress.percent }}%</span>
      </div>
      <a-progress
        :percent="progress.percent"
        :status="progressStatus"
        :stroke-color="progressColor"
        :show-info="false"
        size="small"
      />
      <p v-if="progress.status === 'error' && progress.message" class="mt-1 line-clamp-2 text-xs text-red-500">
        {{ progress.message }}
      </p>
    </div>
  </a-card>
</template>

<script setup lang="ts">
import dayjs from "dayjs"
import { h } from "vue"
import { DeleteOutlined, DownloadOutlined, LinkOutlined } from "@ant-design/icons-vue"
import { useI18n } from "vue-i18n"
import type { FrpDownloadProgress, FrpRelease } from "@/types/frp"

const props = defineProps<{
  release: FrpRelease
  progress?: FrpDownloadProgress | null
  showFrps?: boolean
}>()

const emit = defineEmits<{
  download: []
  copy: []
  delete: []
}>()

const { t } = useI18n()

const downloading = computed(
  () => props.progress?.status === "downloading" || props.progress?.status === "extracting",
)

const progressStatus = computed(() => {
  switch (props.progress?.status) {
    case "error":
      return "exception"
    case "done":
      return "success"
    default:
      return "normal"
  }
})

const progressColor = computed(() => {
  switch (props.progress?.status) {
    case "error":
      return "#ef4444"
    case "done":
      return "#22c55e"
    default:
      return "#6366f1"
  }
})

const statusClass = computed(() =>
  props.progress?.status === "error" ? "text-red-500" : "opacity-70",
)

const statusText = computed(() =>
  props.progress ? t(`frp.status.${props.progress.status}`) : "",
)

/** 字节数格式化 */
function formatSize(bytes: number) {
  if (!bytes) return "-"
  const units = ["B", "KB", "MB", "GB"]
  let value = bytes
  let index = 0
  while (value >= 1024 && index < units.length - 1) {
    value /= 1024
    index += 1
  }
  return `${value.toFixed(index === 0 ? 0 : 1)} ${units[index]}`
}

/** 下载次数格式化（过万显示为 w） */
function formatCount(count: number) {
  if (!count) return "-"
  return count >= 10000 ? `${(count / 10000).toFixed(1)}w` : String(count)
}

/** 发布日期格式化 */
function formatDate(value: string) {
  return value ? dayjs(value).format("YYYY-MM-DD") : "-"
}
</script>
