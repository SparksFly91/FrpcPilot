/**
 * 与 Rust 端统一响应结构 `models::ApiResponse<T>` 一一对应的通用类型。
 */

export interface ApiResponse<T = unknown> {
  success: boolean
  code: number
  msg: string
  data: T | null
}
