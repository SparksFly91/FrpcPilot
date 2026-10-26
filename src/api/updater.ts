import type { Update } from "@tauri-apps/plugin-updater"
import { isTauri } from "@/utils/window"
import { flushPreferences } from "@/utils/preferences"

/** 项目 GitHub 仓库地址（与 tauri.conf.json 中 updater 端点保持一致） */
export const PROJECT_GITHUB_URL = "https://github.com/SparksFly91/FrpcPilot"

/**
 * 检查是否有可用更新。
 *
 * 非 Tauri 环境（纯浏览器调试）下无法调用插件，直接返回 null。
 */
export async function checkForAppUpdate(): Promise<Update | null> {
  if (!isTauri()) return null
  const { check } = await import("@tauri-apps/plugin-updater")
  return check()
}

/** 重启应用（macOS / Linux 安装更新后需要重启才能运行新版本） */
export async function relaunchApp() {
  if (!isTauri()) return
  await flushPreferences()
  const { relaunch } = await import("@tauri-apps/plugin-process")
  await relaunch()
}

/** 用系统默认浏览器打开外部链接 */
export async function openExternal(url: string) {
  if (!isTauri()) {
    // 浏览器调试环境退化为新标签页打开
    window.open(url, "_blank", "noopener,noreferrer")
    return
  }
  const { openUrl } = await import("@tauri-apps/plugin-opener")
  await openUrl(url)
}

/** 打开项目 GitHub 仓库首页 */
export function openProjectRepo() {
  return openExternal(PROJECT_GITHUB_URL)
}

/** 读取当前应用版本（来自 tauri.conf.json / package.json） */
export async function getAppVersion(): Promise<string> {
  if (!isTauri()) return "dev"
  const { getVersion } = await import("@tauri-apps/api/app")
  return getVersion()
}
