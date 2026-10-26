import { invoke } from "@tauri-apps/api/core"

const desktop = "__TAURI_INTERNALS__" in window
let values: Record<string, string> = {}
let pending: Promise<void> = Promise.resolve()
let saveError: unknown

/** 在加载 Vue 状态仓库前读取安装目录内的设置。 */
export async function initializePreferences() {
  if (!desktop) return
  const saved = await invoke<Record<string, string> | null>("get_preferences")
  if (saved !== null) {
    values = saved
  } else {
    values = {}
    await invoke("set_preferences", { preferences: values })
  }
}

function persist() {
  const snapshot = { ...values }
  pending = pending.then(async () => {
    try {
      await invoke("set_preferences", { preferences: snapshot })
      saveError = undefined
    } catch (error) {
      saveError = error
      console.error("Failed to save preferences", error)
      const { message } = await import("ant-design-vue")
      message.error(`保存设置失败 / Failed to save settings: ${String(error)}`)
    }
  })
}

/** VueUse 存储适配器，将应用偏好设置保存在 data/preferences.json 中。 */
export const preferences: Storage = {
  get length() { return desktop ? Object.keys(values).length : localStorage.length },
  key(index) { return desktop ? Object.keys(values)[index] ?? null : localStorage.key(index) },
  getItem(key) { return desktop ? values[key] ?? null : localStorage.getItem(key) },
  setItem(key, value) {
    if (!desktop) return localStorage.setItem(key, value)
    if (values[key] === value) return
    values[key] = value
    persist()
  },
  removeItem(key) {
    if (!desktop) return localStorage.removeItem(key)
    delete values[key]
    persist()
  },
  clear() {
    if (!desktop) return localStorage.clear()
    values = {}
    persist()
  },
}

export async function flushPreferences() {
  await pending
  if (saveError) throw saveError
}
