import { message } from "ant-design-vue"
import { translate } from "@/i18n"
import { useAppStore } from "@/stores/app"
import { useFrpcStore } from "@/stores/frpc"
import { useWindowStore } from "@/stores/window"
import { isTauri } from "@/utils/window"

/** 是否已初始化，避免重复创建托盘 */
let initialized = false

function errMsg(e: unknown) {
  return e instanceof Error ? e.message : String(e)
}

/**
 * 初始化系统托盘（Tauri 前端 API 方式，参见 Tauri 文档「系统托盘」）。
 *
 * 在应用根组件挂载时调用一次即可。托盘菜单共四项：
 * 显示窗口 / 启动穿透 / 停止穿透 / 退出。
 *
 * - 「启动穿透」「停止穿透」与仪表盘的一键启动 / 停止共用 store 逻辑，
 *   并随 Frpc 运行状态联动禁用：未运行时禁用「停止穿透」，运行中禁用「启动穿透」；
 * - 「退出」会先停止正在运行的穿透，再退出整个软件。
 */
export function initTray() {
  if (initialized || !isTauri()) return
  initialized = true
  void createTray().catch((e) => {
    initialized = false
    console.error("系统托盘初始化失败：", e)
  })
}

async function createTray() {
  const appStore = useAppStore()
  const frpcStore = useFrpcStore()
  const windowStore = useWindowStore()

  const [{ TrayIcon }, { Menu, MenuItem, PredefinedMenuItem }, { defaultWindowIcon }] =
    await Promise.all([
      import("@tauri-apps/api/tray"),
      import("@tauri-apps/api/menu"),
      import("@tauri-apps/api/app"),
    ])

  // 显示窗口（始终可用）
  const showItem = await MenuItem.new({
    id: "tray-show",
    text: translate("tray.show"),
    action: () => windowStore.showWindow(),
  })

  // 启动穿透（未运行时可用）
  const startItem = await MenuItem.new({
    id: "tray-start",
    text: translate("tray.start"),
    enabled: frpcStore.canStart,
    action: () => void startFromTray(frpcStore),
  })

  // 停止穿透（运行中才可用）
  const stopItem = await MenuItem.new({
    id: "tray-stop",
    text: translate("tray.stop"),
    enabled: frpcStore.canStop,
    action: () => void stopFromTray(frpcStore),
  })

  // 退出（先停止穿透，再退出软件）
  const quitItem = await MenuItem.new({
    id: "tray-quit",
    text: translate("tray.quit"),
    action: () => windowStore.quitApp(),
  })

  const [separatorTop, separatorBottom] = await Promise.all([
    PredefinedMenuItem.new({ item: "Separator" }),
    PredefinedMenuItem.new({ item: "Separator" }),
  ])

  const menu = await Menu.new({
    items: [showItem, separatorTop, startItem, stopItem, separatorBottom, quitItem],
  })

  // 托盘图标复用应用自身的窗口图标
  const icon = await defaultWindowIcon().catch(() => null)

  await TrayIcon.new({
    id: "frpilot-tray",
    icon: icon ?? undefined,
    tooltip: "Frpilot",
    menu,
    // 左键点击直接显示窗口，右键才弹出菜单
    showMenuOnLeftClick: false,
    action: (event) => {
      // 仅在左键按下时响应，避免 Down / Up 触发两次
      if (event.type === "Click" && event.button === "Left" && event.buttonState === "Down") {
        void windowStore.showWindow()
      }
    },
  })

  // Frpc 运行状态变化时同步菜单可用状态
  watch(
    [() => frpcStore.canStart, () => frpcStore.canStop],
    ([canStart, canStop]) => {
      void startItem.setEnabled(canStart)
      void stopItem.setEnabled(canStop)
    },
  )

  // 语言切换时同步菜单文案
  watch(
    () => appStore.language,
    () => {
      void showItem.setText(translate("tray.show"))
      void startItem.setText(translate("tray.start"))
      void stopItem.setText(translate("tray.stop"))
      void quitItem.setText(translate("tray.quit"))
    },
  )
}

/** 托盘「启动穿透」：复用仪表盘的一键启动逻辑 */
async function startFromTray(frpcStore: ReturnType<typeof useFrpcStore>) {
  if (!frpcStore.canStart) return
  try {
    await frpcStore.startSelected()
  } catch (e) {
    useAppStore().runStatus = "error"
    frpcStore.pushLog("ERROR", translate("dashboard.logStartFailed", { message: errMsg(e) }))
    message.error(errMsg(e))
  }
}

/** 托盘「停止穿透」：复用仪表盘的停止逻辑 */
async function stopFromTray(frpcStore: ReturnType<typeof useFrpcStore>) {
  if (!frpcStore.canStop) return
  try {
    await frpcStore.stop()
  } catch (e) {
    message.error(errMsg(e))
  }
}
