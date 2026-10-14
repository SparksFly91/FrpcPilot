/** 操作系统类型 */
export type Platform = "windows" | "macos" | "linux" | "unknown"

/**
 * 判断当前运行的操作系统类型。
 * 通过 navigator.userAgent 推断，避免额外引入 @tauri-apps/plugin-os。
 */
export function detectPlatform(): Platform {
  const ua = (navigator.userAgent || "").toLowerCase()
  if (ua.includes("mac") || ua.includes("darwin")) return "macos"
  if (ua.includes("win")) return "windows"
  if (ua.includes("linux") || ua.includes("x11")) return "linux"
  return "unknown"
}

/** 在组件中响应式地获取当前平台 */
export function usePlatform() {
  const platform = ref<Platform>(detectPlatform())

  const isMac = computed(() => platform.value === "macos")
  const isWindows = computed(() => platform.value === "windows")
  const isLinux = computed(() => platform.value === "linux")

  return { platform, isMac, isWindows, isLinux }
}
