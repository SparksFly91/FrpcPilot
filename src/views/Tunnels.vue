<template>
  <div class="flex h-full flex-col gap-3">
    <!-- 顶部操作栏（独立磨砂卡片） -->
    <div class="glass-panel glass-panel-flat flex shrink-0 items-center justify-between gap-3 px-5 py-3">
      <h2 class="text-lg font-semibold">{{ t("menu.tunnels") }}</h2>
      <a-space>
        <a-button :loading="loading" :icon="h(ReloadOutlined)" @click="refresh()">
          {{ t("tunnel.refresh") }}
        </a-button>
        <a-button type="primary" :icon="h(PlusOutlined)" @click="openCreate">
          {{ t("tunnel.add") }}
        </a-button>
      </a-space>
    </div>

    <!-- 内容区（独立磨砂卡片，内部滚动） -->
    <div class="glass-panel glass-panel-flat min-h-0 flex-1 overflow-y-auto p-5">
      <a-spin :spinning="loading">
        <a-empty v-if="!tunnels.length" class="mt-20" :description="t('tunnel.empty')" />

        <div v-else class="grid grid-cols-1 gap-4 md:grid-cols-2 xl:grid-cols-3">
          <TunnelCard
            v-for="item in tunnels"
            :key="item.id"
            :tunnel="item"
            @edit="openEdit(item)"
            @delete="onDelete(item)"
            @toggle-enabled="(value) => onToggleEnabled(item, value)"
            @toggle-pinned="(value) => onTogglePinned(item, value)"
          />
        </div>
      </a-spin>
    </div>

    <!-- 新增 / 编辑弹窗 -->
    <TunnelFormModal v-model:open="modalOpen" :tunnel="editing" @saved="onSaved" />
  </div>
</template>

<script setup lang="ts">
import { h } from "vue"
import { PlusOutlined, ReloadOutlined } from "@ant-design/icons-vue"
import { message } from "ant-design-vue"
import { useI18n } from "vue-i18n"
import { deleteTunnel, getTunnelList, setTunnelEnabled, setTunnelPinned } from "@/api/tunnel"
import TunnelCard from "@/components/tunnel/TunnelCard.vue"
import TunnelFormModal from "@/components/tunnel/TunnelFormModal.vue"
import type { Tunnel } from "@/types/tunnel"
import { isTauri } from "@/utils/window"

const { t } = useI18n()

const tunnels = ref<Tunnel[]>([])
const loading = ref(false)
const modalOpen = ref(false)
const editing = ref<Tunnel | null>(null)

function errMsg(e: unknown) {
  return e instanceof Error ? e.message : String(e)
}

/** 拉取隧道列表，silent 为 true 时静默刷新（不显示 loading） */
async function refresh(silent = false) {
  if (!isTauri()) return
  if (!silent) loading.value = true
  try {
    tunnels.value = (await getTunnelList()) ?? []
  } catch (e) {
    message.error(errMsg(e))
  } finally {
    if (!silent) loading.value = false
  }
}

function openCreate() {
  editing.value = null
  modalOpen.value = true
}

function openEdit(tunnel: Tunnel) {
  editing.value = tunnel
  modalOpen.value = true
}

function onSaved() {
  refresh(true)
}

async function onDelete(tunnel: Tunnel) {
  try {
    await deleteTunnel(tunnel.id)
    message.success(t("tunnel.deleteSuccess"))
    refresh(true)
  } catch (e) {
    message.error(errMsg(e))
  }
}

async function onTogglePinned(tunnel: Tunnel, value: boolean) {
  try {
    await setTunnelPinned(tunnel.id, value)
    tunnel.pinned = value
    refresh(true)
  } catch (e) {
    message.error(errMsg(e))
    refresh(true)
  }
}

async function onToggleEnabled(tunnel: Tunnel, value: boolean) {
  try {
    await setTunnelEnabled(tunnel.id, value)
    tunnel.enabled = value
    refresh(true)
  } catch (e) {
    message.error(errMsg(e))
    refresh(true)
  }
}

onMounted(() => refresh())
</script>
