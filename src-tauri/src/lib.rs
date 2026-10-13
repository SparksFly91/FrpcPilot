mod commands;
mod models;
mod services;

use tauri::Manager;

use crate::commands::server::{
    create_server, delete_server, get_server_list, set_server_enabled, set_server_pinned,
    update_server,
};
use crate::services::db::init_pool;

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

            Ok(())
        })
        .plugin(tauri_plugin_store::Builder::default().build())
        // .invoke_handler(tauri::generate_handler![greet])
        .invoke_handler(tauri_helper::tauri_collect_commands!())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
