use sqlx::sqlite::{SqliteConnectOptions, SqlitePool, SqlitePoolOptions};
use tauri::{AppHandle, Manager};

pub async fn init_pool(app: &AppHandle) -> Result<SqlitePool, sqlx::Error> {
    // 数据库文件路径
    let app_dir = app.path().app_data_dir().expect("无法获取APP数据目录");
    std::fs::create_dir_all(&app_dir).unwrap();
    let db_path = app_dir.join("frpilot.db");
    // 初始化数据库连接池
    let options = SqliteConnectOptions::new()
        .filename(&db_path)
        .create_if_missing(true);
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .min_connections(1)
        .connect_with(options)
        .await?;

    // 初始化表结构
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS servers (
            id          INTEGER PRIMARY KEY AUTOINCREMENT,
            name        TEXT    NOT NULL,
            addr        TEXT    NOT NULL,
            port        INTEGER NOT NULL DEFAULT 7000,
            token       TEXT,
            protocol    TEXT    NOT NULL DEFAULT 'tcp',
            tls_enable  INTEGER NOT NULL DEFAULT 0,
            remark      TEXT,
            sort        INTEGER NOT NULL DEFAULT 0,
            pinned      INTEGER NOT NULL DEFAULT 0,
            enabled     INTEGER NOT NULL DEFAULT 0,
            created_at  TEXT    NOT NULL DEFAULT (datetime('now', 'localtime')),
            updated_at  TEXT    NOT NULL DEFAULT (datetime('now', 'localtime'))
        )
        "#,
    )
    .execute(&pool)
    .await?;

    Ok(pool)
}
