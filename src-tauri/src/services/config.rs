//! frpc 配置生成。
//!
//! 根据数据库中「已启用的服务器」与「已启用的隧道」拼装出 frpc 的 TOML 配置。
//! 生成的文件仅包含当前启用的条目，Frpc 每次启动都会重新生成并覆盖。

use crate::models::{Server, Tunnel};

/// 生成 frpc.toml 内容。
///
/// 采用 frp v1 的 TOML 配置格式（`frpc -c frpc.toml`），字段命名与
/// 隧道表单中的注释一致，例如 `localIP` / `remotePort` / `transport.useEncryption`。
pub fn generate_frpc_config(server: &Server, tunnels: &[Tunnel]) -> String {
    let mut out = String::new();

    out.push_str("# 该文件由 FrpcPilot 自动生成，请勿手动修改（重新启动 Frpc 会覆盖此文件）\n\n");

    // 日志：输出到控制台，由 FrpcPilot 采集并实时展示
    // 关闭 ANSI 颜色，避免日志中混入 `\x1b[1;34m` 之类的转义码
    out.push_str("log.to = \"console\"\n");
    out.push_str("log.level = \"info\"\n");
    out.push_str("log.disablePrintColor = true\n\n");

    // 服务器（公共）配置
    out.push_str(&format!("serverAddr = {}\n", toml_str(&server.addr)));
    out.push_str(&format!("serverPort = {}\n", server.port));

    if let Some(token) = non_empty(&server.token) {
        out.push_str("auth.method = \"token\"\n");
        out.push_str(&format!("auth.token = {}\n", toml_str(&token)));
    }

    out.push_str(&format!(
        "transport.protocol = {}\n",
        toml_str(&server.protocol)
    ));
    out.push_str(&format!(
        "transport.tls.enable = {}\n",
        server.tls_enable
    ));

    // 隧道（代理）配置
    for tunnel in tunnels {
        out.push('\n');
        out.push_str("[[proxies]]\n");
        out.push_str(&format!("name = {}\n", toml_str(&tunnel.name)));
        out.push_str(&format!("type = {}\n", toml_str(&tunnel.proxy_type)));
        out.push_str(&format!("localIP = {}\n", toml_str(&tunnel.local_ip)));
        out.push_str(&format!("localPort = {}\n", tunnel.local_port));

        if let Some(remote_port) = tunnel.remote_port {
            out.push_str(&format!("remotePort = {remote_port}\n"));
        }
        if let Some(list) = split_list(&tunnel.custom_domains) {
            out.push_str(&format!("customDomains = {}\n", toml_array(&list)));
        }
        if let Some(subdomain) = non_empty(&tunnel.subdomain) {
            out.push_str(&format!("subdomain = {}\n", toml_str(&subdomain)));
        }
        if let Some(list) = split_list(&tunnel.locations) {
            out.push_str(&format!("locations = {}\n", toml_array(&list)));
        }
        if let Some(secret_key) = non_empty(&tunnel.secret_key) {
            out.push_str(&format!("secretKey = {}\n", toml_str(&secret_key)));
        }

        // 代理内部传输配置
        let mut transport = String::new();
        if tunnel.use_encryption {
            transport.push_str("useEncryption = true\n");
        }
        if tunnel.use_compression {
            transport.push_str("useCompression = true\n");
        }
        if let Some(limit) = non_empty(&tunnel.bandwidth_limit) {
            transport.push_str(&format!("bandwidthLimit = {}\n", toml_str(&limit)));
        }
        if let Some(version) = non_empty(&tunnel.proxy_protocol_version) {
            transport.push_str(&format!("proxyProtocolVersion = {}\n", toml_str(&version)));
        }
        if !transport.is_empty() {
            out.push_str("\n[proxies.transport]\n");
            out.push_str(&transport);
        }

        // 健康检查
        if let Some(check_type) = non_empty(&tunnel.health_check_type) {
            out.push_str("\n[proxies.healthCheck]\n");
            out.push_str(&format!("type = {}\n", toml_str(&check_type)));
            if let Some(value) = tunnel.health_check_timeout {
                out.push_str(&format!("timeoutSeconds = {value}\n"));
            }
            if let Some(value) = tunnel.health_check_max_failed {
                out.push_str(&format!("maxFailed = {value}\n"));
            }
            if let Some(value) = tunnel.health_check_interval {
                out.push_str(&format!("intervalSeconds = {value}\n"));
            }
            if let Some(path) = non_empty(&tunnel.health_check_path) {
                out.push_str(&format!("path = {}\n", toml_str(&path)));
            }
        }

        // 负载均衡
        if let Some(group) = non_empty(&tunnel.load_balancer_group) {
            out.push_str("\n[proxies.loadBalancer]\n");
            out.push_str(&format!("group = {}\n", toml_str(&group)));
            if let Some(key) = non_empty(&tunnel.load_balancer_group_key) {
                out.push_str(&format!("groupKey = {}\n", toml_str(&key)));
            }
        }
    }

    out
}

/// 去除首尾空白，空字符串返回 None
fn non_empty(value: &Option<String>) -> Option<String> {
    value
        .as_ref()
        .map(|v| v.trim().to_string())
        .filter(|v| !v.is_empty())
}

/// 英文逗号分隔字符串 → 字符串数组（空返回 None）
fn split_list(value: &Option<String>) -> Option<Vec<String>> {
    let list: Vec<String> = value
        .as_deref()
        .unwrap_or_default()
        .split(',')
        .map(|item| item.trim().to_string())
        .filter(|item| !item.is_empty())
        .collect();
    if list.is_empty() { None } else { Some(list) }
}

/// 字符串数组 → TOML 数组字面量，如 `["a", "b"]`
fn toml_array(values: &[String]) -> String {
    let items: Vec<String> = values.iter().map(|v| toml_str(v)).collect();
    format!("[{}]", items.join(", "))
}

/// 转义并包裹为 TOML 字符串
fn toml_str(value: &str) -> String {
    let mut out = String::with_capacity(value.len() + 2);
    out.push('"');
    for ch in value.chars() {
        match ch {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if (c as u32) < 0x20 => out.push_str(&format!("\\u{:04X}", c as u32)),
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_server() -> Server {
        Server {
            id: 1,
            name: "本地".into(),
            addr: "1.2.3.4".into(),
            port: 7000,
            token: Some("secret".into()),
            protocol: "tcp".into(),
            tls_enable: true,
            remark: None,
            sort: 0,
            pinned: false,
            enabled: true,
            created_at: String::new(),
            updated_at: String::new(),
        }
    }

    fn sample_tunnel() -> Tunnel {
        Tunnel {
            id: 1,
            name: "ssh".into(),
            proxy_type: "tcp".into(),
            local_ip: "127.0.0.1".into(),
            local_port: 22,
            remote_port: Some(6000),
            custom_domains: None,
            subdomain: None,
            locations: None,
            secret_key: None,
            use_encryption: true,
            use_compression: false,
            bandwidth_limit: None,
            proxy_protocol_version: None,
            health_check_type: None,
            health_check_timeout: None,
            health_check_max_failed: None,
            health_check_interval: None,
            health_check_path: None,
            load_balancer_group: None,
            load_balancer_group_key: None,
            remark: None,
            sort: 0,
            pinned: false,
            enabled: true,
            created_at: String::new(),
            updated_at: String::new(),
        }
    }

    #[test]
    fn generates_basic_config() {
        let config = generate_frpc_config(&sample_server(), &[sample_tunnel()]);
        assert!(config.contains("serverAddr = \"1.2.3.4\""));
        assert!(config.contains("serverPort = 7000"));
        assert!(config.contains("auth.token = \"secret\""));
        assert!(config.contains("log.disablePrintColor = true"));
        assert!(config.contains("[[proxies]]"));
        assert!(config.contains("remotePort = 6000"));
        assert!(config.contains("[proxies.transport]"));
        assert!(config.contains("useEncryption = true"));
    }

    #[test]
    fn escapes_special_characters() {
        assert_eq!(toml_str("a\"b\\c"), "\"a\\\"b\\\\c\"");
    }

    #[test]
    fn splits_domain_list() {
        assert_eq!(
            split_list(&Some("a.com, b.com ,".into())),
            Some(vec!["a.com".to_string(), "b.com".to_string()])
        );
        assert_eq!(split_list(&Some("".into())), None);
    }
}
