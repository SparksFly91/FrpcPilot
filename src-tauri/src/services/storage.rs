//! 按安装位置分类存储应用数据。
//! 此模块在创建任何 WebView 或打开应用数据库连接池之前执行。

use std::fs::{self, File, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};

use serde::Serialize;
use sqlx::SqlitePool;
use tauri::utils::config::{AppDirectoriesOverride, AppDirectoryOverrides};

use super::db::init_pool;

#[derive(Clone, Debug)]
pub struct StoragePaths {
    pub root: PathBuf,
}

impl StoragePaths {
    pub fn discover() -> Result<Self, String> {
        let exe = std::env::current_exe().map_err(|e| e.to_string())?;
        // APPIMAGE 指向实际的安装文件，避免使用只读的 /tmp/.mount_* 挂载目录。
        let appimage = if cfg!(target_os = "linux") && !cfg!(debug_assertions) {
            std::env::var_os("APPIMAGE").map(PathBuf::from)
        } else {
            None
        };
        let mut root = installation_root(&exe, appimage.as_deref(), cfg!(target_os = "macos"))?;
        // 开发和测试使用独立的数据目录，避免影响正式版配置。
        if cfg!(debug_assertions) {
            root = root.join("FrpcPilot-dev");
        }
        Ok(Self { root })
    }

    pub fn data(&self) -> PathBuf {
        self.root.join("data")
    }
    pub fn db(&self) -> PathBuf {
        self.data().join("frpcpilot.db")
    }
    pub fn versions(&self) -> PathBuf {
        self.root.join("frp").join("versions")
    }
    pub fn run(&self) -> PathBuf {
        self.root.join("run")
    }
    pub fn logs(&self) -> PathBuf {
        self.root.join("logs")
    }
    pub fn webview(&self) -> PathBuf {
        self.root.join("webview")
    }
    pub fn overrides(&self) -> AppDirectoriesOverride {
        AppDirectoriesOverride::Directories(AppDirectoryOverrides {
            config: Some(self.data()),
            data: Some(self.data()),
            local_data: Some(self.webview()),
            cache: Some(self.root.join("cache")),
            log: Some(self.root.join("logs")),
        })
    }
}

fn installation_root(exe: &Path, appimage: Option<&Path>, macos: bool) -> Result<PathBuf, String> {
    if macos {
        if let Some(bundle) = exe
            .ancestors()
            .find(|p| p.extension().is_some_and(|e| e == "app"))
        {
            // 数据保存在应用包同级目录，避免 macOS 更新器替换签名应用包时丢失数据。
            return Ok(bundle.with_file_name("FrpcPilot-data"));
        }
    }
    let location = appimage.unwrap_or(exe);
    if !location.is_absolute() {
        return Err(format!("安装位置必须为绝对路径：{}", location.display()));
    }
    location
        .parent()
        .map(Path::to_path_buf)
        .ok_or_else(|| "无法确定安装目录".into())
}

/// 在应用整个生命周期内持有文件锁，进程崩溃后操作系统也会自动释放锁。
pub struct StorageLock(File);

impl StorageLock {
    fn acquire(path: &Path) -> Result<Self, String> {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(false)
            .open(path)
            .map_err(|e| {
                format!(
                    "无法写入 {}：{e}。请将软件安装到当前用户可写的目录。",
                    path.display()
                )
            })?;
        file.try_lock().map_err(|e| {
            format!(
                "数据目录正在被另一个 FrpcPilot 使用：{}（{e}）。请先退出其他实例。",
                path.display()
            )
        })?;
        Ok(Self(file))
    }
}

impl Drop for StorageLock {
    fn drop(&mut self) {
        let _ = self.0.unlock();
    }
}

pub async fn initialize(paths: &StoragePaths) -> Result<(SqlitePool, StorageLock), String> {
    fs::create_dir_all(paths.data()).map_err(|e| {
        format!(
            "无法创建数据目录 {}：{e}。请使用可写的安装目录。",
            paths.data().display()
        )
    })?;
    let lock = StorageLock::acquire(&paths.data().join("storage.lock"))?;
    for dir in [
        paths.versions(),
        paths.run(),
        paths.webview(),
        paths.root.join("cache"),
        paths.logs(),
        paths.root.join("tmp"),
    ] {
        fs::create_dir_all(&dir).map_err(|e| format!("无法创建 {}：{e}", dir.display()))?;
    }
    let pool = init_pool(&paths.db())
        .await
        .map_err(|e| format!("初始化数据库失败：{e}"))?;
    Ok((pool, lock))
}

pub fn write_json(path: &Path, value: &impl Serialize) -> Result<(), String> {
    let bytes = serde_json::to_vec_pretty(value).map_err(|e| e.to_string())?;
    let temp = path.with_extension("json.tmp");
    let mut file = File::create(&temp).map_err(|e| e.to_string())?;
    file.write_all(&bytes)
        .and_then(|_| file.sync_all())
        .map_err(|e| e.to_string())?;
    drop(file);
    // std::fs::rename 在 Windows 和 Unix 上均以原子方式替换已有文件。
    fs::rename(&temp, path).map_err(|e| e.to_string())
}

#[cfg(test)]
#[path = "storage_tests.rs"]
mod tests;
