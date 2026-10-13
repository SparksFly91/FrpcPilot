import bg1 from "@/assets/images/bg-1.webp"
import bg2 from "@/assets/images/bg-2.webp"
import bg3 from "@/assets/images/bg-3.webp"

/** 窗口背景图片标识 */
export type BackgroundKey = "none" | "bg-1" | "bg-2" | "bg-3"

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
]

export const DEFAULT_BACKGROUND: BackgroundKey = "none"

/** 根据标识取图片地址 */
export function getBackgroundUrl(key: BackgroundKey): string {
  return backgroundOptions.find((item) => item.key === key)?.url ?? ""
}
