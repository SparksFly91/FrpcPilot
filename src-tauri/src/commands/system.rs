use std::net::IpAddr;

use sysinfo::{Networks, System};
use tauri::State;
use tauri_helper::auto_collect_command;

use crate::models::{ApiResponse, SystemDetails, SystemInfo};
use crate::services::system::SystemStats;

#[tauri::command]
#[auto_collect_command]
pub fn is_autostart_supported() -> bool {
    !tauri::is_dev() && !cfg!(debug_assertions)
}

/// 获取系统资源占用（CPU / 内存百分比）
#[tauri::command]
#[auto_collect_command]
pub async fn get_system_info(
    state: State<'_, SystemStats>,
) -> Result<ApiResponse<SystemInfo>, String> {
    let (cpu, memory) = state.snapshot();
    Ok(ApiResponse::success(SystemInfo { cpu, memory }))
}

/// 获取系统信息（主机名、系统版本、架构、IP 地址）
#[tauri::command]
#[auto_collect_command]
pub async fn get_system_details() -> Result<ApiResponse<SystemDetails>, String> {
    Ok(ApiResponse::success(collect_details()))
}

fn collect_details() -> SystemDetails {
    let hostname = System::host_name().unwrap_or_else(|| "-".to_string());
    let os = System::long_os_version()
        .or_else(System::os_version)
        .or_else(System::name)
        .unwrap_or_else(|| "-".to_string());
    let arch = System::cpu_arch();

    // 采集本机 IPv4 地址（过滤回环地址），并去重
    let networks = Networks::new_with_refreshed_list();
    let mut ips: Vec<String> = Vec::new();
    for data in networks.list().values() {
        for ip_net in data.ip_networks() {
            if let IpAddr::V4(ipv4) = ip_net.addr {
                if !ipv4.is_loopback() {
                    ips.push(ipv4.to_string());
                }
            }
        }
    }
    ips.sort();
    ips.dedup();

    SystemDetails {
        hostname,
        os,
        arch,
        ips,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn details_are_collected() {
        let d = collect_details();
        println!(
            "hostname={}, os={}, arch={}, ips={:?}",
            d.hostname, d.os, d.arch, d.ips
        );
        assert!(!d.hostname.is_empty());
        assert!(!d.arch.is_empty());
    }
}
