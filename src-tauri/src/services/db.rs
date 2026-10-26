use sqlx::sqlite::{SqliteConnectOptions, SqlitePool, SqlitePoolOptions};
use std::path::Path;

pub async fn init_pool(db_path: &Path) -> Result<SqlitePool, sqlx::Error> {
    // 初始化数据库连接池
    let options = SqliteConnectOptions::new()
        .filename(db_path)
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

    // 隧道表（字段参考 frpc 的 proxy 配置）
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS tunnels (
            id                       INTEGER PRIMARY KEY AUTOINCREMENT,
            name                     TEXT    NOT NULL,
            proxy_type               TEXT    NOT NULL DEFAULT 'tcp',
            local_ip                 TEXT    NOT NULL DEFAULT '127.0.0.1',
            local_port               INTEGER NOT NULL,
            remote_port              INTEGER,
            custom_domains           TEXT,
            subdomain                TEXT,
            locations                TEXT,
            secret_key               TEXT,
            use_encryption           INTEGER NOT NULL DEFAULT 0,
            use_compression          INTEGER NOT NULL DEFAULT 0,
            bandwidth_limit          TEXT,
            proxy_protocol_version   TEXT,
            health_check_type        TEXT,
            health_check_timeout     INTEGER,
            health_check_max_failed  INTEGER,
            health_check_interval    INTEGER,
            health_check_path        TEXT,
            load_balancer_group      TEXT,
            load_balancer_group_key  TEXT,
            remark                   TEXT,
            sort                     INTEGER NOT NULL DEFAULT 0,
            pinned                   INTEGER NOT NULL DEFAULT 0,
            enabled                  INTEGER NOT NULL DEFAULT 0,
            created_at               TEXT    NOT NULL DEFAULT (datetime('now', 'localtime')),
            updated_at               TEXT    NOT NULL DEFAULT (datetime('now', 'localtime'))
        )
        "#,
    )
    .execute(&pool)
    .await?;

    // 已下载 / 已导入的 Frp 版本表
    // 同一版本 + 平台唯一，一个发行包内同时含 frpc 与 frps 两个可执行文件
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS frp_downloads (
            id           INTEGER PRIMARY KEY AUTOINCREMENT,
            version      TEXT    NOT NULL,
            os           TEXT    NOT NULL,
            arch         TEXT    NOT NULL,
            platform     TEXT    NOT NULL,
            file_name    TEXT    NOT NULL,
            archive_path TEXT,
            frpc_path    TEXT,
            frps_path    TEXT,
            file_size    INTEGER NOT NULL DEFAULT 0,
            source       TEXT    NOT NULL DEFAULT 'download',
            created_at   TEXT    NOT NULL DEFAULT (datetime('now', 'localtime')),
            updated_at   TEXT    NOT NULL DEFAULT (datetime('now', 'localtime')),
            UNIQUE(version, platform)
        )
        "#,
    )
    .execute(&pool)
    .await?;

    Ok(pool)
}
