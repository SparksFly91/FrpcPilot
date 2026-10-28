import { invoke } from "@tauri-apps/api/core"
import type { ApiResponse } from "@/types/api"

async function call<T>(command: string): Promise<T> {
  const response = await invoke<ApiResponse<T>>(command)
  if (!response?.success) {
    throw new Error(response?.msg || "操作失败")
  }
  return response.data as T
}

/** 让用户选择并持久化一张自定义背景图片，取消时返回 null。 */
export function chooseCustomBackground() {
  return call<string | null>("choose_custom_background")
}

/** 获取已保存的自定义背景图片路径。 */
export function getCustomBackground() {
  return call<string | null>("get_custom_background")
}
