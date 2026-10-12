/**
 * Tauri 窗口控制封装。
 * 在纯浏览器环境（非 Tauri）下调用会被安全忽略，方便 `pnpm dev` 调试。
 */

export function isTauri() {
  return typeof window !== "undefined" && "__TAURI_INTERNALS__" in window
}

async function currentWindow() {
  if (!isTauri()) return null
  const { getCurrentWindow } = await import("@tauri-apps/api/window")
  return getCurrentWindow()
}

/** 最小化窗口 */
export async function minimizeWindow() {
  const win = await currentWindow()
  await win?.minimize()
}

/** 最大化 / 还原窗口 */
export async function toggleMaximizeWindow() {
  const win = await currentWindow()
  await win?.toggleMaximize()
}

/** 关闭窗口 */
export async function closeWindow() {
  const win = await currentWindow()
  await win?.close()
}

/** 开始拖拽窗口（配合 data-tauri-drag-region 使用） */
export async function startDragging() {
  const win = await currentWindow()
  await win?.startDragging()
}

/** 是否已最大化 */
export async function isWindowMaximized() {
  const win = await currentWindow()
  return (await win?.isMaximized()) ?? false
}
