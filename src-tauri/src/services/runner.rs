//! Frpc 进程管理。
//!
//! 通过 `std::process::Command` 直接拉起 frpc 可执行文件（**刻意绕开 plugin-shell**）：
//! - frpc 与主进程互相独立，frpc 异常退出不会影响 FrpcPilot 本体；
//! - Windows 下使用 `CREATE_NO_WINDOW` 启动，避免弹出黑色控制台窗口；
//! - stdout / stderr 通过管道读取，按行解析后以 `frpc-log` 事件实时推送给前端；
//! - 后台守护线程轮询子进程状态，退出时推送 `frpc-exit` 事件。

use std::io::{BufRead, BufReader, Read};
use std::path::Path;
use std::process::{Child, Command, Stdio};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex, MutexGuard};
use std::thread;
use std::time::Duration;

use tauri::{AppHandle, Emitter};

use crate::models::{FrpcExitEvent, FrpcLogEvent, FrpcStatus};

/// 实时日志事件名
pub const LOG_EVENT: &str = "frpc-log";
/// 进程退出事件名
pub const EXIT_EVENT: &str = "frpc-exit";

/// 守护线程轮询间隔
const POLL_INTERVAL: Duration = Duration::from_millis(250);

/// 运行中的 frpc
struct Running {
    child: Child,
    /// 本次运行的唯一标识，避免停止后新进程被旧守护线程误判
    token: u64,
    version: String,
    frpc_path: String,
    config_path: String,
}

/// Frpc 进程管理器（应用级单例）
#[derive(Default)]
pub struct FrpcRunner {
    slot: Arc<Mutex<Option<Running>>>,
    counter: AtomicU64,
}

impl FrpcRunner {
    fn lock(&self) -> MutexGuard<'_, Option<Running>> {
        // 中毒的锁直接恢复内部数据，避免一次 panic 后永远无法再启动 frpc
        self.slot.lock().unwrap_or_else(|e| e.into_inner())
    }

    /// 当前是否有 frpc 在运行
    pub fn is_running(&self) -> bool {
        self.lock().is_some()
    }

    /// 运行状态快照
    pub fn status(&self) -> FrpcStatus {
        let guard = self.lock();
        match guard.as_ref() {
            Some(running) => FrpcStatus {
                running: true,
                version: Some(running.version.clone()),
                frpc_path: Some(running.frpc_path.clone()),
                config_path: Some(running.config_path.clone()),
                pid: Some(running.child.id()),
            },
            None => FrpcStatus::default(),
        }
    }

    /// 启动 frpc。
    ///
    /// `frpc_path` 为可执行文件路径，`config_path` 为本次生成的 frpc.toml 路径。
    pub fn spawn(
        &self,
        app: &AppHandle,
        frpc_path: &str,
        config_path: &str,
        version: &str,
    ) -> Result<FrpcStatus, String> {
        let mut guard = self.lock();
        if guard.is_some() {
            return Err("Frpc 已在运行中".to_string());
        }

        let exe = Path::new(frpc_path);
        if !exe.is_file() {
            return Err(format!("未找到 frpc 可执行文件：{frpc_path}"));
        }

        let mut command = Command::new(exe);
        command
            .arg("-c")
            .arg(config_path)
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());

        // 工作目录设为 frpc 所在目录，保证其相对路径配置可正常解析
        if let Some(dir) = exe.parent() {
            command.current_dir(dir);
        }

        // Windows：不创建控制台窗口（避免黑色窗口）
        #[cfg(windows)]
        {
            use std::os::windows::process::CommandExt;
            const CREATE_NO_WINDOW: u32 = 0x0800_0000;
            const CREATE_NEW_PROCESS_GROUP: u32 = 0x0000_0200;
            command.creation_flags(CREATE_NO_WINDOW | CREATE_NEW_PROCESS_GROUP);
        }

        let mut child = command
            .spawn()
            .map_err(|e| format!("启动 Frpc 失败：{e}"))?;

        let pid = child.id();
        let stdout = child.stdout.take();
        let stderr = child.stderr.take();
        let token = self.counter.fetch_add(1, Ordering::SeqCst) + 1;

        *guard = Some(Running {
            child,
            token,
            version: version.to_string(),
            frpc_path: frpc_path.to_string(),
            config_path: config_path.to_string(),
        });
        drop(guard);

        // 采集 stdout / stderr
        if let Some(stdout) = stdout {
            spawn_reader(app.clone(), stdout, "stdout");
        }
        if let Some(stderr) = stderr {
            spawn_reader(app.clone(), stderr, "stderr");
        }

        // 守护线程：轮询子进程是否退出
        spawn_waiter(app.clone(), self.slot.clone(), token);

        emit_log(
            app,
            FrpcLogEvent {
                time: None,
                level: "INFO".to_string(),
                source: "system".to_string(),
                message: format!("Frpc {version} 已启动（PID {pid}）"),
            },
        );

        Ok(self.status())
    }

    /// 停止 frpc（用户主动停止）。未运行时为空操作。
    pub fn stop(&self, app: &AppHandle) -> Result<FrpcStatus, String> {
        let mut guard = self.lock();
        if guard.is_none() {
            return Ok(FrpcStatus::default());
        }

        emit_log(
            app,
            FrpcLogEvent {
                time: None,
                level: "WARN".to_string(),
                source: "system".to_string(),
                message: "正在停止 Frpc…".to_string(),
            },
        );

        let version = {
            let running = guard.as_mut().expect("guard 已确认非空");
            let version = running.version.clone();
            // kill + wait 回收进程，避免残留僵尸进程
            let _ = running.child.kill();
            let _ = running.child.wait();
            version
        };
        *guard = None;
        drop(guard);

        emit_log(
            app,
            FrpcLogEvent {
                time: None,
                level: "WARN".to_string(),
                source: "system".to_string(),
                message: "Frpc 已停止".to_string(),
            },
        );
        emit_exit(
            app,
            FrpcExitEvent {
                reason: "stopped".to_string(),
                code: None,
                version: Some(version),
            },
        );

        Ok(FrpcStatus::default())
    }

    /// 应用退出时静默结束 frpc（不发送事件）
    pub fn shutdown(&self) {
        let mut guard = self.lock();
        if let Some(running) = guard.as_mut() {
            let _ = running.child.kill();
            let _ = running.child.wait();
        }
        *guard = None;
    }
}

/// 持续读取子进程输出并推送日志事件
fn spawn_reader<R: Read + Send + 'static>(app: AppHandle, pipe: R, source: &'static str) {
    thread::spawn(move || {
        let mut reader = BufReader::new(pipe);
        let mut buf: Vec<u8> = Vec::new();
        loop {
            buf.clear();
            match reader.read_until(b'\n', &mut buf) {
                Ok(0) => break,
                Ok(_) => {
                    let raw = String::from_utf8_lossy(&buf);
                    let line = strip_ansi(raw.trim_end_matches(['\r', '\n']));
                    if line.trim().is_empty() {
                        continue;
                    }
                    emit_log(&app, parse_line(source, &line));
                }
                Err(_) => break,
            }
        }
    });
}

/// 守护线程：轮询子进程状态，退出时清理并推送事件
fn spawn_waiter(app: AppHandle, slot: Arc<Mutex<Option<Running>>>, token: u64) {
    thread::spawn(move || loop {
        thread::sleep(POLL_INTERVAL);

        let mut guard = slot.lock().unwrap_or_else(|e| e.into_inner());
        let probe = match guard.as_mut() {
            Some(running) if running.token == token => {
                Some((running.child.try_wait(), running.version.clone()))
            }
            // 已被停止 / 已被新进程替换
            _ => None,
        };

        match probe {
            None => return,
            Some((Ok(Some(status)), version)) => {
                *guard = None;
                drop(guard);
                let code = status.code();
                emit_log(
                    &app,
                    FrpcLogEvent {
                        time: None,
                        level: if code.unwrap_or(1) == 0 { "WARN" } else { "ERROR" }.to_string(),
                        source: "system".to_string(),
                        message: match code {
                            Some(code) => format!("Frpc 进程已退出（退出码 {code}）"),
                            None => "Frpc 进程已退出".to_string(),
                        },
                    },
                );
                emit_exit(
                    &app,
                    FrpcExitEvent {
                        reason: "exited".to_string(),
                        code,
                        version: Some(version),
                    },
                );
                return;
            }
            Some((Ok(None), _)) => {} // 仍在运行
            Some((Err(_), _)) => return,
        }
    });
}

/// 解析一行 frpc 日志，识别时间戳与级别
fn parse_line(source: &str, line: &str) -> FrpcLogEvent {
    FrpcLogEvent {
        time: extract_time(line),
        level: detect_level(line),
        source: source.to_string(),
        message: line.to_string(),
    }
}

/// 从行首附近提取 `HH:MM:SS` 形式的时间戳
fn extract_time(line: &str) -> Option<String> {
    for token in line.split_whitespace().take(3) {
        let token = token.trim_end_matches(',');
        let head: String = token.chars().take(8).collect();
        if head.len() == 8
            && head.matches(':').count() == 2
            && head.chars().all(|c| c.is_ascii_digit() || c == ':')
        {
            return Some(head);
        }
    }
    None
}

/// 识别 `[I]` / `[W]` / `[E]` / `[D]` / `[T]` 级别标记
fn detect_level(line: &str) -> String {
    const MARKERS: [(&str, &str); 5] = [
        ("[I]", "INFO"),
        ("[W]", "WARN"),
        ("[E]", "ERROR"),
        ("[D]", "DEBUG"),
        ("[T]", "TRACE"),
    ];

    let mut best: Option<(usize, &str)> = None;
    for (marker, name) in MARKERS {
        if let Some(index) = line.find(marker) {
            if index <= 30 && best.is_none_or(|(best_index, _)| index < best_index) {
                best = Some((index, name));
            }
        }
    }
    if let Some((_, name)) = best {
        return name.to_string();
    }

    // 兜底：frpc 部分输出不带级别标记（如登录失败提示），按关键词判定
    let lower = line.to_ascii_lowercase();
    if lower.contains("failed") || lower.contains("error") || lower.contains("fatal") {
        "ERROR".to_string()
    } else if lower.contains("warn") {
        "WARN".to_string()
    } else {
        "INFO".to_string()
    }
}

/// 剥离 ANSI 转义序列（CSI / OSC），避免终端颜色码混入界面日志
///
/// frpc 已由配置项 `log.disablePrintColor` 关闭颜色，这里作为兜底，
/// 兼容用户自行导入的旧版 frpc 或未来可能变化的输出。
fn strip_ansi(input: &str) -> String {
    let mut out = String::with_capacity(input.len());
    let mut chars = input.chars().peekable();
    while let Some(ch) = chars.next() {
        if ch != '\u{1b}' {
            out.push(ch);
            continue;
        }
        // ESC 序列：根据引导字符决定消费方式
        match chars.peek().copied() {
            // CSI：ESC [ ... 直到终止字节 0x40 - 0x7E
            Some('[') => {
                chars.next();
                for c in chars.by_ref() {
                    if ('\u{40}'..='\u{7e}').contains(&c) {
                        break;
                    }
                }
            }
            // OSC：ESC ] ... 直到 BEL 或 ST（ESC \）
            Some(']') => {
                chars.next();
                while let Some(c) = chars.next() {
                    if c == '\u{7}' {
                        break;
                    }
                    if c == '\u{1b}' && chars.peek() == Some(&'\\') {
                        chars.next();
                        break;
                    }
                }
            }
            // 其它两字节转义序列：ESC + 单个字符
            Some(_) => {
                chars.next();
            }
            None => {}
        }
    }
    out
}

/// 发送日志事件（失败时忽略，避免影响进程管理）
fn emit_log(app: &AppHandle, event: FrpcLogEvent) {
    let _ = app.emit(LOG_EVENT, event);
}

/// 发送退出事件
fn emit_exit(app: &AppHandle, event: FrpcExitEvent) {
    let _ = app.emit(EXIT_EVENT, event);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_log_level() {
        assert_eq!(detect_level("2026/06/18 22:02:03 [I] [root.go:200] ok"), "INFO");
        assert_eq!(detect_level("2026/06/18 22:02:03 [E] [root.go:200] bad"), "ERROR");
        assert_eq!(detect_level("[W] warn only"), "WARN");
        assert_eq!(detect_level("plain message"), "INFO");
    }

    #[test]
    fn extracts_timestamp() {
        assert_eq!(
            extract_time("2026/06/18 22:02:03 [I] msg"),
            Some("22:02:03".to_string())
        );
        assert_eq!(extract_time("no time here"), None);
    }

    #[test]
    fn strips_ansi_escape_codes() {
        let input = "\u{1b}[1;34m2026-10-13 17:19:48.171 [I] hello\u{1b}[0m";
        assert_eq!(strip_ansi(input), "2026-10-13 17:19:48.171 [I] hello");
        // 无转义码时保持原样
        assert_eq!(strip_ansi("plain text"), "plain text");
    }

    #[test]
    fn detects_level_after_stripping_ansi() {
        let line =
            strip_ansi("\u{1b}[1;33m2026-10-13 17:19:50.288 [W] connect to server error\u{1b}[0m");
        assert_eq!(detect_level(&line), "WARN");
    }

    #[test]
    fn falls_back_to_error_for_failed_lines() {
        assert_eq!(
            detect_level("login to the server failed: dial tcp 1.2.3.4:7000"),
            "ERROR"
        );
        assert_eq!(detect_level("connection timeout, retrying"), "INFO");
    }
}
