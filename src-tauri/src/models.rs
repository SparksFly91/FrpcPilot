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

