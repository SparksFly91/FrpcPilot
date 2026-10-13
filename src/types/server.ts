/**
 * 服务器相关的类型定义，字段与 Rust 端 `models::Server` / `models::ServerPayload` 一一对应。
 */

export type { ApiResponse } from "./api"

/** 传输协议，对应 frpc 的 transport.protocol */
export type ServerProtocol = "tcp" | "kcp" | "quic" | "websocket"

/** 服务器配置 */
export interface Server {
  id: number
  /** 名称（仅用于界面展示） */
  name: string
  /** 服务器地址 serverAddr，IP 或域名 */
  addr: string
  /** 服务器端口 serverPort */
  port: number
  /** 鉴权 Token auth.token */
  token: string | null
  /** 传输协议 transport.protocol */
  protocol: ServerProtocol
  /** 是否启用 TLS transport.tls.enable */
  tls_enable: boolean
  /** 备注 */
  remark: string | null
  /** 排序（越小越靠前） */
  sort: number
  /** 是否置顶 */
  pinned: boolean
  /** 是否启用（全局同时只能启用一个） */
  enabled: boolean
  created_at: string
  updated_at: string
}

/** 新增 / 编辑服务器提交数据 */
export interface ServerPayload {
  /** 编辑时携带，新增时省略 */
  id?: number
  name: string
  addr: string
  port: number
  token?: string | null
  protocol: ServerProtocol
  tls_enable: boolean
  remark?: string | null
  sort: number
  pinned: boolean
  enabled: boolean
}

/** 协议下拉选项 */
export const SERVER_PROTOCOLS: { value: ServerProtocol; label: string }[] = [
  { value: "tcp", label: "TCP" },
  { value: "kcp", label: "KCP" },
  { value: "quic", label: "QUIC" },
  { value: "websocket", label: "WebSocket" },
]
