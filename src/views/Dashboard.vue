<template>
  <div class="flex h-full flex-col gap-3">
    <!-- 顶部统计卡片 -->
    <div class="grid shrink-0 grid-cols-1 gap-3 sm:grid-cols-3">
      <!-- 服务器 / 隧道数量 -->
      <div class="glass-panel flex flex-col justify-center gap-2 px-5 py-4">
        <div class="flex items-center gap-3">
          <span
            class="flex h-9 w-9 shrink-0 items-center justify-center rounded-xl bg-indigo-500/10 text-base text-indigo-500"
          >
            <CloudServerOutlined />
          </span>
          <a-statistic
            :title="t('dashboard.serverCount')"
            :value="serverCount"
            :value-style="statValueStyle"
          />
        </div>
        <div class="flex items-center gap-3">
          <span
            class="flex h-9 w-9 shrink-0 items-center justify-center rounded-xl bg-sky-500/10 text-base text-sky-500"
          >
            <ApiOutlined />
          </span>
          <a-statistic
            :title="t('dashboard.tunnelCount')"
            :value="tunnelCount"
            :value-style="statValueStyle"
          />
        </div>
      </div>

      <!-- 系统信息 -->
      <div class="glass-panel flex items-center gap-4 px-5 py-4">
        <span
          class="flex h-12 w-12 shrink-0 items-center justify-center rounded-2xl bg-emerald-500/10 text-xl text-emerald-500"
        >
          <DesktopOutlined />
        </span>
        <div class="flex min-w-0 flex-1 flex-col gap-1.5 text-xs">
          <div class="flex items-center justify-between gap-2">
            <span class="shrink-0 opacity-70">{{ t("dashboard.hostname") }}</span>
            <span class="truncate font-medium" :title="details?.hostname">
              {{ details?.hostname || "-" }}
            </span>
          </div>
          <div class="flex items-center justify-between gap-2">
            <span class="shrink-0 opacity-70">{{ t("dashboard.os") }}</span>
            <span class="truncate font-medium" :title="details?.os">{{ details?.os || "-" }}</span>
          </div>
          <div class="flex items-center justify-between gap-2">
            <span class="shrink-0 opacity-70">{{ t("dashboard.arch") }}</span>
            <span class="truncate font-medium">{{ details?.arch || "-" }}</span>
          </div>
          <div class="flex items-center justify-between gap-2">
            <span class="shrink-0 opacity-70">{{ t("dashboard.ip") }}</span>
            <span class="truncate font-medium" :title="ipFull">{{ ipText }}</span>
          </div>
        </div>
      </div>

      <!-- CPU / 内存 / 运行状态 -->
      <div class="glass-panel flex flex-col justify-center gap-2 px-5 py-4">
        <div class="flex items-center justify-between text-xs">
          <span class="opacity-70">{{ t("system.cpu") }}</span>
          <span class="font-medium tabular-nums">{{ cpu }}%</span>
        </div>
        <a-progress :percent="cpu" :show-info="false" :stroke-color="barColor(cpu)" size="small" />

        <div class="flex items-center justify-between text-xs">
          <span class="opacity-70">{{ t("system.memory") }}</span>
          <span class="font-medium tabular-nums">{{ memory }}%</span>
        </div>
        <a-progress :percent="memory" :show-info="false" :stroke-color="barColor(memory)" size="small" />

        <div class="mt-1 flex items-center gap-2 text-xs">
          <span class="h-2 w-2 shrink-0 rounded-full" :class="statusDotClass" />
          <span class="opacity-70">{{ t("system.status") }}</span>
          <span class="font-medium">{{ statusText }}</span>
        </div>
      </div>
    </div>

    <!-- 快速操作 -->
    <div
      class="glass-panel glass-panel-flat flex shrink-0 flex-wrap items-center justify-between gap-3 px-5 py-3"
    >
      <div class="flex items-center gap-3">
        <span class="shrink-0 text-sm opacity-70">{{ t("dashboard.frpcVersion") }}</span>
        <a-select
          v-model:value="frpcStore.selectedId"
          :options="versionOptions"
          :loading="versionsLoading"
          :disabled="frpcStore.isRunning || frpcStore.starting"
          :placeholder="t('dashboard.frpcVersionEmpty')"
          class="w-64"
          show-search
          option-filter-prop="label"
        />
      </div>

      <a-space>
        <a-button
          type="primary"
          :icon="h(PlayCircleOutlined)"
          :loading="frpcStore.starting"
          :disabled="frpcStore.isRunning || !frpcStore.selectedId"
          @click="start"
        >
          {{ t("dashboard.start") }}
        </a-button>
        <a-button
          danger
          :icon="h(PoweroffOutlined)"
          :loading="frpcStore.stopping"
          :disabled="!frpcStore.isRunning"
          @click="stop"
        >
          {{ t("dashboard.stop") }}
        </a-button>
      </a-space>
    </div>

    <!-- 实时日志 -->
    <div class="glass-panel glass-panel-flat flex min-h-0 flex-1 flex-col overflow-hidden">
      <div
        class="flex shrink-0 items-center justify-between gap-3 border-b border-black/5 px-5 py-3 dark:border-white/10"
      >
        <div class="flex items-center gap-2">
          <h3 class="text-sm font-semibold">{{ t("dashboard.logs") }}</h3>
          <a-tag v-if="frpcStore.runningVersion" class="!me-0">
            {{ frpcStore.runningVersion }}
          </a-tag>
          <a-tag :color="statusTagColor" class="!me-0">{{ statusText }}</a-tag>
        </div>

        <a-tooltip :title="t('dashboard.clear')">
          <a-button size="small" :icon="h(ClearOutlined)" @click="frpcStore.clearLogs" />
        </a-tooltip>
      </div>

      <div
        ref="logBox"
        class="min-h-0 flex-1 overflow-y-auto px-5 py-3 font-mono text-xs leading-relaxed"
      >
        <p v-if="!frpcStore.logs.length" class="opacity-50">{{ t("dashboard.logEmpty") }}</p>
        <div v-for="(line, index) in frpcStore.logs" :key="index" class="flex gap-2">
          <span class="shrink-0 opacity-50">{{ line.time }}</span>
          <span class="shrink-0" :class="levelClass(line.level)">[{{ line.level }}]</span>
          <span class="break-all">{{ line.message }}</span>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { h } from "vue"
import {
  ApiOutlined,
  ClearOutlined,
  CloudServerOutlined,
  DesktopOutlined,
  PlayCircleOutlined,
  PoweroffOutlined,
} from "@ant-design/icons-vue"
import { message } from "ant-design-vue"
import { useI18n } from "vue-i18n"
import { getFrpDownloads } from "@/api/frp"
import { getServerList } from "@/api/server"
import { getSystemDetails } from "@/api/system"
import { getTunnelList } from "@/api/tunnel"
import { useAppStore } from "@/stores/app"
import { useFrpcStore, type LogLevel } from "@/stores/frpc"
import { useSystemStats } from "@/composables/useSystemStats"
import type { FrpDownload } from "@/types/frp"
import type { SystemDetails } from "@/types/system"
import { isTauri } from "@/utils/window"

const { t } = useI18n()
const appStore = useAppStore()
const frpcStore = useFrpcStore()
const { cpu, memory } = useSystemStats()

/** 服务器 / 隧道数量（来自后端，未连接 Tauri 时保持 0） */
const serverCount = ref(0)
const tunnelCount = ref(0)

/** 系统信息（主机名 / 系统 / 架构 / IP） */
const details = ref<SystemDetails | null>(null)

/** 卡片内展示的主 IP（多个时追加数量，完整列表在 tooltip 中显示） */
const ipText = computed(() => {
  const ips = details.value?.ips ?? []
  if (!ips.length) return "-"
  return ips.length > 1 ? `${ips[0]} +${ips.length - 1}` : ips[0]
})
const ipFull = computed(() => (details.value?.ips?.length ? details.value.ips.join(", ") : "-"))

/** 合并卡片中两个统计数值的样式（更紧凑） */
const statValueStyle = { fontSize: "20px", fontWeight: 600, lineHeight: "1.2" }

/** 已下载 / 已导入的 Frp 版本（来自 frp_downloads） */
const frpDownloads = ref<FrpDownload[]>([])
const versionsLoading = ref(false)

/** 仅展示具备 frpc 可执行文件的记录 */
const versionOptions = computed(() =>
  frpDownloads.value
    .filter((item) => item.frpc_path)
    .map((item) => ({ label: `${item.version} · ${item.platform}`, value: String(item.id) })),
)

/** 日志容器与自动滚动 */
const logBox = ref<HTMLElement | null>(null)

function scrollLogsToBottom() {
  nextTick(() => {
    const el = logBox.value
    if (el) el.scrollTop = el.scrollHeight
  })
}

// 日志追加时自动滚动到底部（日志保存在 store 中，切换页面不会清空）
watch(() => frpcStore.logs.length, scrollLogsToBottom)

function errMsg(e: unknown) {
  return e instanceof Error ? e.message : String(e)
}

/** 一键启动：由后端生成 frpc.toml 并拉起独立进程 */
async function start() {
  if (frpcStore.isRunning || frpcStore.starting) return
  if (!frpcStore.selectedId) return

  try {
    await frpcStore.start(Number(frpcStore.selectedId))
  } catch (e) {
    appStore.runStatus = "error"
    frpcStore.pushLog("ERROR", t("dashboard.logStartFailed", { message: errMsg(e) }))
    message.error(errMsg(e))
  }
}

/** 停止 Frpc 服务进程 */
async function stop() {
  if (!frpcStore.isRunning || frpcStore.stopping) return
  try {
    await frpcStore.stop()
  } catch (e) {
    message.error(errMsg(e))
  }
}

/** 拉取服务器 / 隧道数量 */
async function loadCounts() {
  if (!isTauri()) return
  try {
    const [servers, tunnels] = await Promise.all([getServerList(), getTunnelList()])
    serverCount.value = servers?.length ?? 0
    tunnelCount.value = tunnels?.length ?? 0
  } catch {
    // 读取失败时保持 0，不影响仪表盘其它区域展示
  }
}

/** 拉取系统信息（主机名 / 系统 / 架构 / IP） */
async function loadSystemDetails() {
  if (!isTauri()) return
  try {
    details.value = await getSystemDetails()
  } catch {
    // 读取失败时保持占位符
  }
}

/** 拉取已下载 / 已导入的 Frp 版本，供版本下拉选择 */
async function loadFrpDownloads() {
  if (!isTauri()) return
  versionsLoading.value = true
  try {
    frpDownloads.value = (await getFrpDownloads()) ?? []
    const options = versionOptions.value
    if (!options.length) return
    // 运行中保持当前版本；否则已选版本失效时回退到最新一个
    const valid = options.some((option) => option.value === frpcStore.selectedId)
    if (!valid && !frpcStore.isRunning) {
      frpcStore.selectedId = options[0].value
    }
  } catch {
    // 读取失败时保持空列表
  } finally {
    versionsLoading.value = false
  }
}

onMounted(() => {
  loadCounts()
  loadSystemDetails()
  loadFrpDownloads()
  scrollLogsToBottom()
})

/** 进度条颜色：低=蓝，中=橙，高=红 */
function barColor(value: number) {
  if (value >= 85) return "#ef4444"
  if (value >= 60) return "#f59e0b"
  return "#6366f1"
}

const statusDotClass = computed(() => {
  switch (appStore.runStatus) {
    case "running":
      return "bg-green-500 shadow-[0_0_6px_#22c55e]"
    case "error":
      return "bg-red-500 shadow-[0_0_6px_#ef4444]"
    default:
      return "bg-yellow-400"
  }
})

const statusText = computed(() => {
  switch (appStore.runStatus) {
    case "running":
      return t("system.statusRunning")
    case "error":
      return t("system.statusError")
    default:
      return t("system.statusStopped")
  }
})

const statusTagColor = computed(() => {
  switch (appStore.runStatus) {
    case "running":
      return "success"
    case "error":
      return "error"
    default:
      return "default"
  }
})

function levelClass(level: LogLevel) {
  switch (level) {
    case "WARN":
      return "text-amber-500"
    case "ERROR":
      return "text-red-500"
    case "DEBUG":
      return "text-slate-400"
    default:
      return "text-indigo-500"
  }
}
</script>
