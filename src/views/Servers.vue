<template>
  <div class="flex h-full flex-col gap-4 p-6">
    <!-- 顶部：标题 + 操作按钮 -->
    <div class="flex items-center justify-between gap-3">
      <h2 class="text-lg font-semibold">{{ t("menu.servers") }}</h2>
      <a-space>
        <a-button :loading="loading" :icon="h(ReloadOutlined)" @click="refresh()">
          {{ t("server.refresh") }}
        </a-button>
        <a-button type="primary" :icon="h(PlusOutlined)" @click="openCreate">
          {{ t("server.add") }}
        </a-button>
      </a-space>
    </div>

    <!-- 服务器列表 -->
    <a-spin :spinning="loading" class="min-h-0 flex-1">
      <a-empty v-if="!servers.length" class="mt-20" :description="t('server.empty')" />

      <div v-else class="grid grid-cols-1 gap-4 md:grid-cols-2 xl:grid-cols-3">
        <ServerCard v-for="item in servers" :key="item.id" :server="item" @edit="openEdit(item)"
          @delete="onDelete(item)" @toggle-enabled="(value) => onToggleEnabled(item, value)"
          @toggle-pinned="(value) => onTogglePinned(item, value)" />
      </div>
    </a-spin>

    <!-- 新增 / 编辑弹窗 -->
    <ServerFormModal v-model:open="modalOpen" :server="editing" @saved="onSaved" />
  </div>
</template>

<script setup lang="ts">
import { h } from "vue"
import { PlusOutlined, ReloadOutlined } from "@ant-design/icons-vue"
import { message } from "ant-design-vue"
import { useI18n } from "vue-i18n"
import { deleteServer, getServerList, setServerEnabled, setServerPinned } from "@/api/server"
import ServerCard from "@/components/server/ServerCard.vue"
import ServerFormModal from "@/components/server/ServerFormModal.vue"
import type { Server } from "@/types/server"
import { isTauri } from "@/utils/window"

const { t } = useI18n()

const servers = ref<Server[]>([])
const loading = ref(false)
const modalOpen = ref(false)
const editing = ref<Server | null>(null)

function errMsg(e: unknown) {
  return e instanceof Error ? e.message : String(e)
}

/** 拉取服务器列表，silent 为 true 时静默刷新（不显示 loading） */
async function refresh(silent = false) {
  if (!isTauri()) return
  if (!silent) loading.value = true
  try {
    servers.value = (await getServerList()) ?? []
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

function openEdit(server: Server) {
  editing.value = server
  modalOpen.value = true
}

function onSaved() {
  refresh(true)
}

async function onDelete(server: Server) {
  try {
    await deleteServer(server.id)
    message.success(t("server.deleteSuccess"))
    refresh(true)
  } catch (e) {
    message.error(errMsg(e))
  }
}

async function onTogglePinned(server: Server, value: boolean) {
  try {
    await setServerPinned(server.id, value)
    server.pinned = value
    refresh(true)
  } catch (e) {
    message.error(errMsg(e))
    refresh(true)
  }
}

async function onToggleEnabled(server: Server, value: boolean) {
  try {
    await setServerEnabled(server.id, value)
    // 启用互斥：本地同步其它服务器状态
    servers.value.forEach((item) => {
      item.enabled = item.id === server.id ? value : false
    })
    refresh(true)
  } catch (e) {
    message.error(errMsg(e))
    refresh(true)
  }
}

onMounted(() => refresh())
</script>
