import { defineStore } from "pinia"
import { message } from "ant-design-vue"
import { isTauri } from "@/utils/window"
import { isAutostartEnabled, setAutostartEnabled } from "@/api/autostart"
import { translate } from "@/i18n"

/**
 * 开机自启：直接读写系统启动项，状态以系统为准，不做本地持久化。
 *
 * - 进入设置页时读一次系统启动项，保证开关展示的就是系统真实状态；
 * - 切换后重新读取一次，写入失败则回滚开关并提示原因，避免界面与系统不一致；
 * - 无法访问系统启动项（浏览器调试 / 插件不可用）时 supported 为 false，开关置灰。
 */
export const useAutostartStore = defineStore("autostart", () => {
  /** 系统启动项是否已登记本应用 */
  const enabled = ref(false)
  /** 读取 / 写入系统启动项期间显示加载态，同时防止重复操作 */
  const loading = ref(isTauri())
  /** 当前环境能否读写系统启动项 */
  const supported = ref(isTauri())

  function errMsg(e: unknown) {
    return e instanceof Error ? e.message : String(e)
  }

  /** 读取系统启动项的真实状态（进入设置页时调用） */
  async function refresh() {
    if (!isTauri()) return
    loading.value = true
    try {
      enabled.value = await isAutostartEnabled()
      supported.value = true
    } catch {
      // 读取失败（如插件未注册、权限缺失）：禁用开关，不展示可能错误的状态
      supported.value = false
    } finally {
      loading.value = false
    }
  }

  /** 切换开机自启：写入系统启动项，成功 / 失败都以系统状态为准并给出提示 */
  async function toggle(value: boolean) {
    if (!supported.value || loading.value || value === enabled.value) return

    const previous = enabled.value
    loading.value = true
    try {
      await setAutostartEnabled(value)
      // 以系统启动项为准；读回失败时先按目标值展示，下次进入设置页会再次校准
      enabled.value = await isAutostartEnabled().catch(() => value)
      message.success(
        translate(
          value ? "settings.general.autostartEnabled" : "settings.general.autostartDisabled",
        ),
      )
    } catch (e) {
      enabled.value = previous
      message.error(translate("settings.general.autostartFailed", { message: errMsg(e) }))
    } finally {
      loading.value = false
    }
  }

  return { enabled, loading, supported, refresh, toggle }
})
