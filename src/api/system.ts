import { invoke } from "@tauri-apps/api/core"
import type { ApiResponse } from "@/types/api"
import type { SystemDetails, SystemInfo } from "@/types/system"

/**
 * 调用后端命令并统一解包 ApiResponse。
 *
 * 后端系统相关命令均返回 `ApiResponse<T>`，业务失败时
 * `success === false`，此处统一抛出异常，交由调用方处理。
 */
async function call<T>(command: string, args?: Record<string, unknown>): Promise<T> {
  const res = await invoke<ApiResponse<T>>(command, args)
  if (!res?.success) {
    throw new Error(res?.msg || "操作失败")
  }
  return res.data as T
}

/** 获取系统资源占用（CPU / 内存百分比） */
export function getSystemInfo() {
  return call<SystemInfo>("get_system_info")
}

/** 获取系统信息（主机名、系统版本、架构、IP 地址） */
export function getSystemDetails() {
  return call<SystemDetails>("get_system_details")
}
