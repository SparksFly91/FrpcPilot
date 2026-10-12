import { defineStore } from "pinia"
import zhCN from "ant-design-vue/es/locale/zh_CN"
import enUS from "ant-design-vue/es/locale/en_US"
import { setI18nLanguage, type Language } from "@/i18n"

/** Frpc 运行状态 */
export type RunStatus = "running" | "error" | "stopped"

/**
 * 全局应用配置：语言、主题、Frpc 运行状态
 */
export const useAppStore = defineStore("app", () => {
  // 语言（持久化到 localStorage）
  const language = useStorage<Language>("frpilot:language", "zh-CN")

  // 明暗主题：useDark 会在 <html> 上切换 .dark 类名
  const isDark = useDark({
    storageKey: "frpilot:theme",
    selector: "html",
    attribute: "class",
    valueDark: "dark",
    valueLight: "",
  })
  const toggleDark = useToggle(isDark)

  // Frpc 运行状态（后续由后端驱动）
  const runStatus = ref<RunStatus>("stopped")

  // Ant Design Vue 组件库语言包
  const antdLocale = computed(() => (language.value === "zh-CN" ? zhCN : enUS))

  // 语言变化时同步到 vue-i18n
  watch(
    language,
    (value) => setI18nLanguage(value),
    { immediate: true },
  )

  function setLanguage(value: Language) {
    language.value = value
  }

  function toggleLanguage() {
    language.value = language.value === "zh-CN" ? "en-US" : "zh-CN"
  }

  return {
    language,
    isDark,
    toggleDark,
    runStatus,
    antdLocale,
    setLanguage,
    toggleLanguage,
  }
})
