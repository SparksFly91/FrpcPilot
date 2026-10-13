/**
 * 隧道相关的类型定义，字段与 Rust 端 `models::Tunnel` / `models::TunnelPayload` 一一对应。
 * 字段命名参考 frpc 的 proxy 配置（如 localIP / remotePort / customDomains 等）。
 */

/** 代理类型，对应 frpc 的 proxy type */
export type TunnelType =
  | "tcp"
  | "udp"
  | "http"
  | "https"
  | "stcp"
  | "xtcp"
  | "sudp"
  | "tcpmux"

/** 隧道配置 */
export interface Tunnel {
  id: number
  /** 隧道名称，对应 frpc 的 proxy name */
  name: string
  /** 代理类型，对应 frpc 的 type */
  type: TunnelType
  /** 本地 IP，对应 localIP */
  local_ip: string
  /** 本地端口，对应 localPort */
  local_port: number
  /** 远程端口，对应 remotePort（tcp / udp 必填） */
  remote_port: number | null
  /** 自定义域名，对应 customDomains（多个以英文逗号分隔） */
  custom_domains: string | null
  /** 子域名，对应 subdomain */
  subdomain: string | null
  /** URL 前缀，对应 locations（多个以英文逗号分隔） */
  locations: string | null
  /** 密钥，对应 secretKey（stcp / xtcp / sudp） */
  secret_key: string | null
  /** 传输加密，对应 transport.useEncryption */
  use_encryption: boolean
  /** 传输压缩，对应 transport.useCompression */
  use_compression: boolean
  /** 带宽限制，对应 transport.bandwidthLimit */
  bandwidth_limit: string | null
  /** Proxy Protocol 版本，对应 transport.proxyProtocolVersion */
  proxy_protocol_version: string | null
  /** 健康检查类型，对应 healthCheck.type：tcp / http */
  health_check_type: string | null
  /** 健康检查超时（秒），对应 healthCheck.timeoutSeconds */
  health_check_timeout: number | null
  /** 健康检查最大失败次数，对应 healthCheck.maxFailed */
  health_check_max_failed: number | null
  /** 健康检查间隔（秒），对应 healthCheck.intervalSeconds */
  health_check_interval: number | null
  /** 健康检查路径，对应 healthCheck.path */
  health_check_path: string | null
  /** 负载均衡组，对应 loadBalancer.group */
  load_balancer_group: string | null
  /** 负载均衡组密钥，对应 loadBalancer.groupKey */
  load_balancer_group_key: string | null
  /** 备注 */
  remark: string | null
  /** 排序（越小越靠前） */
  sort: number
  /** 是否置顶 */
  pinned: boolean
  /** 是否启用（可同时启用多个） */
  enabled: boolean
  created_at: string
  updated_at: string
}

/** 新增 / 编辑隧道提交数据 */
export interface TunnelPayload {
  /** 编辑时携带，新增时省略 */
  id?: number
  name: string
  type: TunnelType
  local_ip: string
  local_port: number
  remote_port?: number | null
  custom_domains?: string | null
  subdomain?: string | null
  locations?: string | null
  secret_key?: string | null
  use_encryption: boolean
  use_compression: boolean
  bandwidth_limit?: string | null
  proxy_protocol_version?: string | null
  health_check_type?: string | null
  health_check_timeout?: number | null
  health_check_max_failed?: number | null
  health_check_interval?: number | null
  health_check_path?: string | null
  load_balancer_group?: string | null
  load_balancer_group_key?: string | null
  remark?: string | null
  sort: number
  pinned: boolean
  enabled: boolean
}

/** 代理类型列表 */
export const TUNNEL_TYPES: TunnelType[] = [
  "tcp",
  "udp",
  "http",
  "https",
  "stcp",
  "xtcp",
  "sudp",
  "tcpmux",
]

/** 需要填写远程端口的代理类型 */
export const TYPES_WITH_REMOTE_PORT: TunnelType[] = ["tcp", "udp"]

/** 需要填写自定义域名 / 子域名的代理类型 */
export const TYPES_WITH_DOMAIN: TunnelType[] = ["http", "https", "tcpmux"]

/** 需要填写密钥的代理类型 */
export const TYPES_WITH_SECRET: TunnelType[] = ["stcp", "xtcp", "sudp"]
