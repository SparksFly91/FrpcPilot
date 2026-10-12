/** 操作系统类型 */
export type Platform = "windows" | "macos" | "linux" | "unknown"

/**
 * 判断当前运行的操作系统类型。
 * 通过 navigator.userAgent 推断，避免额外引入 @tauri-apps/plugin-os。
 */
export function usePlatform() {
  const ua = (navigator.userAgent || "").toLowerCase()

  const platform = computed<Platform>(() => {
    if (ua.includes("mac") || ua.includes("darwin")) return "macos"
    if (ua.includes("win")) return "windows"
    if (ua.includes("linux") || ua.includes("x11")) return "linux"
    return "unknown"
  })

  const isMac = computed(() => platform.value === "macos")
  const isWindows = computed(() => platform.value === "windows")
  const isLinux = computed(() => platform.value === "linux")

  return { platform, isMac, isWindows, isLinux }
}
