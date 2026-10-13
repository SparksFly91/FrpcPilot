import { invoke } from "@tauri-apps/api/core"
import type { ApiResponse, Server, ServerPayload } from "@/types/server"

/**
 * 调用后端命令并统一解包 ApiResponse。
 *
 * 后端所有服务器相关命令均返回 `ApiResponse<T>`，业务失败时
 * `success === false`，此处统一抛出异常，交由调用方提示。
 */
async function call<T>(command: string, args?: Record<string, unknown>): Promise<T> {
  const res = await invoke<ApiResponse<T>>(command, args)
  if (!res?.success) {
    throw new Error(res?.msg || "操作失败")
  }
  return res.data as T
}

/** 获取服务器列表 */
export function getServerList() {
  return call<Server[]>("get_server_list")
}

/** 新增服务器 */
export function createServer(payload: ServerPayload) {
  return call<Server>("create_server", { payload })
}

/** 编辑服务器 */
export function updateServer(payload: ServerPayload) {
  return call<Server>("update_server", { payload })
}

/** 删除服务器 */
export function deleteServer(id: number) {
  return call<null>("delete_server", { id })
}

/** 启用 / 禁用服务器（启用时后端会自动关闭其它服务器） */
export function setServerEnabled(id: number, enabled: boolean) {
  return call<Server>("set_server_enabled", { id, enabled })
}

/** 置顶 / 取消置顶服务器 */
export function setServerPinned(id: number, pinned: boolean) {
  return call<Server>("set_server_pinned", { id, pinned })
}
