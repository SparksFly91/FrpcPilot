mod commands;
mod models;
mod services;

use tauri::{Manager, RunEvent};
use tauri_plugin_autostart::ManagerExt as _;

use crate::commands::background::{choose_custom_background, get_custom_background};
use crate::commands::frp::{
    delete_frp, download_frp, get_frp_downloads, get_frp_releases, import_frp,
};
use crate::commands::runner::{get_frpc_status, start_frpc, stop_frpc};
use crate::commands::server::{
    create_server, delete_server, get_server_list, set_server_enabled, set_server_pinned,
    update_server,
};
use crate::commands::storage::{PreferencesLock, get_preferences, set_preferences};
use crate::commands::system::{get_system_details, get_system_info, is_autostart_supported};
use crate::commands::tunnel::{
    create_tunnel, delete_tunnel, get_tunnel_list, set_tunnel_enabled, set_tunnel_pinned,
    update_tunnel,
};
use crate::services::runner::FrpcRunner;
use crate::services::storage::{self, StoragePaths};
use crate::services::system::SystemStats;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let paths = StoragePaths::discover().expect("无法确定软件安装目录");
    // 此入口在启动 Tauri、Tokio 或任何工作线程之前调用。
    // 将更新、解压及子进程继承的临时文件目录统一指向安装目录。
    if std::fs::create_dir_all(paths.root.join("tmp")).is_ok() {
        unsafe {
            std::env::set_var("TEMP", paths.root.join("tmp"));
            std::env::set_var("TMP", paths.root.join("tmp"));
            std::env::set_var("TMPDIR", paths.root.join("tmp"));
        }
    }
    let mut context = tauri::generate_context!();
    context.config_mut().app.app_directories_override = Some(paths.overrides());
    // 在打开 WebView 前初始化数据目录，同时保留各平台的窗口配置。
    for window in &mut context.config_mut().app.windows {
        window.create = false;
    }
    let mut builder = tauri::Builder::default()
        .manage(paths)
        .manage(PreferencesLock::default())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init()) // 对话框插件
        .plugin(tauri_plugin_updater::Builder::new().build()) // 更新插件
        .plugin(tauri_plugin_process::init()) // 进程插件（更新后重启）
        .plugin(tauri_plugin_prevent_default::Builder::new().build());

    // 开发版依赖 Vite，调试构建在 Windows 上还会打开控制台窗口。
    if is_autostart_supported() {
        builder = builder.plugin(tauri_plugin_autostart::init(
            tauri_plugin_autostart::MacosLauncher::LaunchAgent,
            None,
        ));
    }

    let app = builder
        .setup(|app| {
            use tauri_plugin_dialog::{DialogExt, MessageDialogKind};

            let paths = app.state::<StoragePaths>();
            let initialize = || -> Result<_, Box<dyn std::error::Error>> {
                let (pool, lock) = tauri::async_runtime::block_on(storage::initialize(&paths))
                    .map_err(std::io::Error::other)?;
                app.manage(pool);
                app.manage(lock);
                Ok(())
            };
            if let Err(error) = initialize() {
                app.dialog().message(format!(
                    "无法初始化安装目录中的数据：{}\n\n{error}\n\n请确认安装目录可写、磁盘空间充足，并退出使用同一数据目录的其他实例。",
                    paths.root.display()
                )).title("FrpcPilot 数据初始化失败").kind(MessageDialogKind::Error).blocking_show();
                return Err(error);
            }

            // 插件会动态解析当前可执行文件的位置；重新安装或移动应用后刷新
            // 已启用的自启动项，原本禁用的自启动项仍保持禁用。
            if is_autostart_supported() {
                let autostart = app.autolaunch();
                let result = autostart.is_enabled().and_then(|enabled| {
                    if enabled {
                        autostart.enable()
                    } else {
                        Ok(())
                    }
                });
                if let Err(error) = result {
                    eprintln!("Failed to refresh autostart executable path: {error}");
                }
            }

            // 系统资源占用（CPU / 内存）后台采集
            let stats = SystemStats::default();
            stats.start();
            app.manage(stats);

            // Frpc 进程管理器（一键启动 / 停止与日志实时回显）
            app.manage(FrpcRunner::default());

            for config in &app.config().app.windows {
                let window = tauri::WebviewWindowBuilder::from_config(app.handle(), config)?;
                #[cfg(not(target_os = "macos"))]
                let window = window.data_directory(paths.webview());
                window.build()?;
            }

            Ok(())
        })
        .plugin(tauri_plugin_store::Builder::default().build())
        // .invoke_handler(tauri::generate_handler![greet])
        .invoke_handler(tauri_helper::tauri_collect_commands!())
        .build(context)
        .expect("error while building tauri application");

    app.run(|app_handle, event| {
        // 应用退出时结束 frpc 子进程，避免残留后台进程
        if let RunEvent::Exit = event {
            app_handle.state::<FrpcRunner>().shutdown();
        }
    });
}
