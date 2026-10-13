use std::sync::Arc;
use std::sync::atomic::{AtomicU32, Ordering};
use std::thread;
use std::time::Duration;

use sysinfo::{CpuRefreshKind, MemoryRefreshKind, RefreshKind, System};

/// 系统资源占用（CPU / 内存百分比）。
///
/// 通过后台线程定时刷新并缓存，命令调用时直接读取缓存值，
/// 避免每次请求都阻塞等待 CPU 采样间隔。
#[derive(Clone, Default)]
pub struct SystemStats {
    cpu: Arc<AtomicU32>,
    memory: Arc<AtomicU32>,
}

impl SystemStats {
    /// 启动后台刷新线程（每秒刷新一次 CPU / 内存占用）
    pub fn start(&self) {
        let cpu = self.cpu.clone();
        let memory = self.memory.clone();

        thread::spawn(move || {
            // 仅刷新 CPU 使用率与内存，避免枚举进程导致首次采样过慢
            let mut sys = System::new_with_specifics(
                RefreshKind::nothing()
                    .with_cpu(CpuRefreshKind::nothing().with_cpu_usage())
                    .with_memory(MemoryRefreshKind::nothing().with_ram()),
            );
            // 首次刷新后间隔一小段时间再采样，保证 CPU 使用率准确
            sys.refresh_cpu_usage();
            thread::sleep(Duration::from_millis(500));

            loop {
                sys.refresh_cpu_usage();
                sys.refresh_memory();

                cpu.store(sys.global_cpu_usage().round() as u32, Ordering::Relaxed);
                memory.store(
                    usage_percent(sys.used_memory(), sys.total_memory()),
                    Ordering::Relaxed,
                );

                thread::sleep(Duration::from_secs(1));
            }
        });
    }

    /// 读取当前缓存的占用百分比
    pub fn snapshot(&self) -> (u32, u32) {
        (
            self.cpu.load(Ordering::Relaxed),
            self.memory.load(Ordering::Relaxed),
        )
    }
}

/// 计算百分比（0 - 100）
fn usage_percent(used: u64, total: u64) -> u32 {
    if total == 0 {
        return 0;
    }
    ((used as f64 / total as f64) * 100.0).round() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn snapshot_returns_real_percentages() {
        let stats = SystemStats::default();
        stats.start();
        thread::sleep(Duration::from_millis(1500));
        let (cpu, memory) = stats.snapshot();
        println!("real cpu = {cpu}%, memory = {memory}%");
        assert!(cpu <= 100, "cpu out of range: {cpu}");
        assert!(memory <= 100, "memory out of range: {memory}");
        assert!(memory > 0, "memory should be greater than 0");
    }
}
