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
pub struct TunnelPayload {
    /// 编辑时携带，新增时为空
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

