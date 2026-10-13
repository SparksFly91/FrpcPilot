import { invoke } from "@tauri-apps/api/core"
import type { ApiResponse } from "@/types/api"
import type { Tunnel, TunnelPayload } from "@/types/tunnel"

/**
 * 调用后端命令并统一解包 ApiResponse。
 *
 * 后端所有隧道相关命令均返回 `ApiResponse<T>`，业务失败时
 * `success === false`，此处统一抛出异常，交由调用方提示。
 */
async function call<T>(command: string, args?: Record<string, unknown>): Promise<T> {
  const res = await invoke<ApiResponse<T>>(command, args)
  if (!res?.success) {
    throw new Error(res?.msg || "操作失败")
  }
  return res.data as T
}

/** 获取隧道列表 */
export function getTunnelList() {
  return call<Tunnel[]>("get_tunnel_list")
}

/** 新增隧道 */
export function createTunnel(payload: TunnelPayload) {
  return call<Tunnel>("create_tunnel", { payload })
}

/** 编辑隧道 */
export function updateTunnel(payload: TunnelPayload) {
  return call<Tunnel>("update_tunnel", { payload })
}

/** 删除隧道 */
export function deleteTunnel(id: number) {
  return call<null>("delete_tunnel", { id })
}

/** 启用 / 禁用隧道 */
export function setTunnelEnabled(id: number, enabled: boolean) {
  return call<Tunnel>("set_tunnel_enabled", { id, enabled })
}

/** 置顶 / 取消置顶隧道 */
export function setTunnelPinned(id: number, pinned: boolean) {
  return call<Tunnel>("set_tunnel_pinned", { id, pinned })
}
