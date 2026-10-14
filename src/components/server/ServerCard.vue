<template>
  <a-card class="glass-card" :body-style="{ padding: '16px' }" hoverable>
    <!-- 头部：名称 + 状态标签 + 操作 -->
    <div class="flex items-start justify-between gap-3">
      <div class="min-w-0 flex-1">
        <div class="flex flex-wrap items-center gap-2">
          <span class="truncate text-base font-semibold">{{ server.name }}</span>
          <a-tag v-if="server.pinned" color="gold" class="!me-0">{{ t("server.pinned") }}</a-tag>
          <a-tag v-if="server.enabled" color="green" class="!me-0">{{ t("server.enabled") }}</a-tag>
        </div>
        <div class="mt-1 truncate font-mono text-sm opacity-70">
          {{ server.addr }}:{{ server.port }}
        </div>
      </div>

      <div class="flex shrink-0 items-center gap-1">
        <a-tooltip :title="t('server.edit')">
          <a-button type="text" size="small" :icon="h(EditOutlined)" @click="emit('edit')" />
        </a-tooltip>
        <a-popconfirm
          :title="t('server.deleteConfirm', { name: server.name })"
          :ok-text="t('common.confirm')"
          :cancel-text="t('common.cancel')"
          @confirm="emit('delete')"
        >
          <a-button type="text" size="small" danger :icon="h(DeleteOutlined)" />
        </a-popconfirm>
      </div>
    </div>

    <!-- 元信息 -->
    <div class="mt-3 flex flex-wrap items-center gap-2">
      <a-tag color="blue" class="!me-0 uppercase">{{ server.protocol }}</a-tag>
      <a-tag v-if="server.tls_enable" color="cyan" class="!me-0">TLS</a-tag>
      <a-tag v-if="server.token" color="purple" class="!me-0">Token</a-tag>
    </div>

    <p v-if="server.remark" class="mt-2 line-clamp-2 text-xs opacity-60">{{ server.remark }}</p>

    <a-divider class="!my-3" />

    <!-- 底部开关 -->
    <div class="flex items-center justify-end gap-6">
      <label class="flex cursor-pointer items-center gap-2 text-xs">
        <a-switch
          size="small"
          :checked="server.pinned"
          @change="onPinnedChange"
        />
        <span>{{ t("server.pinned") }}</span>
      </label>
      <label class="flex cursor-pointer items-center gap-2 text-xs">
        <a-switch
          size="small"
          :checked="server.enabled"
          @change="onEnabledChange"
        />
        <span>{{ t("server.enabled") }}</span>
      </label>
    </div>
  </a-card>
</template>

<script setup lang="ts">
import { h } from "vue"
import { DeleteOutlined, EditOutlined } from "@ant-design/icons-vue"
import { useI18n } from "vue-i18n"
import type { Server } from "@/types/server"

defineProps<{ server: Server }>()

const emit = defineEmits<{
  edit: []
  delete: []
  toggleEnabled: [value: boolean]
  togglePinned: [value: boolean]
}>()

const { t } = useI18n()

type SwitchValue = boolean | string | number

function onPinnedChange(checked: SwitchValue) {
  emit("togglePinned", Boolean(checked))
}

function onEnabledChange(checked: SwitchValue) {
  emit("toggleEnabled", Boolean(checked))
}
</script>
