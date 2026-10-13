/**
 * Frp 下载相关的类型定义，字段与 Rust 端 `models::FrpRelease` / `models::FrpDownload`
 * 等一一对应。
 */

/** GitHub 上的 Frp 发行包（卡片展示用） */
export interface FrpRelease {
  /** 版本号，如 v0.71.0 */
  version: string
  /** 操作系统，如 windows / darwin / linux */
  os: string
  /** CPU 架构，如 amd64 / arm64 */
  arch: string
  /** 平台标识（os_arch），如 windows_amd64 */
  platform: string
  /** 发行包文件名 */
  file_name: string
  /** 文件大小（字节） */
  size: number
  /** GitHub 下载次数 */
  download_count: number
  /** 发布日期（ISO 8601） */
  published_at: string
  /** GitHub 原始下载地址 */
  download_url: string
  /** 镜像加速后的下载地址 */
  mirror_url: string
  /** 是否已下载 / 已导入 */
  downloaded: boolean
  /** 是否匹配当前运行平台（用于前端默认过滤） */
  is_current: boolean
  /** 是否为仅本机存在的记录（未匹配到发行包） */
  local_only: boolean
  /** 本机记录 ID（用于删除） */
  local_id: number | null
  /** 本机 frpc 可执行文件路径 */
  frpc_path: string | null
  /** 本机 frps 可执行文件路径 */
  frps_path: string | null
}

/** 已下载 / 已导入的 Frp 版本 */
export interface FrpDownload {
  id: number
  version: string
  os: string
  arch: string
  platform: string
  file_name: string
  archive_path: string | null
  frpc_path: string | null
  frps_path: string | null
  file_size: number
  /** 来源：download（应用内下载）/ import（本地导入） */
  source: string
  created_at: string
  updated_at: string
}

/** 下载请求 */
export interface FrpDownloadPayload {
  version: string
  os: string
  arch: string
  platform: string
  file_name: string
  /** GitHub 原始下载地址（后端会拼接镜像前缀） */
  download_url: string
}

/** 下载进度状态 */
export type FrpDownloadStatus = "downloading" | "extracting" | "done" | "error"

/** Frp 下载进度事件 */
export interface FrpDownloadProgress {
  /** 事件键，与发行包文件名一致 */
  key: string
  version: string
  platform: string
  /** 已下载字节数 */
  downloaded: number
  /** 总字节数（0 表示未知） */
  total: number
  /** 进度百分比（0 - 100） */
  percent: number
  status: FrpDownloadStatus
  /** 附加信息（错误原因等） */
  message: string | null
}

/** Frpc 运行状态 */
export interface FrpcStatus {
  /** 是否正在运行 */
  running: boolean
  /** 当前运行的 Frpc 版本号 */
  version: string | null
  /** frpc 可执行文件路径 */
  frpc_path: string | null
  /** 本次运行生成的 frpc.toml 路径 */
  config_path: string | null
  /** frpc 进程 PID */
  pid: number | null
}

/** Frpc 实时日志事件（`frpc-log`） */
export interface FrpcLogEvent {
  /** frpc 输出中的时间戳（HH:MM:SS，可能为空） */
  time: string | null
  /** 日志级别：INFO / WARN / ERROR / DEBUG / TRACE */
  level: string
  /** 来源：stdout / stderr / system */
  source: string
  /** 日志内容 */
  message: string
}

/** Frpc 进程退出事件（`frpc-exit`） */
export interface FrpcExitEvent {
  /** 退出原因：stopped（用户主动停止）/ exited（进程自行退出） */
  reason: string
  /** 退出码（主动终止时可能为空） */
  code: number | null
  /** 退出时对应的 Frpc 版本号 */
  version: string | null
}
