import { defineStore } from "pinia"
import { exitApp, hideWindow, isTauri, revealWindow } from "@/utils/window"
import { useFrpcStore } from "@/stores/frpc"

/** 关闭窗口时用户的选择：最小化到系统托盘 / 直接退出 */
export type CloseChoice = "tray" | "exit"

/**
 * 窗口生命周期：关闭确认、最小化到系统托盘、退出软件。
 *
 * - 顶部栏关闭按钮与系统原生关闭（如 Alt+F4）都会先弹出确认对话框；
 * - 选择「最小化到系统托盘」时隐藏窗口，程序保持后台运行；
 * - 选择「直接退出」时若穿透正在运行会先停止，再退出整个软件。
 */
export const useWindowStore = defineStore("window", () => {
  const frpcStore = useFrpcStore()

  /** 关闭确认对话框是否打开 */
  const closeConfirmOpen = ref(false)
  /** 当前选中的关闭方式 */
  const closeChoice = ref<CloseChoice>("tray")
  /** 是否正在退出（停止穿透 + 退出软件） */
  const quitting = ref(false)

  let initialized = false

  /** 应用启动时调用一次：拦截系统原生关闭，改为弹出确认对话框 */
  async function init() {
    if (initialized || !isTauri()) return
    initialized = true

    const { getCurrentWindow } = await import("@tauri-apps/api/window")
    await getCurrentWindow().onCloseRequested((event) => {
      event.preventDefault()
      requestClose()
    })
  }

  /** 请求关闭窗口：弹出确认对话框（默认选中最小化到系统托盘） */
  function requestClose() {
    closeChoice.value = "tray"
    closeConfirmOpen.value = true
  }

  /** 取消关闭 */
  function cancelClose() {
    closeConfirmOpen.value = false
  }

  /** 确认关闭：按所选方式处理 */
  async function confirmClose() {
    const choice = closeChoice.value
    closeConfirmOpen.value = false
    if (choice === "tray") {
      await minimizeToTray()
    } else {
      await quitApp()
    }
  }

  /** 最小化到系统托盘（隐藏窗口，保持程序运行） */
  async function minimizeToTray() {
    await hideWindow()
  }

  /** 显示并聚焦主窗口（从系统托盘恢复） */
  async function showWindow() {
    await revealWindow()
  }

  /** 退出软件：穿透正在运行则先停止，再退出 */
  async function quitApp() {
    if (quitting.value) return
    quitting.value = true
    try {
      if (frpcStore.isRunning) {
        await frpcStore.stop()
      }
    } catch {
      // 停止失败不阻塞退出，后端在应用退出时会兜底结束 frpc 子进程
    }
    await exitApp()
  }

  return {
    closeConfirmOpen,
    closeChoice,
    quitting,
    init,
    requestClose,
    cancelClose,
    confirmClose,
    minimizeToTray,
    showWindow,
    quitApp,
  }
})
