<template>
  <a-card class="tunnel-card" :body-style="{ padding: '16px' }" hoverable>
    <!-- 头部：名称 + 状态标签 + 操作 -->
    <div class="flex items-start justify-between gap-3">
      <div class="min-w-0 flex-1">
        <div class="flex flex-wrap items-center gap-2">
          <span class="truncate text-base font-semibold">{{ tunnel.name }}</span>
          <a-tag v-if="tunnel.pinned" color="gold" class="!me-0">{{ t("tunnel.pinned") }}</a-tag>
          <a-tag v-if="tunnel.enabled" color="green" class="!me-0">{{ t("tunnel.enabled") }}</a-tag>
        </div>
        <div class="mt-1 truncate font-mono text-sm opacity-70">
          {{ tunnel.local_ip }}:{{ tunnel.local_port }}
          <template v-if="tunnel.remote_port != null">
            <span class="mx-1 opacity-50">→</span>{{ tunnel.remote_port }}
          </template>
        </div>
      </div>

      <div class="flex shrink-0 items-center gap-1">
        <a-tooltip :title="t('tunnel.edit')">
          <a-button type="text" size="small" :icon="h(EditOutlined)" @click="emit('edit')" />
        </a-tooltip>
        <a-popconfirm
          :title="t('tunnel.deleteConfirm', { name: tunnel.name })"
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
      <a-tag :color="typeColor" class="!me-0 uppercase">{{ typeLabel }}</a-tag>
      <a-tag v-if="domainText" color="blue" class="!me-0 max-w-full truncate">{{ domainText }}</a-tag>
      <a-tag v-if="tunnel.use_encryption" color="cyan" class="!me-0">
        {{ t("tunnel.useEncryption") }}
      </a-tag>
      <a-tag v-if="tunnel.use_compression" color="purple" class="!me-0">
        {{ t("tunnel.useCompression") }}
      </a-tag>
      <a-tag v-if="tunnel.secret_key" color="volcano" class="!me-0">Key</a-tag>
    </div>

    <p v-if="tunnel.remark" class="mt-2 line-clamp-2 text-xs opacity-60">{{ tunnel.remark }}</p>

    <a-divider class="!my-3" />

    <!-- 底部开关 -->
    <div class="flex items-center justify-end gap-6">
      <label class="flex cursor-pointer items-center gap-2 text-xs">
        <a-switch size="small" :checked="tunnel.pinned" @change="onPinnedChange" />
        <span>{{ t("tunnel.pinned") }}</span>
      </label>
      <label class="flex cursor-pointer items-center gap-2 text-xs">
        <a-switch size="small" :checked="tunnel.enabled" @change="onEnabledChange" />
        <span>{{ t("tunnel.enabled") }}</span>
      </label>
    </div>
  </a-card>
</template>

<script setup lang="ts">
import { h } from "vue"
import { DeleteOutlined, EditOutlined } from "@ant-design/icons-vue"
import { useI18n } from "vue-i18n"
import type { Tunnel, TunnelType } from "@/types/tunnel"

const props = defineProps<{ tunnel: Tunnel }>()

const emit = defineEmits<{
  edit: []
  delete: []
  toggleEnabled: [value: boolean]
  togglePinned: [value: boolean]
}>()

const { t } = useI18n()

/** 代理类型对应的标签配色 */
const TYPE_COLORS: Record<TunnelType, string> = {
  tcp: "blue",
  udp: "geekblue",
  http: "green",
  https: "cyan",
  stcp: "purple",
  xtcp: "magenta",
  sudp: "volcano",
  tcpmux: "gold",
}

const typeColor = computed(() => TYPE_COLORS[props.tunnel.type] ?? "default")
const typeLabel = computed(() => t(`tunnel.types.${props.tunnel.type}`))

/** 自定义域名 / 子域名摘要 */
const domainText = computed(() => {
  const parts: string[] = []
  if (props.tunnel.custom_domains) parts.push(props.tunnel.custom_domains)
  if (props.tunnel.subdomain) parts.push(`*.${props.tunnel.subdomain}`)
  return parts.join(", ")
})

type SwitchValue = boolean | string | number

function onPinnedChange(checked: SwitchValue) {
  emit("togglePinned", Boolean(checked))
}

function onEnabledChange(checked: SwitchValue) {
  emit("toggleEnabled", Boolean(checked))
}
</script>

<style scoped lang="scss">
.tunnel-card {
  border-color: rgba(15, 23, 42, 0.08);
  transition: border-color 0.2s ease, box-shadow 0.2s ease;
}

.dark .tunnel-card {
  border-color: rgba(255, 255, 255, 0.1);
}
</style>
