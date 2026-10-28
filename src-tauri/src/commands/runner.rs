//! 一键启动 / 停止 Frpc 相关命令。
//!
//! 启动流程：读取所选 frpc 版本 → 汇总已启用的服务器与隧道 → 生成 frpc.toml
//! → 交给 `FrpcRunner` 以独立进程方式拉起，并实时回传日志。

use std::path::PathBuf;

use chrono::Local;
use sqlx::SqlitePool;
use tauri::{AppHandle, Manager, State};
use tauri_helper::auto_collect_command;

use crate::models::{ApiResponse, FrpDownload, FrpcStatus, Server, Tunnel};
use crate::services::config::generate_frpc_config;
use crate::services::runner::FrpcRunner;
use crate::services::storage::StoragePaths;

// 统一查询字段，使用 concat! 保证结果为 'static str（sqlx 0.9 要求）
macro_rules! server_fields {
    () => {
        "id, name, addr, port, token, protocol, tls_enable, remark, sort, pinned, enabled, created_at, updated_at"
    };
}

macro_rules! tunnel_fields {
    () => {
        "id, name, proxy_type, local_ip, local_port, remote_port, custom_domains, subdomain, locations, secret_key, use_encryption, use_compression, bandwidth_limit, proxy_protocol_version, health_check_type, health_check_timeout, health_check_max_failed, health_check_interval, health_check_path, load_balancer_group, load_balancer_group_key, remark, sort, pinned, enabled, created_at, updated_at"
    };
}

macro_rules! download_fields {
    () => {
        "id, version, os, arch, platform, file_name, archive_path, frpc_path, frps_path, file_size, source, created_at, updated_at"
    };
}

/// 本次运行生成的 frpc.toml 路径（安装目录/run/frpc.toml）
fn config_file_path(app: &AppHandle) -> Result<PathBuf, String> {
    let dir = app.state::<StoragePaths>().run();
    std::fs::create_dir_all(&dir).map_err(|e| format!("创建配置目录失败：{e}"))?;
    Ok(dir.join("frpc.toml"))
}

/// 获取 Frpc 当前运行状态
#[tauri::command]
#[auto_collect_command]
pub async fn get_frpc_status(
    runner: State<'_, FrpcRunner>,
) -> Result<ApiResponse<FrpcStatus>, String> {
    Ok(ApiResponse::success(runner.status()))
}

/// 一键启动 Frpc
#[tauri::command]
#[auto_collect_command]
pub async fn start_frpc(
    app: AppHandle,
    pool: State<'_, SqlitePool>,
    runner: State<'_, FrpcRunner>,
    download_id: i64,
) -> Result<ApiResponse<FrpcStatus>, String> {
    if runner.is_running() {
        return Ok(ApiResponse::error(409, "Frpc 已在运行中，请先停止后再启动"));
    }

    // 1. 读取所选 Frpc 版本记录
    let download = match sqlx::query_as::<_, FrpDownload>(concat!(
        "SELECT ",
        download_fields!(),
        " FROM frp_downloads WHERE id = ?"
    ))
    .bind(download_id)
    .fetch_optional(&*pool)
    .await
    {
        Ok(Some(row)) => row,
        Ok(None) => {
            return Ok(ApiResponse::error(
                404,
                "所选 Frpc 版本不存在，请刷新后重试",
            ));
        }
        Err(e) => return Ok(ApiResponse::error(500, &e.to_string())),
    };

    let Some(frpc_path) = download
        .frpc_path
        .clone()
        .filter(|path| !path.trim().is_empty())
    else {
        return Ok(ApiResponse::error(
            400,
            "所选版本缺少 frpc 可执行文件，请重新下载或导入",
        ));
    };
    if !std::path::Path::new(&frpc_path).is_file() {
        return Ok(ApiResponse::error(
            400,
            &format!("frpc 可执行文件不存在：{frpc_path}"),
        ));
    }

    // 2. 已启用的服务器（全局同时只启用一个）
    let server = match sqlx::query_as::<_, Server>(concat!(
        "SELECT ",
        server_fields!(),
        " FROM servers WHERE enabled = 1 ORDER BY pinned DESC, sort ASC, id ASC LIMIT 1"
    ))
    .fetch_optional(&*pool)
    .await
    {
        Ok(Some(row)) => row,
        Ok(None) => {
            return Ok(ApiResponse::error(
                400,
                "尚未启用任何服务器，请先在服务器页面启用一个服务器",
            ));
        }
        Err(e) => return Ok(ApiResponse::error(500, &e.to_string())),
    };

    // 3. 已启用的隧道（可同时启用多个）
    let tunnels = match sqlx::query_as::<_, Tunnel>(concat!(
        "SELECT ",
        tunnel_fields!(),
        " FROM tunnels WHERE enabled = 1 ORDER BY pinned DESC, sort ASC, id ASC"
    ))
    .fetch_all(&*pool)
    .await
    {
        Ok(list) => list,
        Err(e) => return Ok(ApiResponse::error(500, &e.to_string())),
    };
    if tunnels.is_empty() {
        return Ok(ApiResponse::error(
            400,
            "尚未启用任何隧道，请先在隧道列表页面启用至少一个隧道",
        ));
    }

    // 4. 生成 frpc.toml
    let config = generate_frpc_config(&server, &tunnels);
    let config_path = match config_file_path(&app) {
        Ok(path) => path,
        Err(e) => return Ok(ApiResponse::error(500, &e)),
    };
    if let Err(e) = std::fs::write(&config_path, config) {
        return Ok(ApiResponse::error(
            500,
            &format!("写入 frpc.toml 失败：{e}"),
        ));
    }

    // 5. 启动 frpc 进程
    let config_path_str = config_path.to_string_lossy().to_string();
    let paths = app.state::<StoragePaths>();
    let log_path = paths
        .logs()
        .join(format!("frpc-{}.log", Local::now().format("%Y-%m-%d")));
    match runner.spawn(
        &app,
        &frpc_path,
        &config_path_str,
        &download.version,
        &log_path,
    ) {
        Ok(status) => Ok(ApiResponse::success(status)),
        Err(e) => Ok(ApiResponse::error(500, &e)),
    }
}

/// 停止 Frpc
#[tauri::command]
#[auto_collect_command]
pub async fn stop_frpc(
    app: AppHandle,
    runner: State<'_, FrpcRunner>,
) -> Result<ApiResponse<FrpcStatus>, String> {
    match runner.stop(&app) {
        Ok(status) => Ok(ApiResponse::success(status)),
        Err(e) => Ok(ApiResponse::error(500, &e)),
    }
}
