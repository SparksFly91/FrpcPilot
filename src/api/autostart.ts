import { isTauri } from "@/utils/window"
import { invoke } from "@tauri-apps/api/core"

export async function isAutostartSupported(): Promise<boolean> {
  if (!isTauri()) return false
  return invoke<boolean>("is_autostart_supported")
}

/**
 * 开机自启封装（Tauri 官方 autostart 插件）。
 *
 * 自启状态由系统启动项决定（Windows 注册表 Run 键 / macOS LaunchAgent /
 * Linux XDG autostart），这里只做读写，不在前端另行持久化，避免两端不一致。
 * 非 Tauri 环境（纯浏览器调试）下调用会被安全忽略。
 */

/** 读取系统启动项：本应用是否已设为开机自启 */
export async function isAutostartEnabled(): Promise<boolean> {
  if (!isTauri()) return false
  const { isEnabled } = await import("@tauri-apps/plugin-autostart")
  return isEnabled()
}

/** 开启 / 关闭开机自启（直接写入系统启动项） */
export async function setAutostartEnabled(enabled: boolean): Promise<void> {
  if (!isTauri()) return
  const { enable, disable } = await import("@tauri-apps/plugin-autostart")
  await (enabled ? enable() : disable())
}
