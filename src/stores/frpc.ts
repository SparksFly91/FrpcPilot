import { defineStore } from "pinia"
import dayjs from "dayjs"
import { listen } from "@tauri-apps/api/event"
import { getFrpcStatus, startFrpc, stopFrpc } from "@/api/frp"
import { translate } from "@/i18n"
import { useAppStore } from "@/stores/app"
import type { FrpcExitEvent, FrpcLogEvent, FrpcStatus } from "@/types/frp"
import { isTauri } from "@/utils/window"

/** 日志级别 */
export type LogLevel = "INFO" | "WARN" | "ERROR" | "DEBUG"

/** 一行实时日志 */
export interface LogLine {
  time: string
  level: LogLevel
  message: string
}

/** 日志最大保留行数 */
const MAX_LOGS = 500

/**
 * Frpc 运行状态与实时日志。
 *
 * - 日志、选中版本放在 store 中，切换路由（组件卸载）后依然保留；
 * - 事件监听在应用根组件初始化一次，离开仪表盘后仍持续收集日志；
 * - 运行中时下拉框只展示当前使用的版本（由组件禁用下拉）。
 */
export const useFrpcStore = defineStore("frpc", () => {
  const appStore = useAppStore()

  const logs = ref<LogLine[]>([])
  /** 下拉框选中的版本记录 ID；运行时即正在使用的版本 */
  const selectedId = ref<string | undefined>(undefined)
  /** 正在运行的 Frpc 版本号（用于日志区标签） */
  const runningVersion = ref("")
  const starting = ref(false)
  const stopping = ref(false)
  const initialized = ref(false)

  /** 是否正在运行 */
  const isRunning = computed(() => appStore.runStatus === "running")

  /** 追加一行日志（最多保留 MAX_LOGS 行） */
  function pushLog(level: LogLevel, message: string, time?: string) {
    logs.value.push({ time: time || dayjs().format("HH:mm:ss"), level, message })
    if (logs.value.length > MAX_LOGS) {
      logs.value.splice(0, logs.value.length - MAX_LOGS)
    }
  }

  function clearLogs() {
    logs.value = []
  }

  /** 将后端日志级别归一化为 UI 级别 */
  function normalizeLevel(level: string): LogLevel {
    switch (level.toUpperCase()) {
      case "WARN":
      case "WARNING":
        return "WARN"
      case "ERROR":
      case "FATAL":
        return "ERROR"
      case "DEBUG":
      case "TRACE":
        return "DEBUG"
      default:
        return "INFO"
    }
  }

  /** 后端实时日志事件（frpc-log） */
  function onBackendLog(event: FrpcLogEvent) {
    pushLog(normalizeLevel(event.level), event.message, event.time ?? undefined)
  }

  /** 后端进程退出事件（frpc-exit） */
  function onBackendExit(event: FrpcExitEvent) {
    runningVersion.value = ""
    if (event.reason === "stopped") {
      appStore.runStatus = "stopped"
      return
    }
    appStore.runStatus = event.code != null && event.code !== 0 ? "error" : "stopped"
  }

  /** 应用启动时调用一次：订阅后端事件并同步运行状态 */
  async function init() {
    if (initialized.value) return
    initialized.value = true

    pushLog("INFO", translate("dashboard.logReady"))

    if (!isTauri()) return

    // 监听在应用根组件初始化一次，长期有效（应用运行期间无需取消订阅）
    await listen<FrpcLogEvent>("frpc-log", (event) => onBackendLog(event.payload))
    await listen<FrpcExitEvent>("frpc-exit", (event) => onBackendExit(event.payload))

    // 同步当前运行状态
    try {
      const status = await getFrpcStatus()
      appStore.runStatus = status.running ? "running" : "stopped"
      runningVersion.value = status.running ? status.version ?? "" : ""
    } catch {
      // 读取失败时保持默认状态
    }
  }

  /** 一键启动：由后端生成 frpc.toml 并拉起独立进程 */
  async function start(id: number): Promise<FrpcStatus> {
    starting.value = true
    try {
      const status = await startFrpc(id)
      appStore.runStatus = "running"
      selectedId.value = String(id)
      runningVersion.value = status.version ?? ""
      return status
    } finally {
      starting.value = false
    }
  }

  /** 停止 Frpc 服务进程 */
  async function stop(): Promise<FrpcStatus> {
    stopping.value = true
    try {
      const status = await stopFrpc()
      appStore.runStatus = "stopped"
      runningVersion.value = ""
      return status
    } finally {
      stopping.value = false
    }
  }

  return {
    logs,
    selectedId,
    runningVersion,
    starting,
    stopping,
    isRunning,
    pushLog,
    clearLogs,
    init,
    start,
    stop,
  }
})
