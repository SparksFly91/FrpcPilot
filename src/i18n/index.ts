import { createI18n } from "vue-i18n"
import zhCN from "./locales/zh-CN.json"
import enUS from "./locales/en-US.json"

/** 支持的语言类型 */
export type Language = "zh-CN" | "en-US"

/** 全部语言包 */
export const messages = {
  "zh-CN": zhCN,
  "en-US": enUS,
}

/** 可选语言列表（用于语言切换下拉） */
export const languageOptions: { key: Language; label: string }[] = [
  { key: "zh-CN", label: "简体中文" },
  { key: "en-US", label: "English" },
]

export const DEFAULT_LANGUAGE: Language = "zh-CN"

export const i18n = createI18n({
  legacy: false, // 使用组合式 API
  globalInjection: true, // 允许在模板中使用 $t
  locale: DEFAULT_LANGUAGE,
  fallbackLocale: "en-US",
  messages,
})

/** 切换全局语言 */
export function setI18nLanguage(language: Language) {
  i18n.global.locale.value = language
}

/** 在非组件环境（如路由守卫、store）中获取翻译函数 */
export function translate(key: string, params?: Record<string, unknown>) {
  return i18n.global.t(key, params ?? {})
}

export default i18n
