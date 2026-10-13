/**
 * 系统相关的类型定义，字段与 Rust 端 `models::SystemInfo` / `models::SystemDetails` 一一对应。
 */

/** 系统资源占用（百分比） */
export interface SystemInfo {
  /** CPU 占用百分比 */
  cpu: number
  /** 内存占用百分比 */
  memory: number
}

/** 系统信息（主机名、系统版本、架构、IP 地址） */
export interface SystemDetails {
  /** 主机名 */
  hostname: string
  /** 操作系统版本 */
  os: string
  /** 系统架构 */
  arch: string
  /** 本机 IPv4 地址列表（不含回环地址） */
  ips: string[]
}
