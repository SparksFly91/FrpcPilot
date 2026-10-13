use sqlx::SqlitePool;
use tauri::State;
use tauri_helper::auto_collect_command;

use crate::models::{ApiResponse, Tunnel, TunnelPayload};

/// 允许的代理类型（对应 frpc 的 type）
const PROXY_TYPES: [&str; 8] = [
    "tcp", "udp", "http", "https", "stcp", "xtcp", "sudp", "tcpmux",
];

/// 允许的 Proxy Protocol 版本
const PROXY_PROTOCOL_VERSIONS: [&str; 2] = ["v1", "v2"];

/// 允许的健康检查类型
const HEALTH_CHECK_TYPES: [&str; 2] = ["tcp", "http"];

/// 统一查询字段，使用 concat! 保证结果为 'static str（sqlx 0.9 要求）
macro_rules! select_fields {
    () => {
        "id, name, proxy_type, local_ip, local_port, remote_port, custom_domains, subdomain, locations, secret_key, use_encryption, use_compression, bandwidth_limit, proxy_protocol_version, health_check_type, health_check_timeout, health_check_max_failed, health_check_interval, health_check_path, load_balancer_group, load_balancer_group_key, remark, sort, pinned, enabled, created_at, updated_at"
    };
}

/// 校验提交数据，返回错误信息（None 表示通过）
fn validate(payload: &TunnelPayload) -> Option<String> {
    if payload.name.trim().is_empty() {
        return Some("隧道名称不能为空".to_string());
    }
    if payload.local_ip.trim().is_empty() {
        return Some("本地 IP 不能为空".to_string());
    }
    if payload.local_port < 1 || payload.local_port > 65535 {
        return Some("本地端口必须在 1 - 65535 之间".to_string());
    }
    if let Some(remote_port) = payload.remote_port {
        if !(1..=65535).contains(&remote_port) {
            return Some("远程端口必须在 1 - 65535 之间".to_string());
        }
    }
    None
}

/// 归一化代理类型，非法值回退为 tcp
fn normalize_proxy_type(raw: &str) -> String {
    let value = raw.trim().to_ascii_lowercase();
    if PROXY_TYPES.contains(&value.as_str()) {
        value
    } else {
        "tcp".to_string()
    }
}

/// 空字符串转 None，避免写入无意义的空串
fn clean(value: &Option<String>) -> Option<String> {
    value
        .as_ref()
        .map(|v| v.trim().to_string())
        .filter(|v| !v.is_empty())
}

/// 归一化枚举值：合法则返回清理后的值，否则返回 None
fn normalize_choice(value: &Option<String>, allowed: &[&str]) -> Option<String> {
    clean(value)
        .map(|v| v.to_ascii_lowercase())
        .filter(|v| allowed.contains(&v.as_str()))
}

/// 仅保留正整数，非法值（<= 0）返回 None
fn positive(value: Option<i32>) -> Option<i32> {
    value.filter(|v| *v > 0)
}

async fn fetch_all(pool: &SqlitePool) -> Result<Vec<Tunnel>, sqlx::Error> {
    sqlx::query_as::<_, Tunnel>(concat!(
        "SELECT ",
        select_fields!(),
        " FROM tunnels ORDER BY pinned DESC, sort ASC, id ASC"
    ))
    .fetch_all(pool)
    .await
}

async fn fetch_by_id(pool: &SqlitePool, id: i64) -> Result<Tunnel, sqlx::Error> {
    sqlx::query_as::<_, Tunnel>(concat!(
        "SELECT ",
        select_fields!(),
        " FROM tunnels WHERE id = ?"
    ))
    .bind(id)
    .fetch_one(pool)
    .await
}

/// 获取隧道列表
#[tauri::command]
#[auto_collect_command]
pub async fn get_tunnel_list(
    state: State<'_, SqlitePool>,
) -> Result<ApiResponse<Vec<Tunnel>>, String> {
    Ok(match fetch_all(&state).await {
        Ok(list) => ApiResponse::success(list),
        Err(e) => ApiResponse::error(500, &e.to_string()),
    })
}

/// 新增隧道
#[tauri::command]
#[auto_collect_command]
pub async fn create_tunnel(
    state: State<'_, SqlitePool>,
    payload: TunnelPayload,
) -> Result<ApiResponse<Tunnel>, String> {
    if let Some(msg) = validate(&payload) {
        return Ok(ApiResponse::error(400, &msg));
    }

    Ok(match insert_tunnel(&state, &payload).await {
        Ok(tunnel) => ApiResponse::success(tunnel),
        Err(e) => ApiResponse::error(500, &e.to_string()),
    })
}

async fn insert_tunnel(pool: &SqlitePool, payload: &TunnelPayload) -> Result<Tunnel, sqlx::Error> {
    let result = sqlx::query(
        "INSERT INTO tunnels (name, proxy_type, local_ip, local_port, remote_port, custom_domains, \
         subdomain, locations, secret_key, use_encryption, use_compression, bandwidth_limit, \
         proxy_protocol_version, health_check_type, health_check_timeout, health_check_max_failed, \
         health_check_interval, health_check_path, load_balancer_group, load_balancer_group_key, \
         remark, sort, pinned, enabled) \
         VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
    )
    .bind(payload.name.trim())
    .bind(normalize_proxy_type(&payload.proxy_type))
    .bind(payload.local_ip.trim())
    .bind(payload.local_port)
    .bind(payload.remote_port)
    .bind(clean(&payload.custom_domains))
    .bind(clean(&payload.subdomain))
    .bind(clean(&payload.locations))
    .bind(clean(&payload.secret_key))
    .bind(payload.use_encryption)
    .bind(payload.use_compression)
    .bind(clean(&payload.bandwidth_limit))
    .bind(normalize_choice(
        &payload.proxy_protocol_version,
        &PROXY_PROTOCOL_VERSIONS,
    ))
    .bind(normalize_choice(
        &payload.health_check_type,
        &HEALTH_CHECK_TYPES,
    ))
    .bind(positive(payload.health_check_timeout))
    .bind(positive(payload.health_check_max_failed))
    .bind(positive(payload.health_check_interval))
    .bind(clean(&payload.health_check_path))
    .bind(clean(&payload.load_balancer_group))
    .bind(clean(&payload.load_balancer_group_key))
    .bind(clean(&payload.remark))
    .bind(payload.sort)
    .bind(payload.pinned)
    .bind(payload.enabled)
    .execute(pool)
    .await?;

    let id = result.last_insert_rowid();
    fetch_by_id(pool, id).await
}

/// 编辑隧道
#[tauri::command]
#[auto_collect_command]
pub async fn update_tunnel(
    state: State<'_, SqlitePool>,
    payload: TunnelPayload,
) -> Result<ApiResponse<Tunnel>, String> {
    let Some(id) = payload.id else {
        return Ok(ApiResponse::error(400, "缺少隧道 ID"));
    };
    if let Some(msg) = validate(&payload) {
        return Ok(ApiResponse::error(400, &msg));
    }

    Ok(match update_tunnel_row(&state, id, &payload).await {
        Ok(Some(tunnel)) => ApiResponse::success(tunnel),
        Ok(None) => ApiResponse::error(404, "隧道不存在"),
        Err(e) => ApiResponse::error(500, &e.to_string()),
    })
}

async fn update_tunnel_row(
    pool: &SqlitePool,
    id: i64,
    payload: &TunnelPayload,
) -> Result<Option<Tunnel>, sqlx::Error> {
    let result = sqlx::query(
        "UPDATE tunnels SET name = ?, proxy_type = ?, local_ip = ?, local_port = ?, remote_port = ?, \
         custom_domains = ?, subdomain = ?, locations = ?, secret_key = ?, use_encryption = ?, \
         use_compression = ?, bandwidth_limit = ?, proxy_protocol_version = ?, health_check_type = ?, \
         health_check_timeout = ?, health_check_max_failed = ?, health_check_interval = ?, \
         health_check_path = ?, load_balancer_group = ?, load_balancer_group_key = ?, remark = ?, \
         sort = ?, pinned = ?, enabled = ?, updated_at = datetime('now', 'localtime') \
         WHERE id = ?",
    )
    .bind(payload.name.trim())
    .bind(normalize_proxy_type(&payload.proxy_type))
    .bind(payload.local_ip.trim())
    .bind(payload.local_port)
    .bind(payload.remote_port)
    .bind(clean(&payload.custom_domains))
    .bind(clean(&payload.subdomain))
    .bind(clean(&payload.locations))
    .bind(clean(&payload.secret_key))
    .bind(payload.use_encryption)
    .bind(payload.use_compression)
    .bind(clean(&payload.bandwidth_limit))
    .bind(normalize_choice(
        &payload.proxy_protocol_version,
        &PROXY_PROTOCOL_VERSIONS,
    ))
    .bind(normalize_choice(
        &payload.health_check_type,
        &HEALTH_CHECK_TYPES,
    ))
    .bind(positive(payload.health_check_timeout))
    .bind(positive(payload.health_check_max_failed))
    .bind(positive(payload.health_check_interval))
    .bind(clean(&payload.health_check_path))
    .bind(clean(&payload.load_balancer_group))
    .bind(clean(&payload.load_balancer_group_key))
    .bind(clean(&payload.remark))
    .bind(payload.sort)
    .bind(payload.pinned)
    .bind(payload.enabled)
    .bind(id)
    .execute(pool)
    .await?;

    if result.rows_affected() == 0 {
        return Ok(None);
    }

    Ok(Some(fetch_by_id(pool, id).await?))
}

/// 删除隧道
#[tauri::command]
#[auto_collect_command]
pub async fn delete_tunnel(
    state: State<'_, SqlitePool>,
    id: i64,
) -> Result<ApiResponse<()>, String> {
    Ok(
        match sqlx::query("DELETE FROM tunnels WHERE id = ?")
            .bind(id)
            .execute(&*state)
            .await
        {
            Ok(result) if result.rows_affected() == 0 => ApiResponse::error(404, "隧道不存在"),
            Ok(_) => ApiResponse::success_empty(),
            Err(e) => ApiResponse::error(500, &e.to_string()),
        },
    )
}

/// 启用 / 禁用隧道
#[tauri::command]
#[auto_collect_command]
pub async fn set_tunnel_enabled(
    state: State<'_, SqlitePool>,
    id: i64,
    enabled: bool,
) -> Result<ApiResponse<Tunnel>, String> {
    Ok(match toggle_enabled(&state, id, enabled).await {
        Ok(Some(tunnel)) => ApiResponse::success(tunnel),
        Ok(None) => ApiResponse::error(404, "隧道不存在"),
        Err(e) => ApiResponse::error(500, &e.to_string()),
    })
}

async fn toggle_enabled(
    pool: &SqlitePool,
    id: i64,
    enabled: bool,
) -> Result<Option<Tunnel>, sqlx::Error> {
    let result = sqlx::query(
        "UPDATE tunnels SET enabled = ?, updated_at = datetime('now', 'localtime') WHERE id = ?",
    )
    .bind(enabled)
    .bind(id)
    .execute(pool)
    .await?;

    if result.rows_affected() == 0 {
        return Ok(None);
    }

    Ok(Some(fetch_by_id(pool, id).await?))
}

/// 置顶 / 取消置顶隧道
#[tauri::command]
#[auto_collect_command]
pub async fn set_tunnel_pinned(
    state: State<'_, SqlitePool>,
    id: i64,
    pinned: bool,
) -> Result<ApiResponse<Tunnel>, String> {
    Ok(
        match sqlx::query(
            "UPDATE tunnels SET pinned = ?, updated_at = datetime('now', 'localtime') WHERE id = ?",
        )
        .bind(pinned)
        .bind(id)
        .execute(&*state)
        .await
        {
            Ok(result) if result.rows_affected() == 0 => ApiResponse::error(404, "隧道不存在"),
            Ok(_) => match fetch_by_id(&state, id).await {
                Ok(tunnel) => ApiResponse::success(tunnel),
                Err(e) => ApiResponse::error(500, &e.to_string()),
            },
            Err(e) => ApiResponse::error(500, &e.to_string()),
        },
    )
}
