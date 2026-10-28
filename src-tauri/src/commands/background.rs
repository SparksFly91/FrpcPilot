use tauri::{AppHandle, State};
use tauri_helper::auto_collect_command;
use tauri_plugin_dialog::DialogExt;

use crate::models::ApiResponse;
use crate::services::background::{find_custom_background, import_custom_background};
use crate::services::storage::StoragePaths;

/// 打开原生文件选择器并将图片复制到应用数据目录。
#[tauri::command]
#[auto_collect_command]
pub async fn choose_custom_background(
    app: AppHandle,
    paths: State<'_, StoragePaths>,
) -> Result<ApiResponse<Option<String>>, String> {
    let picked = app
        .dialog()
        .file()
        .add_filter("图片", &["png", "jpg", "jpeg", "webp"])
        .set_title("选择背景图片")
        .blocking_pick_file();
    let Some(file) = picked else {
        return Ok(ApiResponse::success(None));
    };
    let source = file.into_path().map_err(|e| e.to_string())?;
    let saved = import_custom_background(&paths, &source)?;
    Ok(ApiResponse::success(Some(
        saved.to_string_lossy().into_owned(),
    )))
}

/// 获取已保存的自定义背景路径，图片内容通过受限的 asset 协议读取。
#[tauri::command]
#[auto_collect_command]
pub async fn get_custom_background(
    paths: State<'_, StoragePaths>,
) -> Result<ApiResponse<Option<String>>, String> {
    Ok(ApiResponse::success(
        find_custom_background(&paths).map(|path| path.to_string_lossy().into_owned()),
    ))
}
