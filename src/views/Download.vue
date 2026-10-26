<template>
  <div class="flex h-full flex-col gap-3">
    <!-- 顶部操作栏（独立磨砂卡片） -->
    <div
      class="glass-panel glass-panel-flat flex shrink-0 flex-wrap items-center justify-between gap-3 px-5 py-3"
    >
      <div class="flex items-center gap-2">
        <h2 class="text-lg font-semibold">{{ t("menu.download") }}</h2>
        <a-tag v-if="releases.length" class="!me-0">{{ releases.length }}</a-tag>
      </div>

      <div class="flex flex-wrap items-center gap-x-5 gap-y-2">
        <label class="flex items-center gap-2 text-xs">
          <span class="shrink-0 opacity-70">{{ t("frp.platformFilter") }}</span>
          <a-select
            v-model:value="selectedPlatform"
            :aria-label="t('frp.platformFilter')"
            :options="platformOptions"
            class="w-44"
            size="small"
          />
        </label>

        <a-space>
          <a-button :loading="loading" :icon="h(ReloadOutlined)" @click="refresh()">
            {{ t("frp.refresh") }}
          </a-button>
          <a-button :loading="importing" :icon="h(ImportOutlined)" @click="onImport">
            {{ t("frp.import") }}
          </a-button>
        </a-space>
      </div>
    </div>

    <!-- 内容区（独立磨砂卡片，内部滚动） -->
    <div class="glass-panel glass-panel-flat min-h-0 flex-1 overflow-y-auto p-5">
      <a-spin :spinning="loading">
        <a-empty v-if="!releases.length" class="mt-20" :description="t('frp.empty')" />

        <div v-else class="grid grid-cols-1 gap-4 md:grid-cols-2 xl:grid-cols-3">
          <FrpCard
            v-for="item in releases"
            :key="`${item.version}-${item.platform}-${item.file_name}`"
            :release="item"
            :progress="progressMap[item.file_name]"
            @download="onDownload(item)"
            @copy="onCopy(item)"
            @delete="onDelete(item)"
          />
        </div>
      </a-spin>
    </div>
  </div>
</template>

<script setup lang="ts">
import { h } from "vue"
import { ImportOutlined, ReloadOutlined } from "@ant-design/icons-vue"
import { message } from "ant-design-vue"
import { listen, type UnlistenFn } from "@tauri-apps/api/event"
import { useI18n } from "vue-i18n"
import { deleteFrp, downloadFrp, getFrpReleases, importFrp } from "@/api/frp"
import FrpCard from "@/components/frp/FrpCard.vue"
import type { FrpDownloadProgress, FrpRelease } from "@/types/frp"
import { isTauri } from "@/utils/window"

const { t } = useI18n()

// 组件名需与路由 name 一致，以便被 Layout 的 <keep-alive :include> 缓存
defineOptions({ name: "Download" })

const loading = ref(false)
const importing = ref(false)

const ALL_PLATFORMS = "all"
const selectedPlatform = ref(ALL_PLATFORMS)
const platformInitialized = ref(false)

/** 后端返回的全部发行包（含所有架构） */
const allReleases = ref<FrpRelease[]>([])

/**
 * 从发行包数据中生成平台筛选项，首项固定为“全部”。
 */
const platformOptions = computed(() => [
  { value: ALL_PLATFORMS, label: t("frp.allPlatforms") },
  ...Array.from(new Set(allReleases.value.map((item) => item.platform)))
    .sort((a, b) => a.localeCompare(b))
    .map((platform) => ({ value: platform, label: platform })),
])

/** 按版本降序排列；同版本按平台稳定排序，保证“全部”视图按版本分组展示。 */
function compareReleases(a: FrpRelease, b: FrpRelease) {
  const aVersion = (a.version.match(/\d+/g) ?? []).slice(0, 3).map(Number)
  const bVersion = (b.version.match(/\d+/g) ?? []).slice(0, 3).map(Number)
  for (let index = 0; index < 3; index += 1) {
    const difference = (bVersion[index] ?? 0) - (aVersion[index] ?? 0)
    if (difference !== 0) return difference
  }

  return (
    b.published_at.localeCompare(a.published_at) ||
    a.platform.localeCompare(b.platform) ||
    a.file_name.localeCompare(b.file_name)
  )
}

const releases = computed(() => {
  const filtered =
    selectedPlatform.value === ALL_PLATFORMS
      ? allReleases.value
      : allReleases.value.filter((item) => item.platform === selectedPlatform.value)

  return [...filtered].sort(compareReleases)
})

/** 各发行包的下载进度（以文件名为键） */
const progressMap = ref<Record<string, FrpDownloadProgress>>({})

function errMsg(e: unknown) {
  return e instanceof Error ? e.message : String(e)
}

/** 拉取 Frp 版本列表，silent 为 true 时静默刷新（不显示 loading） */
async function refresh(silent = false) {
  if (!isTauri()) return
  if (!silent) loading.value = true
  try {
    allReleases.value = (await getFrpReleases()) ?? []
    if (!platformInitialized.value) {
      selectedPlatform.value =
        allReleases.value.find((item) => item.is_current)?.platform ?? ALL_PLATFORMS
      platformInitialized.value = true
    } else if (
      selectedPlatform.value !== ALL_PLATFORMS &&
      !allReleases.value.some((item) => item.platform === selectedPlatform.value)
    ) {
      selectedPlatform.value = ALL_PLATFORMS
    }
  } catch (e) {
    message.error(errMsg(e))
  } finally {
    if (!silent) loading.value = false
  }
}

async function onDownload(item: FrpRelease) {
  if (!item.download_url) return
  progressMap.value[item.file_name] = {
    key: item.file_name,
    version: item.version,
    platform: item.platform,
    downloaded: 0,
    total: item.size,
    percent: 0,
    status: "downloading",
    message: null,
  }

  try {
    const row = await downloadFrp({
      version: item.version,
      os: item.os,
      arch: item.arch,
      platform: item.platform,
      file_name: item.file_name,
      download_url: item.download_url,
    })
    // 直接更新本地状态，避免重复请求 GitHub API
    item.downloaded = true
    item.local_id = row.id
    item.frpc_path = row.frpc_path
    item.frps_path = row.frps_path
    message.success(t("frp.downloadSuccess"))
  } catch (e) {
    message.error(errMsg(e))
  }
}

async function onCopy(item: FrpRelease) {
  const link = item.mirror_url || item.download_url
  if (!link) return
  try {
    await navigator.clipboard.writeText(link)
    message.success(t("frp.copySuccess"))
  } catch {
    message.error(t("frp.copyFailed"))
  }
}

async function onDelete(item: FrpRelease) {
  if (item.local_id == null) return
  try {
    await deleteFrp(item.local_id)
    message.success(t("frp.deleteSuccess"))
    if (item.local_only) {
      // 仅本机的记录直接移除
      allReleases.value = allReleases.value.filter((it) => it !== item)
    } else {
      item.downloaded = false
      item.local_id = null
      item.frpc_path = null
      item.frps_path = null
    }
  } catch (e) {
    message.error(errMsg(e))
  }
}

async function onImport() {
  importing.value = true
  try {
    const rows = await importFrp()
    if (rows?.length) {
      message.success(t("frp.importSuccess", { count: rows.length }))
    } else {
      message.info(t("frp.importEmpty"))
    }
    await refresh(true)
  } catch (e) {
    message.error(errMsg(e))
  } finally {
    importing.value = false
  }
}

/** 下载进度事件：更新对应卡片的进度条 */
function onProgress(p: FrpDownloadProgress) {
  progressMap.value[p.key] = p
  if (p.status === "done") {
    setTimeout(() => {
      delete progressMap.value[p.key]
    }, 1500)
  }
}

let unlisten: UnlistenFn | null = null

onMounted(async () => {
  refresh()
  if (isTauri()) {
    unlisten = await listen<FrpDownloadProgress>("frp-download-progress", (event) =>
      onProgress(event.payload),
    )
  }
})

onUnmounted(() => unlisten?.())
</script>
