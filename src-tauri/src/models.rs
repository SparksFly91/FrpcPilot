use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize)]
pub struct PageResult<T> {
    pub total: i64,
    pub list: Vec<T>,
    pub page: i32,
    pub page_size: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    success: bool,
    code: i32,
    msg: String,
    data: Option<T>,
}

impl<T> ApiResponse<T> {
    // 成功响应
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            code: 0,
            msg: "操作成功".to_string(),
            data: Some(data),
        }
    }
    // 错误响应
    pub fn error(code: i32, message: &str) -> Self {
        Self {
            success: false,
            code,
            msg: message.to_string(),
            data: None,
        }
    }
}

impl ApiResponse<()> {
    // 成功响应，数据为空
    pub fn success_empty() -> Self {
        Self {
            success: true,
            code: 0,
            msg: "操作成功".to_string(),
            data: None,
        }
    }
}

/// 服务器配置（对应 SQLite 中的 servers 表）
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Server {
    pub id: i64,
    /// 名称（仅用于界面展示）
    pub name: String,
    /// 服务器地址，对应 frpc 的 serverAddr，可为 IP 或域名
    pub addr: String,
    /// 服务器端口，对应 frpc 的 serverPort，默认 7000
    pub port: i32,
    /// 鉴权 Token，对应 auth.token
    pub token: Option<String>,
    /// 传输协议，对应 transport.protocol：tcp / kcp / quic / websocket
    pub protocol: String,
    /// 是否启用 TLS，对应 transport.tls.enable
    pub tls_enable: bool,
    /// 备注
    pub remark: Option<String>,
    /// 排序（越小越靠前）
    pub sort: i32,
    /// 是否置顶
    pub pinned: bool,
    /// 是否启用（全局同时只能启用一个）
    pub enabled: bool,
    pub created_at: String,
    pub updated_at: String,
}

/// 新增 / 编辑服务器的提交数据
#[derive(Debug, Serialize, Deserialize)]
pub struct ServerPayload {
    /// 编辑时携带，新增时为空
    pub id: Option<i64>,
    pub name: String,
    pub addr: String,
    pub port: i32,
    pub token: Option<String>,
    pub protocol: String,
    pub tls_enable: bool,
    pub remark: Option<String>,
    pub sort: i32,
    pub pinned: bool,
    pub enabled: bool,
}

/// 隧道代理配置（对应 SQLite 中的 tunnels 表）
///
/// 字段命名参考 frpc 的 proxy 配置，其中 `proxy_type` 序列化为 `type`，
/// 与前端类型保持一致（`type` 是 Rust 关键字，故字段名做映射）。
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Tunnel {
    pub id: i64,
    /// 隧道名称，对应 frpc 的 proxy name
    pub name: String,
    /// 代理类型，对应 frpc 的 type：tcp / udp / http / https / stcp / xtcp / sudp / tcpmux
    #[serde(rename = "type")]
    pub proxy_type: String,
    /// 本地 IP，对应 localIP
    pub local_ip: String,
    /// 本地端口，对应 localPort
    pub local_port: i32,
    /// 远程端口，对应 remotePort（tcp / udp 必填）
    pub remote_port: Option<i32>,
    /// 自定义域名，对应 customDomains（多个以英文逗号分隔）
    pub custom_domains: Option<String>,
    /// 子域名，对应 subdomain
    pub subdomain: Option<String>,
    /// URL 前缀，对应 locations（多个以英文逗号分隔）
    pub locations: Option<String>,
    /// 密钥，对应 secretKey（stcp / xtcp / sudp）
    pub secret_key: Option<String>,
    /// 传输加密，对应 transport.useEncryption
    pub use_encryption: bool,
    /// 传输压缩，对应 transport.useCompression
    pub use_compression: bool,
    /// 带宽限制，对应 transport.bandwidthLimit
    pub bandwidth_limit: Option<String>,
    /// Proxy Protocol 版本，对应 transport.proxyProtocolVersion
    pub proxy_protocol_version: Option<String>,
    /// 健康检查类型，对应 healthCheck.type：tcp / http
    pub health_check_type: Option<String>,
    /// 健康检查超时（秒），对应 healthCheck.timeoutSeconds
    pub health_check_timeout: Option<i32>,
    /// 健康检查最大失败次数，对应 healthCheck.maxFailed
    pub health_check_max_failed: Option<i32>,
    /// 健康检查间隔（秒），对应 healthCheck.intervalSeconds
    pub health_check_interval: Option<i32>,
    /// 健康检查路径，对应 healthCheck.path
    pub health_check_path: Option<String>,
    /// 负载均衡组，对应 loadBalancer.group
    pub load_balancer_group: Option<String>,
    /// 负载均衡组密钥，对应 loadBalancer.groupKey
    pub load_balancer_group_key: Option<String>,
    /// 备注
    pub remark: Option<String>,
    /// 排序（越小越靠前）
    pub sort: i32,
    /// 是否置顶
    pub pinned: bool,
    /// 是否启用（可同时启用多个）
    pub enabled: bool,
    pub created_at: String,
    pub updated_at: String,
}

/// 新增 / 编辑隧道的提交数据
#[derive(Debug, Serialize, Deserialize)]
pub struct TunnelPayload {    /// 编辑时携带，新增时为空
    pub id: Option<i64>,
    pub name: String,
    #[serde(rename = "type")]
    pub proxy_type: String,
    pub local_ip: String,
    pub local_port: i32,
    pub remote_port: Option<i32>,
    pub custom_domains: Option<String>,
    pub subdomain: Option<String>,
    pub locations: Option<String>,
    pub secret_key: Option<String>,
    pub use_encryption: bool,
    pub use_compression: bool,
    pub bandwidth_limit: Option<String>,
    pub proxy_protocol_version: Option<String>,
    pub health_check_type: Option<String>,
    pub health_check_timeout: Option<i32>,
    pub health_check_max_failed: Option<i32>,
    pub health_check_interval: Option<i32>,
    pub health_check_path: Option<String>,
    pub load_balancer_group: Option<String>,
    pub load_balancer_group_key: Option<String>,
    pub remark: Option<String>,
    pub sort: i32,
    pub pinned: bool,
    pub enabled: bool,
}

/// 已下载 / 已导入的 Frp 版本（对应 SQLite 中的 frp_downloads 表）
///
/// 一个发行包（version + platform）同时包含 frpc 与 frps 两个可执行文件，
/// 故 `frpc_path` / `frps_path` 分别记录解压（或导入）后的可执行文件路径。
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct FrpDownload {
    pub id: i64,
    /// 版本号，如 v0.71.0
    pub version: String,
    /// 操作系统，如 windows / darwin / linux
    pub os: String,
    /// CPU 架构，如 amd64 / arm64
    pub arch: String,
    /// 平台标识（os_arch），如 windows_amd64
    pub platform: String,
    /// 发行包文件名
    pub file_name: String,
    /// 压缩包保存路径（导入的记录可能为空）
    pub archive_path: Option<String>,
    /// 解压 / 导入后的 frpc 可执行文件路径
    pub frpc_path: Option<String>,
    /// 解压 / 导入后的 frps 可执行文件路径
    pub frps_path: Option<String>,
    /// 文件大小（字节）
    pub file_size: i64,
    /// 来源：download（应用内下载）/ import（本地导入）
    pub source: String,
    pub created_at: String,
    pub updated_at: String,
}

/// GitHub 上的 Frp 发行包（前端卡片展示用）
#[derive(Debug, Serialize, Deserialize)]
pub struct FrpRelease {
    /// 版本号，如 v0.71.0
    pub version: String,
    /// 操作系统
    pub os: String,
    /// CPU 架构
    pub arch: String,
    /// 平台标识（os_arch）
    pub platform: String,
    /// 发行包文件名
    pub file_name: String,
    /// 文件大小（字节）
    pub size: i64,
    /// GitHub 下载次数
    pub download_count: i64,
    /// 发布日期（ISO 8601）
    pub published_at: String,
    /// GitHub 原始下载地址
    pub download_url: String,
    /// 镜像加速后的下载地址
    pub mirror_url: String,
    /// 是否已下载 / 已导入
    pub downloaded: bool,
    /// 是否匹配当前运行平台（用于前端默认过滤）
    pub is_current: bool,
    /// 是否为仅本机存在的记录（未匹配到发行包）
    pub local_only: bool,
    /// 本机记录 ID（用于删除）
    pub local_id: Option<i64>,
    /// 本机 frpc 可执行文件路径
    pub frpc_path: Option<String>,
    /// 本机 frps 可执行文件路径
    pub frps_path: Option<String>,
}

/// 下载 / 导入请求
#[derive(Debug, Serialize, Deserialize)]
pub struct FrpDownloadPayload {
    pub version: String,
    pub os: String,
    pub arch: String,
    pub platform: String,
    pub file_name: String,
    /// GitHub 原始下载地址（后端会拼接镜像前缀）
    pub download_url: String,
}

/// Frp 下载进度事件（通过 `frp-download-progress` 事件推送）
#[derive(Debug, Clone, Serialize)]
pub struct FrpDownloadProgress {
    /// 事件键，与发行包文件名一致，用于前端定位卡片
    pub key: String,
    pub version: String,
    pub platform: String,
    /// 已下载字节数
    pub downloaded: u64,
    /// 总字节数（0 表示未知）
    pub total: u64,
    /// 进度百分比（0 - 100）
    pub percent: u8,
    /// 状态：downloading / extracting / done / error
    pub status: String,
    /// 附加信息（错误原因等）
    pub message: Option<String>,
}

/// 系统资源占用（百分比，0 - 100）
#[derive(Debug, Serialize, Deserialize)]
pub struct SystemInfo {
    /// CPU 占用百分比
    pub cpu: u32,
    /// 内存占用百分比
    pub memory: u32,
}

/// 系统信息（主机名、系统版本、架构、IP 地址）
#[derive(Debug, Serialize, Deserialize)]
pub struct SystemDetails {
    /// 主机名
    pub hostname: String,
    /// 操作系统版本
    pub os: String,
    /// 系统架构
    pub arch: String,
    /// 本机 IPv4 地址列表（不含回环地址）
    pub ips: Vec<String>,
}

/// Frpc 运行状态（仪表盘快速操作区 / 侧边栏状态点使用）
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FrpcStatus {
    /// 是否正在运行
    pub running: bool,
    /// 当前运行的 Frpc 版本号
    pub version: Option<String>,
    /// frpc 可执行文件路径
    pub frpc_path: Option<String>,
    /// 本次运行生成的 frpc.toml 路径
    pub config_path: Option<String>,
    /// frpc 进程 PID
    pub pid: Option<u32>,
}

/// Frpc 实时日志事件（通过 `frpc-log` 事件推送）
#[derive(Debug, Clone, Serialize)]
pub struct FrpcLogEvent {
    /// frpc 输出中的时间戳（HH:MM:SS，可能为空）
    pub time: Option<String>,
    /// 日志级别：INFO / WARN / ERROR / DEBUG / TRACE
    pub level: String,
    /// 来源：stdout / stderr / system（system 为 Frpilot 自身补充的日志）
    pub source: String,
    /// 日志内容
    pub message: String,
}

/// Frpc 进程退出事件（通过 `frpc-exit` 事件推送）
#[derive(Debug, Clone, Serialize)]
pub struct FrpcExitEvent {
    /// 退出原因：stopped（用户主动停止）/ exited（进程自行退出）
    pub reason: String,
    /// 退出码（主动终止时可能为空）
    pub code: Option<i32>,
    /// 退出时对应的 Frpc 版本号
    pub version: Option<String>,
}

