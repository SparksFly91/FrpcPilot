import { defineStore } from "pinia"
import { message } from "ant-design-vue"
import type { Update } from "@tauri-apps/plugin-updater"
import { checkForAppUpdate, relaunchApp } from "@/api/updater"
import { isTauri } from "@/utils/window"
import { translate } from "@/i18n"

/** 供弹窗展示的更新信息（纯数据，不含方法） */
export interface UpdateInfo {
  version: string
  currentVersion: string
  date?: string
  body?: string
}

/**
 * 应用更新状态：检查 / 忽略版本 / 下载安装。
 *
 * 更新弹窗由 {@link UpdateModal} 全局渲染，顶部栏与「关于」页共用此处的检查逻辑。
 */
export const useUpdateStore = defineStore("update", () => {
  /** 用户选择「忽略该版本」后持久化的版本号，静默检查时不再提示 */
  const ignoredVersion = useStorage<string>("frpilot:ignored-update", "")

  const checking = ref(false)
  const installing = ref(false)
  const modalOpen = ref(false)
  const info = ref<UpdateInfo | null>(null)

  /** 下载进度 */
  const downloaded = ref(0)
  const contentLength = ref<number | null>(null)
  const progress = ref(0)

  /** 持有原始 Update 资源（含下载 / 安装方法），不放进响应式以免代理破坏其方法 */
  let pending: Update | null = null

  const hasUpdate = computed(() => info.value !== null)

  function errMsg(e: unknown) {
    return e instanceof Error ? e.message : String(e)
  }

  function resetProgress() {
    downloaded.value = 0
    contentLength.value = null
    progress.value = 0
  }

  /**
   * 检查更新。
   *
   * @param silent 静默检查（启动时自动调用）：无更新或已忽略该版本时都不打扰用户。
   */
  async function checkForUpdate(silent = false) {
    if (checking.value) return

    if (!isTauri()) {
      if (!silent) message.info(translate("update.desktopOnly"))
      return
    }

    checking.value = true
    try {
      const update = await checkForAppUpdate()

      if (!update) {
        if (!silent) message.success(translate("update.upToDate"))
        return
      }

      // 静默检查时跳过用户已忽略的版本
      if (silent && ignoredVersion.value === update.version) {
        await update.close()
        return
      }

      pending = update
      info.value = {
        version: update.version,
        currentVersion: update.currentVersion,
        date: update.date,
        body: update.body,
      }
      resetProgress()
      modalOpen.value = true
    } catch (e) {
      if (!silent) {
        message.error(translate("update.checkFailed", { message: errMsg(e) }))
      }
    } finally {
      checking.value = false
    }
  }

  /** 放弃当前更新：关闭弹窗并释放后端资源 */
  async function discard() {
    const update = pending
    pending = null
    info.value = null
    modalOpen.value = false
    resetProgress()
    try {
      await update?.close()
    } catch {
      // 资源释放失败不影响交互
    }
  }

  /** 取消（本次不更新，下次启动仍会提示） */
  function cancel() {
    void discard()
  }

  /** 忽略该版本（持久化，静默检查不再提示） */
  async function ignoreVersion() {
    if (info.value) ignoredVersion.value = info.value.version
    await discard()
    message.success(translate("update.ignoreSuccess"))
  }

  /** 下载并安装更新，随后重启应用以运行新版本 */
  async function install() {
    if (!pending || installing.value) return

    installing.value = true
    resetProgress()
    try {
      await pending.downloadAndInstall((event) => {
        switch (event.event) {
          case "Started":
            contentLength.value = event.data.contentLength ?? null
            break
          case "Progress":
            downloaded.value += event.data.chunkLength
            progress.value = contentLength.value
              ? Math.min(100, Math.round((downloaded.value / contentLength.value) * 100))
              : Math.min(99, progress.value + 1)
            break
          case "Finished":
            progress.value = 100
            break
        }
      })

      // Windows 会在安装后自动退出；macOS / Linux 需要主动重启
      pending = null
      info.value = null
      modalOpen.value = false
      await relaunchApp()
    } catch (e) {
      message.error(errMsg(e))
    } finally {
      installing.value = false
    }
  }

  return {
    ignoredVersion,
    checking,
    installing,
    modalOpen,
    info,
    downloaded,
    contentLength,
    progress,
    hasUpdate,
    checkForUpdate,
    cancel,
    ignoreVersion,
    install,
  }
})
