import { defineStore } from "pinia"
import zhCN from "ant-design-vue/es/locale/zh_CN"
import enUS from "ant-design-vue/es/locale/en_US"
import { setI18nLanguage, type Language } from "@/i18n"
import {
  DEFAULT_BACKGROUND,
  getBackgroundUrl,
  type BackgroundKey,
} from "@/config/backgrounds"

/** Frpc 运行状态 */
export type RunStatus = "running" | "error" | "stopped"

/**
 * 全局应用配置：语言、主题、窗口背景、Frpc 运行状态
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

  // 窗口背景图片（持久化）
  const background = useStorage<BackgroundKey>("frpilot:background", DEFAULT_BACKGROUND)
  const backgroundUrl = computed(() => getBackgroundUrl(background.value))

  // 把背景图同步到 <html>（整窗背景，避免元素高度误差造成的上下留白）
  watchEffect(() => {
    const root = document.documentElement
    const url = backgroundUrl.value
    if (url) {
      root.style.setProperty("--app-bg-image", `url('${url}')`)
      root.classList.add("has-bg-image")
    } else {
      root.classList.remove("has-bg-image")
      root.style.removeProperty("--app-bg-image")
    }
  })

  // 磨砂玻璃透明度（0 - 100，数值越大越透明，持久化）
  const cardTransparency = useStorage<number>("frpilot:card-transparency", 55)
  const modalTransparency = useStorage<number>("frpilot:modal-transparency", 38)

  /** 透明度百分比 → 背景 alpha（越大越透明，alpha 越小；并做范围兜底） */
  function alphaFromTransparency(value: number) {
    const percent = Math.min(100, Math.max(0, Number(value) || 0))
    return (100 - percent) / 100
  }

  const cardAlpha = computed(() => alphaFromTransparency(cardTransparency.value))
  const modalAlpha = computed(() => alphaFromTransparency(modalTransparency.value))

  // 把透明度实时写入 <html> 的 CSS 变量，供磨砂玻璃样式（卡片 / 弹窗）即时生效
  watchEffect(() => {
    const root = document.documentElement
    root.style.setProperty("--app-card-alpha", String(cardAlpha.value))
    root.style.setProperty("--app-card-alpha-hover", String(Math.min(cardAlpha.value + 0.15, 0.95)))
    root.style.setProperty("--app-modal-alpha", String(modalAlpha.value))
  })

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

  function setBackground(value: BackgroundKey) {
    background.value = value
  }

  return {
    language,
    isDark,
    toggleDark,
    background,
    backgroundUrl,
    cardTransparency,
    modalTransparency,
    runStatus,
    antdLocale,
    setLanguage,
    toggleLanguage,
    setBackground,
  }
})
