import bg1 from "@/assets/images/bg-1.webp"
import bg2 from "@/assets/images/bg-2.webp"
import bg3 from "@/assets/images/bg-3.webp"
import bg4 from "@/assets/images/bg-4.webp"
import bg5 from "@/assets/images/bg-5.webp"
import bg6 from "@/assets/images/bg-6.webp"
import bg7 from "@/assets/images/bg-7.webp"
import bg8 from "@/assets/images/bg-8.webp"
import bg9 from "@/assets/images/bg-9.webp"

/** 窗口背景图片标识 */
export type BackgroundKey = "none" | "bg-1" | "bg-2" | "bg-3" | "bg-4" | "bg-5" | "bg-6" | "bg-7" | "bg-8" | "bg-9"

export interface BackgroundOption {
  /** 唯一标识 */
  key: BackgroundKey
  /** i18n 文案 key */
  label: string
  /** 图片地址（Vite 打包后为最终资源路径） */
  url: string
}

/** 可选背景列表，新增背景图时在此追加即可 */
export const backgroundOptions: BackgroundOption[] = [
  { key: "none", label: "settings.appearance.none", url: "" },
  { key: "bg-1", label: "settings.appearance.bg1", url: bg1 },
  { key: "bg-2", label: "settings.appearance.bg2", url: bg2 },
  { key: "bg-3", label: "settings.appearance.bg3", url: bg3 },
  { key: "bg-4", label: "settings.appearance.bg4", url: bg4 },
  { key: "bg-5", label: "settings.appearance.bg5", url: bg5 },
  { key: "bg-6", label: "settings.appearance.bg6", url: bg6 },
  { key: "bg-7", label: "settings.appearance.bg7", url: bg7 },
  { key: "bg-8", label: "settings.appearance.bg8", url: bg8 },
  { key: "bg-9", label: "settings.appearance.bg9", url: bg9 },
]

export const DEFAULT_BACKGROUND: BackgroundKey = "none"

/** 根据标识取图片地址 */
export function getBackgroundUrl(key: BackgroundKey): string {
  return backgroundOptions.find((item) => item.key === key)?.url ?? ""
}
