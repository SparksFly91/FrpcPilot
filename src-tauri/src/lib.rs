mod commands;
mod models;
mod services;

use tauri::Manager;

use crate::commands::frp::{
    delete_frp, download_frp, get_frp_downloads, get_frp_releases, import_frp,
};
use crate::commands::server::{
    create_server, delete_server, get_server_list, set_server_enabled, set_server_pinned,
    update_server,
};
use crate::commands::system::{get_system_details, get_system_info};
use crate::commands::tunnel::{
    create_tunnel, delete_tunnel, get_tunnel_list, set_tunnel_enabled, set_tunnel_pinned,
    update_tunnel,
};
use crate::services::db::init_pool;
use crate::services::system::SystemStats;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init()) // 对话框插件
        .plugin(tauri_plugin_updater::Builder::new().build()) // 更新插件
        .plugin(tauri_plugin_process::init()) // 进程插件（更新后重启）
        .plugin(tauri_plugin_prevent_default::Builder::new().build())
        .setup(|app| {
            let pool = tauri::async_runtime::block_on(async {
                init_pool(app.handle())
                    .await
                    .expect("sqlite数据库连接池初始化失败!")
            });
            app.manage(pool);

            // 系统资源占用（CPU / 内存）后台采集
            let stats = SystemStats::default();
            stats.start();
            app.manage(stats);

            Ok(())
        })
        .plugin(tauri_plugin_store::Builder::default().build())
        // .invoke_handler(tauri::generate_handler![greet])
        .invoke_handler(tauri_helper::tauri_collect_commands!())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
