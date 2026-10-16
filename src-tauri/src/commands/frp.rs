use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::time::Duration;

use futures_util::StreamExt;
use serde_json::Value;
use sqlx::SqlitePool;
use tauri::{AppHandle, Emitter, Manager, State};
use tauri_helper::auto_collect_command;
use tokio::io::AsyncWriteExt;

use crate::models::{
    ApiResponse, FrpDownload, FrpDownloadPayload, FrpDownloadProgress, FrpRelease,
};

/// GitHub 镜像加速前缀（国内访问 GitHub 不稳定，拉取版本与下载统一走镜像）
const GITHUB_MIRROR: &str = "https://gh-proxy.com/";
/// frp 仓库 releases API
const RELEASES_API: &str = "https://api.github.com/repos/fatedier/frp/releases?per_page=30";
/// 下载进度事件名
const PROGRESS_EVENT: &str = "frp-download-progress";

/// frp_downloads 表统一查询字段
macro_rules! frp_fields {
    () => {
        "id, version, os, arch, platform, file_name, archive_path, frpc_path, frps_path, file_size, source, created_at, updated_at"
    };
}

fn http_client() -> Result<reqwest::Client, reqwest::Error> {
    reqwest::Client::builder()
        .user_agent("FrpcPilot/0.1 (+https://github.com/fatedier/frp)")
        .connect_timeout(Duration::from_secs(15))
        .build()
}

/// 当前运行平台（os + arch），并映射为 frp 发行包的命名
fn current_platform() -> (String, String) {
    let os = match std::env::consts::OS {
        "macos" => "darwin",
        other => other,
    }
    .to_string();
    let arch = match std::env::consts::ARCH {
        "x86_64" => "amd64",
        "aarch64" => "arm64",
        "x86" => "386",
        other => other,
    }
    .to_string();
    (os, arch)
}

/// 解析发行包文件名，返回 (版本, os, arch)
///
/// 形如 `frp_0.71.0_windows_amd64.zip` / `frp_0.71.0_linux_arm_hf.tar.gz`，
/// arch 部分可能包含下划线（如 arm_hf、mips_softfloat），故剩余部分整体拼接。
fn parse_asset(name: &str) -> Option<(String, String, String)> {
    let rest = name.strip_prefix("frp_")?;
    let rest = rest
        .strip_suffix(".tar.gz")
        .or_else(|| rest.strip_suffix(".zip"))?;
    let mut parts = rest.split('_');
    let version = parts.next()?.to_string();
    let os = parts.next()?.to_string();
    let arch = parts.collect::<Vec<_>>().join("_");
    if version.is_empty() || os.is_empty() || arch.is_empty() {
        return None;
    }
    Some((version, os, arch))
}

/// 从文件名中解析版本号（形如 0.71.0），返回带 v 前缀的版本号
fn parse_version(name: &str) -> Option<String> {
    for raw in name.split(|c: char| !(c.is_ascii_digit() || c == '.' || c == 'v' || c == 'V')) {
        let token = raw.trim_start_matches(|c| c == 'v' || c == 'V');
        let parts: Vec<&str> = token.split('.').collect();
        let valid = parts.len() >= 3
            && parts[..3]
                .iter()
                .all(|p| !p.is_empty() && p.chars().all(|c| c.is_ascii_digit()));
        if valid {
            return Some(format!("v{}.{}.{}", parts[0], parts[1], parts[2]));
        }
    }
    None
}

async fn fetch_locals(pool: &SqlitePool) -> Result<Vec<FrpDownload>, sqlx::Error> {
    sqlx::query_as::<_, FrpDownload>(concat!(
        "SELECT ",
        frp_fields!(),
        " FROM frp_downloads ORDER BY id ASC"
    ))
    .fetch_all(pool)
    .await
}

async fn fetch_local(
    pool: &SqlitePool,
    version: &str,
    platform: &str,
) -> Result<FrpDownload, sqlx::Error> {
    sqlx::query_as::<_, FrpDownload>(concat!(
        "SELECT ",
        frp_fields!(),
        " FROM frp_downloads WHERE version = ? AND platform = ?"
    ))
    .bind(version)
    .bind(platform)
    .fetch_one(pool)
    .await
}

/// 拉取 frp releases（镜像优先，失败回退直连）
async fn fetch_releases(client: &reqwest::Client) -> Result<Vec<Value>, String> {
    let urls = [
        format!("{GITHUB_MIRROR}{RELEASES_API}"),
        RELEASES_API.to_string(),
    ];
    let mut last_err = String::new();
    for url in urls {
        match client.get(&url).send().await {
            Ok(resp) if resp.status().is_success() => match resp.json::<Value>().await {
                Ok(Value::Array(list)) => return Ok(list),
                Ok(_) => last_err = "返回数据格式异常".to_string(),
                Err(e) => last_err = e.to_string(),
            },
            Ok(resp) => last_err = format!("HTTP {}", resp.status()),
            Err(e) => last_err = e.to_string(),
        }
    }
    Err(format!("获取 Frp 版本列表失败：{last_err}"))
}

/// 应用数据目录下的 frp 版本存储根目录
fn versions_dir(app: &AppHandle) -> Result<PathBuf, String> {
    Ok(app
        .path()
        .app_data_dir()
        .map_err(|e| e.to_string())?
        .join("frp")
        .join("versions"))
}

/// 发送下载进度事件
fn emit_progress(
    app: &AppHandle,
    payload: &FrpDownloadPayload,
    downloaded: u64,
    total: u64,
    status: &str,
    message: Option<&str>,
) {
    let percent = if total > 0 {
        ((downloaded as f64 / total as f64) * 100.0).clamp(0.0, 100.0).round() as u8
    } else {
        0
    };
    let _ = app.emit(
        PROGRESS_EVENT,
        FrpDownloadProgress {
            key: payload.file_name.clone(),
            version: payload.version.clone(),
            platform: payload.platform.clone(),
            downloaded,
            total,
            percent,
            status: status.to_string(),
            message: message.map(|m| m.to_string()),
        },
    );
}

/// 获取 Frp 版本列表（返回所有架构的发行包，并合并本机已下载 / 已导入记录）
///
/// 前端根据「展示所有架构的 Frpc / Frps」开关在本地过滤，避免频繁请求
/// GitHub API 触发限流；默认仅展示当前运行平台由前端完成。
#[tauri::command]
#[auto_collect_command]
pub async fn get_frp_releases(
    state: State<'_, SqlitePool>,
) -> Result<ApiResponse<Vec<FrpRelease>>, String> {
    let client = match http_client() {
        Ok(c) => c,
        Err(e) => return Ok(ApiResponse::error(500, &e.to_string())),
    };
    let releases = match fetch_releases(&client).await {
        Ok(list) => list,
        Err(e) => return Ok(ApiResponse::error(500, &e)),
    };
    let locals = match fetch_locals(&state).await {
        Ok(list) => list,
        Err(e) => return Ok(ApiResponse::error(500, &e.to_string())),
    };

    let (cur_os, cur_arch) = current_platform();
    let cur_platform = format!("{cur_os}_{cur_arch}");

    let mut list: Vec<FrpRelease> = Vec::new();
    let mut matched: HashSet<String> = HashSet::new();

    for rel in &releases {
        let version = rel
            .get("tag_name")
            .and_then(Value::as_str)
            .unwrap_or_default()
            .to_string();
        if version.is_empty() {
            continue;
        }
        let published_at = rel
            .get("published_at")
            .and_then(Value::as_str)
            .unwrap_or_default()
            .to_string();
        let Some(assets) = rel.get("assets").and_then(Value::as_array) else {
            continue;
        };

        for asset in assets {
            let name = asset.get("name").and_then(Value::as_str).unwrap_or_default();
            let Some((_v, os, arch)) = parse_asset(name) else {
                continue;
            };
            let platform = format!("{os}_{arch}");
            let matches = os == cur_os && arch == cur_arch;

            let size = asset.get("size").and_then(Value::as_i64).unwrap_or(0);
            let download_count = asset
                .get("download_count")
                .and_then(Value::as_i64)
                .unwrap_or(0);
            let url = asset
                .get("browser_download_url")
                .and_then(Value::as_str)
                .unwrap_or_default()
                .to_string();

            let local = locals
                .iter()
                .find(|l| l.version == version && l.platform == platform);
            if let Some(l) = local {
                matched.insert(format!("{}|{}", l.version, l.platform));
            }

            list.push(FrpRelease {
                version: version.clone(),
                os,
                arch,
                platform,
                file_name: name.to_string(),
                size,
                download_count,
                published_at: published_at.clone(),
                mirror_url: format!("{GITHUB_MIRROR}{url}"),
                download_url: url,
                downloaded: local.is_some(),
                is_current: matches,
                local_only: false,
                local_id: local.map(|l| l.id),
                frpc_path: local.and_then(|l| l.frpc_path.clone()),
                frps_path: local.and_then(|l| l.frps_path.clone()),
            });
        }
    }

    // 本机已下载 / 导入但未匹配到发行包的记录，单独作为本地卡片展示
    for l in &locals {
        if matched.contains(&format!("{}|{}", l.version, l.platform)) {
            continue;
        }
        list.push(FrpRelease {
            version: l.version.clone(),
            os: l.os.clone(),
            arch: l.arch.clone(),
            platform: l.platform.clone(),
            file_name: l.file_name.clone(),
            size: l.file_size,
            download_count: 0,
            published_at: String::new(),
            download_url: String::new(),
            mirror_url: String::new(),
            downloaded: true,
            is_current: l.platform == cur_platform,
            local_only: true,
            local_id: Some(l.id),
            frpc_path: l.frpc_path.clone(),
            frps_path: l.frps_path.clone(),
        });
    }

    // 当前平台优先，其次版本新到旧
    list.sort_by(|a, b| {
        let ap = a.platform == cur_platform;
        let bp = b.platform == cur_platform;
        bp.cmp(&ap)
            .then_with(|| b.published_at.cmp(&a.published_at))
            .then_with(|| b.version.cmp(&a.version))
    });

    Ok(ApiResponse::success(list))
}

/// 获取本机已下载 / 已导入的 Frp 版本列表（仪表盘选择 frpc 版本用）
#[tauri::command]
#[auto_collect_command]
pub async fn get_frp_downloads(
    state: State<'_, SqlitePool>,
) -> Result<ApiResponse<Vec<FrpDownload>>, String> {
    let list = sqlx::query_as::<_, FrpDownload>(concat!(
        "SELECT ",
        frp_fields!(),
        " FROM frp_downloads ORDER BY created_at DESC, id DESC"
    ))
    .fetch_all(&*state)
    .await;

    Ok(match list {
        Ok(list) => ApiResponse::success(list),
        Err(e) => ApiResponse::error(500, &e.to_string()),
    })
}

/// 下载并解压指定 Frp 发行包
#[tauri::command]
#[auto_collect_command]
pub async fn download_frp(
    app: AppHandle,
    state: State<'_, SqlitePool>,
    payload: FrpDownloadPayload,
) -> Result<ApiResponse<FrpDownload>, String> {
    match do_download(&app, &state, &payload).await {
        Ok(row) => Ok(ApiResponse::success(row)),
        Err(e) => {
            emit_progress(&app, &payload, 0, 0, "error", Some(&e));
            Ok(ApiResponse::error(500, &e))
        }
    }
}

async fn do_download(
    app: &AppHandle,
    pool: &SqlitePool,
    payload: &FrpDownloadPayload,
) -> Result<FrpDownload, String> {
    let client = http_client().map_err(|e| e.to_string())?;

    let ver_dir = versions_dir(app)?.join(&payload.version);
    let extract_dir = ver_dir.join(&payload.platform);
    tokio::fs::create_dir_all(&extract_dir)
        .await
        .map_err(|e| e.to_string())?;
    let archive_path = ver_dir.join(&payload.file_name);

    // 压缩包不存在（或为空）时才重新下载
    let need_download = match tokio::fs::metadata(&archive_path).await {
        Ok(meta) => meta.len() == 0,
        Err(_) => true,
    };
    if need_download {
        download_file(app, &client, payload, &archive_path).await?;
    }

    // 已解压过则跳过解压
    if find_binary(&extract_dir, "frpc").is_none() && find_binary(&extract_dir, "frps").is_none() {
        emit_progress(app, payload, 0, 0, "extracting", None);
        extract_archive(&archive_path, &extract_dir).await?;
    }

    let frpc = find_binary(&extract_dir, "frpc");
    let frps = find_binary(&extract_dir, "frps");
    if frpc.is_none() && frps.is_none() {
        return Err("解压完成，但未找到 frpc / frps 可执行文件".to_string());
    }
    if let Some(path) = &frpc {
        make_executable(path);
    }
    if let Some(path) = &frps {
        make_executable(path);
    }

    let size = tokio::fs::metadata(&archive_path)
        .await
        .map(|m| m.len() as i64)
        .unwrap_or(0);

    sqlx::query(
        "INSERT INTO frp_downloads (version, os, arch, platform, file_name, archive_path, frpc_path, frps_path, file_size, source) \
         VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, 'download') \
         ON CONFLICT(version, platform) DO UPDATE SET \
            file_name = excluded.file_name, archive_path = excluded.archive_path, \
            frpc_path = excluded.frpc_path, frps_path = excluded.frps_path, \
            file_size = excluded.file_size, source = 'download', \
            updated_at = datetime('now', 'localtime')",
    )
    .bind(&payload.version)
    .bind(&payload.os)
    .bind(&payload.arch)
    .bind(&payload.platform)
    .bind(&payload.file_name)
    .bind(archive_path.to_string_lossy().to_string())
    .bind(frpc.as_ref().map(|p| p.to_string_lossy().to_string()))
    .bind(frps.as_ref().map(|p| p.to_string_lossy().to_string()))
    .bind(size)
    .execute(pool)
    .await
    .map_err(|e| e.to_string())?;

    emit_progress(app, payload, 1, 1, "done", None);
    fetch_local(pool, &payload.version, &payload.platform)
        .await
        .map_err(|e| e.to_string())
}

/// 下载压缩包（镜像优先，失败回退直连），并持续上报进度
async fn download_file(
    app: &AppHandle,
    client: &reqwest::Client,
    payload: &FrpDownloadPayload,
    archive_path: &Path,
) -> Result<(), String> {
    let urls = [
        format!("{GITHUB_MIRROR}{}", payload.download_url),
        payload.download_url.clone(),
    ];
    let mut last_err = String::new();

    for url in urls {
        match client.get(&url).send().await {
            Ok(resp) if resp.status().is_success() => {
                let total = resp.content_length().unwrap_or(0);
                let mut file = tokio::fs::File::create(archive_path)
                    .await
                    .map_err(|e| e.to_string())?;
                let mut downloaded: u64 = 0;
                let mut stream = resp.bytes_stream();
                let mut stream_err: Option<String> = None;
                emit_progress(app, payload, 0, total, "downloading", None);

                while let Some(chunk) = stream.next().await {
                    match chunk {
                        Ok(bytes) => {
                            file.write_all(&bytes).await.map_err(|e| e.to_string())?;
                            downloaded += bytes.len() as u64;
                            emit_progress(app, payload, downloaded, total, "downloading", None);
                        }
                        Err(e) => {
                            stream_err = Some(e.to_string());
                            break;
                        }
                    }
                }

                if let Some(err) = stream_err {
                    // 本次下载中断，清理残缺文件后尝试下一个源
                    let _ = file.shutdown().await;
                    let _ = tokio::fs::remove_file(archive_path).await;
                    last_err = err;
                    continue;
                }

                let _ = file.flush().await;
                return Ok(());
            }
            Ok(resp) => last_err = format!("HTTP {}", resp.status()),
            Err(e) => last_err = e.to_string(),
        }
    }

    Err(format!("下载失败：{last_err}"))
}

/// 解压发行包。Windows 为 zip、类 Unix 为 tar.gz，统一使用系统 tar 命令
/// （Windows 10+ 内置 bsdtar，可同时处理 zip 与 tar.gz）
async fn extract_archive(archive: &Path, dest: &Path) -> Result<(), String> {
    let archive = archive.to_path_buf();
    let dest = dest.to_path_buf();
    tokio::task::spawn_blocking(move || {
        let output = std::process::Command::new("tar")
            .arg("-xf")
            .arg(&archive)
            .arg("-C")
            .arg(&dest)
            .output()
            .map_err(|e| format!("调用 tar 解压失败：{e}"))?;
        if !output.status.success() {
            return Err(format!(
                "解压失败：{}",
                String::from_utf8_lossy(&output.stderr).trim()
            ));
        }
        Ok(())
    })
    .await
    .map_err(|e| e.to_string())?
}

/// 在目录中递归查找 frpc / frps 可执行文件
fn find_binary(dir: &Path, stem: &str) -> Option<PathBuf> {
    let stem_lower = stem.to_ascii_lowercase();
    let mut stack = vec![dir.to_path_buf()];
    while let Some(current) = stack.pop() {
        let Ok(entries) = std::fs::read_dir(&current) else {
            continue;
        };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                stack.push(path);
                continue;
            }
            let Some(name) = path.file_name().map(|s| s.to_string_lossy().to_ascii_lowercase())
            else {
                continue;
            };
            if name == stem_lower || name == format!("{stem_lower}.exe") {
                return Some(path);
            }
        }
    }
    None
}

#[cfg(unix)]
fn make_executable(path: &Path) {
    use std::os::unix::fs::PermissionsExt;
    if let Ok(meta) = std::fs::metadata(path) {
        let mut perms = meta.permissions();
        perms.set_mode(0o755);
        let _ = std::fs::set_permissions(path, perms);
    }
}

#[cfg(not(unix))]
fn make_executable(_path: &Path) {}

/// 删除已下载 / 已导入的版本（同时清理本地文件）
#[tauri::command]
#[auto_collect_command]
pub async fn delete_frp(
    app: AppHandle,
    state: State<'_, SqlitePool>,
    id: i64,
) -> Result<ApiResponse<()>, String> {
    let row = match sqlx::query_as::<_, FrpDownload>(concat!(
        "SELECT ",
        frp_fields!(),
        " FROM frp_downloads WHERE id = ?"
    ))
    .bind(id)
    .fetch_optional(&*state)
    .await
    {
        Ok(Some(row)) => row,
        Ok(None) => return Ok(ApiResponse::error(404, "记录不存在")),
        Err(e) => return Ok(ApiResponse::error(500, &e.to_string())),
    };

    if let Ok(base) = versions_dir(&app) {
        let ver_dir = base.join(&row.version);
        let _ = tokio::fs::remove_dir_all(ver_dir.join(&row.platform)).await;
        if let Some(archive) = &row.archive_path {
            let _ = tokio::fs::remove_file(archive).await;
        }
        // 版本目录已空则一并删除
        if let Ok(mut entries) = tokio::fs::read_dir(&ver_dir).await {
            if matches!(entries.next_entry().await, Ok(None)) {
                let _ = tokio::fs::remove_dir(&ver_dir).await;
            }
        }
    }

    match sqlx::query("DELETE FROM frp_downloads WHERE id = ?")
        .bind(id)
        .execute(&*state)
        .await
    {
        Ok(_) => Ok(ApiResponse::success_empty()),
        Err(e) => Ok(ApiResponse::error(500, &e.to_string())),
    }
}

/// 导入本地已下载的 frpc / frps（记录可执行文件路径，不复制文件）
#[tauri::command]
#[auto_collect_command]
pub async fn import_frp(
    app: AppHandle,
    state: State<'_, SqlitePool>,
) -> Result<ApiResponse<Vec<FrpDownload>>, String> {
    use tauri_plugin_dialog::DialogExt;

    let picked = app
        .dialog()
        .file()
        .set_title("选择本地已下载的 frpc / frps")
        .blocking_pick_files();
    let Some(files) = picked else {
        return Ok(ApiResponse::success(Vec::new()));
    };

    let (os, arch) = current_platform();
    let platform = format!("{os}_{arch}");
    let mut result = Vec::new();

    for file in files {
        let path = match file.into_path() {
            Ok(path) => path,
            Err(e) => return Ok(ApiResponse::error(500, &e.to_string())),
        };
        let file_name = path
            .file_name()
            .map(|s| s.to_string_lossy().to_string())
            .unwrap_or_default();
        let size = std::fs::metadata(&path).map(|m| m.len() as i64).unwrap_or(0);
        let lower = file_name.to_ascii_lowercase();
        let is_frps = lower.contains("frps");
        let version = parse_version(&file_name).unwrap_or_else(|| file_name.clone());
        let full = path.to_string_lossy().to_string();
        let (frpc_path, frps_path) = if is_frps {
            (None, Some(full))
        } else {
            (Some(full), None)
        };

        let exec = sqlx::query(
            "INSERT INTO frp_downloads (version, os, arch, platform, file_name, frpc_path, frps_path, file_size, source) \
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, 'import') \
             ON CONFLICT(version, platform) DO UPDATE SET \
                frpc_path = COALESCE(excluded.frpc_path, frp_downloads.frpc_path), \
                frps_path = COALESCE(excluded.frps_path, frp_downloads.frps_path), \
                file_size = CASE WHEN excluded.file_size > 0 THEN excluded.file_size ELSE frp_downloads.file_size END, \
                source = 'import', updated_at = datetime('now', 'localtime')",
        )
        .bind(&version)
        .bind(&os)
        .bind(&arch)
        .bind(&platform)
        .bind(&file_name)
        .bind(frpc_path)
        .bind(frps_path)
        .bind(size)
        .execute(&*state)
        .await;

        match exec {
            Ok(_) => {
                if let Ok(row) = fetch_local(&state, &version, &platform).await {
                    result.push(row);
                }
            }
            Err(e) => return Ok(ApiResponse::error(500, &e.to_string())),
        }
    }

    Ok(ApiResponse::success(result))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_asset_handles_common_names() {
        assert_eq!(
            parse_asset("frp_0.71.0_windows_amd64.zip"),
            Some(("0.71.0".into(), "windows".into(), "amd64".into()))
        );
        assert_eq!(
            parse_asset("frp_0.71.0_linux_arm64.tar.gz"),
            Some(("0.71.0".into(), "linux".into(), "arm64".into()))
        );
        // arch 含下划线的情况
        assert_eq!(
            parse_asset("frp_0.71.0_linux_arm_hf.tar.gz"),
            Some(("0.71.0".into(), "linux".into(), "arm_hf".into()))
        );
        assert_eq!(parse_asset("frp_0.71.0_linux_mips_softfloat.tar.gz"), {
            Some(("0.71.0".into(), "linux".into(), "mips_softfloat".into()))
        });
        assert_eq!(parse_asset("readme.md"), None);
    }

    #[test]
    fn parse_version_extracts_semver() {
        assert_eq!(parse_version("frpc_0.71.0_windows_amd64.exe"), Some("v0.71.0".into()));
        assert_eq!(parse_version("frps-v0.60.0"), Some("v0.60.0".into()));
        assert_eq!(parse_version("frpc.exe"), None);
        assert_eq!(parse_version("frpc_0.71.exe"), None);
    }

    #[test]
    fn current_platform_is_known() {
        let (os, arch) = current_platform();
        assert!(!os.is_empty());
        assert!(!arch.is_empty());
        println!("current platform: {os}_{arch}");
    }
}
