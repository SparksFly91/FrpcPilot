use sqlx::SqlitePool;
use tauri::State;
use tauri_helper::auto_collect_command;

use crate::models::{ApiResponse};

#[tauri::command]
#[auto_collect_command]
pub async fn get_server_list(state: State<'_, SqlitePool>) -> Result<ApiResponse<()>, String> {
    Ok(ApiResponse::success_empty())
}