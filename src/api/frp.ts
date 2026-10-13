import { invoke } from "@tauri-apps/api/core"
import type { ApiResponse } from "@/types/api"
import type { FrpDownload, FrpDownloadPayload, FrpRelease } from "@/types/frp"

/**
 * 调用后端命令并统一解包 ApiResponse。
 *
 * 后端所有 Frp 下载相关命令均返回 `ApiResponse<T>`，业务失败时
 * `success === false`，此处统一抛出异常，交由调用方提示。
 */
async function call<T>(command: string, args?: Record<string, unknown>): Promise<T> {
  const res = await invoke<ApiResponse<T>>(command, args)
  if (!res?.success) {
    throw new Error(res?.msg || "操作失败")
  }
  return res.data as T
}

/**
 * 获取 Frp 版本列表（返回所有架构，前端按开关过滤）
 */
export function getFrpReleases() {
  return call<FrpRelease[]>("get_frp_releases")
}

/** 下载并解压指定 Frp 发行包 */
export function downloadFrp(payload: FrpDownloadPayload) {
  return call<FrpDownload>("download_frp", { payload })
}

/** 获取本机已下载 / 已导入的 Frp 版本列表（仪表盘选择 frpc 版本用） */
export function getFrpDownloads() {
  return call<FrpDownload[]>("get_frp_downloads")
}

/** 删除已下载 / 已导入的版本 */
export function deleteFrp(id: number) {
  return call<null>("delete_frp", { id })
}

/** 导入本地已下载的 frpc / frps（返回导入后的记录） */
export function importFrp() {
  return call<FrpDownload[]>("import_frp")
}
