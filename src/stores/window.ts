import { defineStore } from "pinia"
import { exitApp, hideWindow, isTauri, revealWindow } from "@/utils/window"
import { useFrpcStore } from "@/stores/frpc"

/** 关闭窗口时用户的选择：最小化到系统托盘 / 直接退出 */
export type CloseChoice = "tray" | "exit"

/** 关闭行为：每次询问 / 保持以上选择 */
export type CloseBehavior = "ask" | "remember"

/**
 * 窗口生命周期：关闭确认、最小化到系统托盘、退出软件。
 *
 * - 顶部栏关闭按钮与系统原生关闭（如 Alt+F4）都会先经过 requestClose；
 * - 关闭行为为「每次询问」时弹出确认对话框，选择「最小化到系统托盘」会隐藏窗口、
 *   程序保持后台运行，选择「直接退出」会先停止穿透再退出整个软件；
 * - 关闭行为为「保持以上选择」时不弹窗，直接按记住的关闭方式处理。
 *
 * 关闭行为与关闭方式均持久化，可在「设置 - 常规」中再次修改，与关闭弹窗内容保持一致。
 */
export const useWindowStore = defineStore("window", () => {
  const frpcStore = useFrpcStore()

  /** 关闭确认对话框是否打开 */
  const closeConfirmOpen = ref(false)
  /** 记住的关闭方式（弹窗默认选中项；「保持以上选择」时直接生效，持久化） */
  const closeChoice = useStorage<CloseChoice>("frpcpilot:close-choice", "tray")
  /** 关闭行为：每次询问 / 保持以上选择（持久化，默认每次询问） */
  const closeBehavior = useStorage<CloseBehavior>("frpcpilot:close-behavior", "ask")
  /** 是否正在退出（停止穿透 + 退出软件） */
  const quitting = ref(false)

  let initialized = false

  /** 应用启动时调用一次：拦截系统原生关闭，改为按关闭行为处理 */
  async function init() {
    if (initialized || !isTauri()) return
    initialized = true

    const { getCurrentWindow } = await import("@tauri-apps/api/window")
    await getCurrentWindow().onCloseRequested((event) => {
      event.preventDefault()
      requestClose()
    })
  }

  /**
   * 请求关闭窗口：
   * - 「每次询问」弹出确认对话框；
   * - 「保持以上选择」不弹窗，直接按记住的方式处理。
   */
  function requestClose() {
    if (closeBehavior.value === "remember") {
      void applyClose(closeChoice.value)
      return
    }
    closeConfirmOpen.value = true
  }

  /** 取消关闭 */
  function cancelClose() {
    closeConfirmOpen.value = false
  }

  /** 确认关闭：持久化所选行为与方式，再按所选方式处理 */
  async function confirmClose(
    choice: CloseChoice = closeChoice.value,
    behavior: CloseBehavior = closeBehavior.value,
  ) {
    closeChoice.value = choice
    closeBehavior.value = behavior
    closeConfirmOpen.value = false
    await applyClose(choice)
  }

  /** 按所选关闭方式执行：最小化到托盘 / 退出软件 */
  async function applyClose(choice: CloseChoice) {
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
    closeBehavior,
    quitting,
    init,
    requestClose,
    cancelClose,
    confirmClose,
    applyClose,
    minimizeToTray,
    showWindow,
    quitApp,
  }
})
