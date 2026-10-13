use sqlx::SqlitePool;
use tauri::State;
use tauri_helper::auto_collect_command;

use crate::models::{ApiResponse, Server, ServerPayload};

/// 允许的传输协议（对应 transport.protocol）
const PROTOCOLS: [&str; 4] = ["tcp", "kcp", "quic", "websocket"];

/// 统一查询字段，使用 concat! 保证结果为 'static str（sqlx 0.9 要求）
macro_rules! select_fields {
    () => {
        "id, name, addr, port, token, protocol, tls_enable, remark, sort, pinned, enabled, created_at, updated_at"
    };
}

/// 校验提交数据，返回错误信息（None 表示通过）
fn validate(payload: &ServerPayload) -> Option<String> {
    if payload.name.trim().is_empty() {
        return Some("服务器名称不能为空".to_string());
    }
    if payload.addr.trim().is_empty() {
        return Some("服务器地址不能为空".to_string());
    }
    if payload.port < 1 || payload.port > 65535 {
        return Some("端口号必须在 1 - 65535 之间".to_string());
    }
    None
}

/// 归一化协议，非法值回退为 tcp
fn normalize_protocol(raw: &str) -> String {
    let value = raw.trim().to_ascii_lowercase();
    if PROTOCOLS.contains(&value.as_str()) {
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

async fn fetch_all(pool: &SqlitePool) -> Result<Vec<Server>, sqlx::Error> {
    sqlx::query_as::<_, Server>(concat!(
        "SELECT ",
        select_fields!(),
        " FROM servers ORDER BY pinned DESC, sort ASC, id ASC"
    ))
    .fetch_all(pool)
    .await
}

async fn fetch_by_id(pool: &SqlitePool, id: i64) -> Result<Server, sqlx::Error> {
    sqlx::query_as::<_, Server>(concat!(
        "SELECT ",
        select_fields!(),
        " FROM servers WHERE id = ?"
    ))
    .bind(id)
    .fetch_one(pool)
    .await
}

/// 取消其它服务器的启用状态（启用互斥）
async fn disable_others<'e, E>(executor: E, id: i64) -> Result<(), sqlx::Error>
where
    E: sqlx::Executor<'e, Database = sqlx::Sqlite>,
{
    sqlx::query(
        "UPDATE servers SET enabled = 0, updated_at = datetime('now', 'localtime') WHERE enabled = 1 AND id <> ?",
    )
    .bind(id)
    .execute(executor)
    .await?;
    Ok(())
}

/// 获取服务器列表
#[tauri::command]
#[auto_collect_command]
pub async fn get_server_list(
    state: State<'_, SqlitePool>,
) -> Result<ApiResponse<Vec<Server>>, String> {
    Ok(match fetch_all(&state).await {
        Ok(list) => ApiResponse::success(list),
        Err(e) => ApiResponse::error(500, &e.to_string()),
    })
}

/// 新增服务器
#[tauri::command]
#[auto_collect_command]
pub async fn create_server(
    state: State<'_, SqlitePool>,
    payload: ServerPayload,
) -> Result<ApiResponse<Server>, String> {
    if let Some(msg) = validate(&payload) {
        return Ok(ApiResponse::error(400, &msg));
    }

    Ok(match insert_server(&state, &payload).await {
        Ok(server) => ApiResponse::success(server),
        Err(e) => ApiResponse::error(500, &e.to_string()),
    })
}

async fn insert_server(pool: &SqlitePool, payload: &ServerPayload) -> Result<Server, sqlx::Error> {
    let mut tx = pool.begin().await?;

    let result = sqlx::query(
        "INSERT INTO servers (name, addr, port, token, protocol, tls_enable, remark, sort, pinned, enabled) \
         VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
    )
    .bind(payload.name.trim())
    .bind(payload.addr.trim())
    .bind(payload.port)
    .bind(clean(&payload.token))
    .bind(normalize_protocol(&payload.protocol))
    .bind(payload.tls_enable)
    .bind(clean(&payload.remark))
    .bind(payload.sort)
    .bind(payload.pinned)
    .bind(payload.enabled)
    .execute(&mut *tx)
    .await?;

    let id = result.last_insert_rowid();

    // 若新增时启用了该服务器，则关闭其它服务器的启用状态
    if payload.enabled {
        disable_others(&mut *tx, id).await?;
    }

    tx.commit().await?;
    fetch_by_id(pool, id).await
}

/// 编辑服务器
#[tauri::command]
#[auto_collect_command]
pub async fn update_server(
    state: State<'_, SqlitePool>,
    payload: ServerPayload,
) -> Result<ApiResponse<Server>, String> {
    let Some(id) = payload.id else {
        return Ok(ApiResponse::error(400, "缺少服务器 ID"));
    };
    if let Some(msg) = validate(&payload) {
        return Ok(ApiResponse::error(400, &msg));
    }

    Ok(match update_server_row(&state, id, &payload).await {
        Ok(Some(server)) => ApiResponse::success(server),
        Ok(None) => ApiResponse::error(404, "服务器不存在"),
        Err(e) => ApiResponse::error(500, &e.to_string()),
    })
}

async fn update_server_row(
    pool: &SqlitePool,
    id: i64,
    payload: &ServerPayload,
) -> Result<Option<Server>, sqlx::Error> {
    let mut tx = pool.begin().await?;

    let result = sqlx::query(
        "UPDATE servers SET name = ?, addr = ?, port = ?, token = ?, protocol = ?, tls_enable = ?, \
         remark = ?, sort = ?, pinned = ?, enabled = ?, updated_at = datetime('now', 'localtime') \
         WHERE id = ?",
    )
    .bind(payload.name.trim())
    .bind(payload.addr.trim())
    .bind(payload.port)
    .bind(clean(&payload.token))
    .bind(normalize_protocol(&payload.protocol))
    .bind(payload.tls_enable)
    .bind(clean(&payload.remark))
    .bind(payload.sort)
    .bind(payload.pinned)
    .bind(payload.enabled)
    .bind(id)
    .execute(&mut *tx)
    .await?;

    if result.rows_affected() == 0 {
        tx.rollback().await?;
        return Ok(None);
    }

    // 若编辑时启用了该服务器，则关闭其它服务器的启用状态
    if payload.enabled {
        disable_others(&mut *tx, id).await?;
    }

    tx.commit().await?;
    Ok(Some(fetch_by_id(pool, id).await?))
}

/// 删除服务器
#[tauri::command]
#[auto_collect_command]
pub async fn delete_server(
    state: State<'_, SqlitePool>,
    id: i64,
) -> Result<ApiResponse<()>, String> {
    Ok(
        match sqlx::query("DELETE FROM servers WHERE id = ?")
            .bind(id)
            .execute(&*state)
            .await
        {
            Ok(result) if result.rows_affected() == 0 => ApiResponse::error(404, "服务器不存在"),
            Ok(_) => ApiResponse::success_empty(),
            Err(e) => ApiResponse::error(500, &e.to_string()),
        },
    )
}

/// 启用 / 禁用服务器（启用时自动关闭其它服务器）
#[tauri::command]
#[auto_collect_command]
pub async fn set_server_enabled(
    state: State<'_, SqlitePool>,
    id: i64,
    enabled: bool,
) -> Result<ApiResponse<Server>, String> {
    Ok(match toggle_enabled(&state, id, enabled).await {
        Ok(Some(server)) => ApiResponse::success(server),
        Ok(None) => ApiResponse::error(404, "服务器不存在"),
        Err(e) => ApiResponse::error(500, &e.to_string()),
    })
}

async fn toggle_enabled(
    pool: &SqlitePool,
    id: i64,
    enabled: bool,
) -> Result<Option<Server>, sqlx::Error> {
    let mut tx = pool.begin().await?;

    let result = sqlx::query(
        "UPDATE servers SET enabled = ?, updated_at = datetime('now', 'localtime') WHERE id = ?",
    )
    .bind(enabled)
    .bind(id)
    .execute(&mut *tx)
    .await?;

    if result.rows_affected() == 0 {
        tx.rollback().await?;
        return Ok(None);
    }

    if enabled {
        disable_others(&mut *tx, id).await?;
    }

    tx.commit().await?;
    Ok(Some(fetch_by_id(pool, id).await?))
}

/// 置顶 / 取消置顶服务器
#[tauri::command]
#[auto_collect_command]
pub async fn set_server_pinned(
    state: State<'_, SqlitePool>,
    id: i64,
    pinned: bool,
) -> Result<ApiResponse<Server>, String> {
    Ok(
        match sqlx::query(
            "UPDATE servers SET pinned = ?, updated_at = datetime('now', 'localtime') WHERE id = ?",
        )
        .bind(pinned)
        .bind(id)
        .execute(&*state)
        .await
        {
            Ok(result) if result.rows_affected() == 0 => ApiResponse::error(404, "服务器不存在"),
            Ok(_) => match fetch_by_id(&state, id).await {
                Ok(server) => ApiResponse::success(server),
                Err(e) => ApiResponse::error(500, &e.to_string()),
            },
            Err(e) => ApiResponse::error(500, &e.to_string()),
        },
    )
}
