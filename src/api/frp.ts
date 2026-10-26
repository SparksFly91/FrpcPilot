import { invoke } from "@tauri-apps/api/core"
import type { ApiResponse } from "@/types/api"
import type {
  FrpDownload,
  FrpDownloadPayload,
  FrpRelease,
  FrpcStatus,
} from "@/types/frp"

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
 * 获取 Frp 版本列表（返回所有架构，前端按平台筛选）
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

/**
 * 一键启动 Frpc。
 *
 * 后端会依据所选版本、已启用的服务器与隧道生成 frpc.toml 并拉起独立进程，
 * 随后通过 `frpc-log` / `frpc-exit` 事件回传运行日志与退出信息。
 */
export function startFrpc(downloadId: number) {
  return call<FrpcStatus>("start_frpc", { downloadId })
}

/** 停止 Frpc 服务进程 */
export function stopFrpc() {
  return call<FrpcStatus>("stop_frpc")
}

/** 获取 Frpc 当前运行状态 */
export function getFrpcStatus() {
  return call<FrpcStatus>("get_frpc_status")
}
