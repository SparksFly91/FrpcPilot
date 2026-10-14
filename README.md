# Frpilot

> 一个基于 **Tauri 2 + Vue 3 + TypeScript** 的 [frp](https://github.com/fatedier/frp) 可视化客户端，帮你用图形界面管理 frpc 版本、服务器与隧道，从此告别手写 `frpc.toml`。

Frpilot 已从「应用骨架」成长为可用的一体化客户端：下载 frpc、配置服务器、编排隧道、一键启动并实时查看日志，全部在同一个窗口里完成，并内置应用内自动更新。

---

## ✨ 已实现功能

### Frpc 版本管理

- 在线拉取 Frp 官方版本列表，支持「所有架构 frpc / frps」开关，按平台与架构筛选
- 一键下载并自动解压，实时显示下载 / 解压进度与状态
- 导入本地已有的 `frpc` / `frps`
- 删除已下载版本、复制下载直链

### 服务器管理

- frps 服务器的新增 / 编辑 / 删除
- 服务器地址、端口、鉴权 Token、传输协议、TLS 开关、备注
- 置顶与排序；启用开关（**同一时间仅允许启用一台**服务器）

### 隧道管理

- 覆盖 **TCP / UDP / HTTP / HTTPS / STCP / XTCP / SUDP / TCPMUX** 等代理类型
- 本地 IP / 端口、远程端口、自定义域名、子域名、URL 前缀
- 高级配置：密钥、加密、压缩、带宽限制、Proxy Protocol、健康检查、负载均衡
- 启用 / 置顶 / 排序

### 一键启动与实时日志

- 仪表盘选择 frpc 版本后「一键启动 / 停止」
- 自动依据所选版本、已启用的服务器与隧道生成 `frpc.toml` 并拉起独立进程
- 通过 `frpc-log` / `frpc-exit` 事件实时回显运行日志与进程退出信息

### 仪表盘

- 系统信息：主机名 / 系统 / 架构 / IP 地址
- 服务器与隧道数量统计
- 侧边栏常驻 CPU 占用、内存占用与运行状态指示点（🟢 运行正常 · 🔴 运行异常 · 🟡 未启动）

### 应用内自动更新

- 启动时静默检查，支持手动检查更新
- 更新弹窗以 **Markdown** 渲染更新说明，展示下载进度
- 支持「忽略该版本」、一键更新，安装完成后自动重启

### 关于页面

- 项目介绍、技术栈标签、使用流程指引
- 开源地址一键跳转、当前版本信息与「检查更新」入口

### 界面与体验

- **自定义标题栏**：Windows / macOS 双套方案，窗口可拖拽移动；顶部栏集成语言 / 主题 / 检查更新 / GitHub / 设置与窗口控制按钮
- **macOS 特化**：还原原生红绿灯，隐藏冗余的最小化 / 关闭按钮，禁用 macOS 下不需要的最大化
- **明暗主题**：跟随切换，Tailwind 暗色变体与 Ant Design Vue 暗色算法同步，选择持久化
- **悬浮磨砂玻璃**：侧边栏、内容区与卡片统一为圆角 + `backdrop-filter` 半透明面板
- **国际化**：内置简体中文 / English，语言选择持久化，Ant Design Vue 组件语言包联动

---

## 🖼️ 界面预览

| 仪表盘 | 服务器 |
| --- | --- |
| ![仪表盘](docs/snipastes/仪表盘.webp) | ![服务器](docs/snipastes/服务器.webp) |

| 隧道列表 | Frpc 下载 |
| --- | --- |
| ![隧道列表](docs/snipastes/隧道列表.webp) | ![Frpc 下载](docs/snipastes/frpc下载.webp) |

| 设置 · 外观 | 设置 · 关于 |
| --- | --- |
| ![外观设置](docs/snipastes/设置.webp) | ![关于](docs/snipastes/关于.webp) |

---

## 🧱 技术栈

| 分类       | 选型                                                                  |
| ---------- | --------------------------------------------------------------------- |
| 桌面框架   | [Tauri 2](https://v2.tauri.app/)（Rust 后端）                          |
| 前端框架   | Vue 3（`<script setup>`）+ TypeScript                                  |
| 构建工具   | Vite 8                                                                |
| UI 组件库  | Ant Design Vue 4 + `@ant-design/icons-vue`                            |
| 样式       | Tailwind CSS 4（原子化）+ SCSS                                        |
| 状态管理   | Pinia                                                                 |
| 路由       | Vue Router                                                            |
| 国际化     | Vue I18n 11                                                           |
| Markdown   | `markdown-it`（更新日志渲染）                                          |
| 工具库     | `@vueuse/core`                                                        |
| 后端       | Rust：`sqlx` + SQLite、`reqwest`（下载）、`sysinfo`（系统采集）        |
| 插件       | `updater` / `process` / `dialog` / `opener` / `store` / `prevent-default` |
| 自动化     | `unplugin-auto-import`、`unplugin-vue-components`（Antd 按需自动导入） |

---

## 📦 环境要求

- **Node.js** ≥ 20（推荐 20 / 22 LTS）
- **pnpm** ≥ 9（项目使用 pnpm，含 `pnpm-lock.yaml`）
- **Rust** 稳定版工具链（`rustup`，建议 ≥ 1.90）
- **系统依赖**：请参考 [Tauri 前置依赖](https://v2.tauri.app/start/prerequisites/)（Windows 需 WebView2 与 MSVC 构建工具；macOS 需 Xcode Command Line Tools；Linux 需 `webkit2gtk` 等）

---

## 🚀 快速开始

```bash
# 1. 安装依赖
pnpm install

# 2. 仅启动前端（浏览器调试，窗口 / 命令类操作会安全空转）
pnpm dev

# 3. 启动完整桌面应用（推荐，含 Rust 后端）
pnpm tauri dev
```

### 常用脚本

| 命令               | 说明                                                   |
| ------------------ | ------------------------------------------------------ |
| `pnpm dev`         | 启动 Vite 开发服务器（端口 `1420`）                     |
| `pnpm tauri dev`   | 启动 Tauri 桌面开发环境（自动拉起前端）                 |
| `pnpm build`       | 类型检查（`vue-tsc --noEmit`）+ 构建前端产物到 `dist/`  |
| `pnpm preview`     | 预览前端构建产物                                       |
| `pnpm tauri build` | 打包生成各平台安装包                                   |

> ⚠️ `pnpm dev` 的端口固定为 `1420` 且 `strictPort: true`，占用时会直接报错退出。

---

## 📁 项目结构

```
Frpilot/
├── src/                            # 前端（Vue 3 + TypeScript）
│   ├── api/                        # Tauri 命令封装（frp / server / tunnel / system / updater）
│   ├── assets/main.css             # Tailwind 入口 + 全局样式（磨砂玻璃 / Markdown / 弹窗）
│   ├── components/                 # 通用组件
│   │   ├── frp/FrpCard.vue
│   │   ├── server/ServerCard.vue · ServerFormModal.vue
│   │   ├── tunnel/TunnelCard.vue · TunnelFormModal.vue
│   │   ├── WindowsTitleBar.vue · MacTitleBar.vue · TitleBarActions.vue
│   │   ├── UpdateModal.vue         # 更新弹窗（Markdown 更新日志 + 进度）
│   │   └── MarkdownBlock.vue       # Markdown 渲染
│   ├── composables/                # usePlatform · useSystemStats
│   ├── config/backgrounds.ts       # 内置窗口背景图
│   ├── i18n/                       # 国际化（zh-CN / en-US）
│   ├── layout/                     # 主布局 + 标题栏 + 侧边栏
│   ├── router/router.ts            # 路由表
│   ├── stores/                     # app · frpc · update（Pinia）
│   ├── types/                      # 自动生成的类型声明与领域类型
│   ├── utils/                      # window（窗口控制）· markdown（渲染）
│   └── views/                      # 页面（仪表盘 / 服务器 / 隧道 / 下载 / 设置）
├── src-tauri/                      # 后端（Rust）
│   ├── src/
│   │   ├── commands/               # Tauri 命令（frp / runner / server / system / tunnel）
│   │   ├── services/               # 数据库 / frpc.toml 生成 / 进程管理 / 系统采集
│   │   ├── models.rs               # ApiResponse / PageResult 等通用模型
│   │   └── lib.rs                  # 应用入口与命令注册
│   ├── capabilities/default.json   # 权限声明（窗口控制 / updater / process 等）
│   └── tauri.conf.json             # Tauri 配置（无边框窗口 1100×720）
├── docs/
│   ├── CHANGELOG.md                # 版本更新日志（发布流程会读取）
│   └── snipastes/                  # 界面截图
├── .github/workflows/release.yml   # 多平台发布工作流
├── index.html
├── vite.config.ts                  # 别名 / 自动导入 / Tailwind 插件
└── package.json
```

---

## 🛠️ 开发指南

### 别名与自动导入

- `@` 指向 `src/`
- 自动导入已开启，以下内容**无需手动 import**：
  - Vue / Vue Router / Pinia / `@vueuse/core` 的 API（如 `ref`、`computed`、`watch`、`useDark`、`useStorage`）
  - Ant Design Vue 组件（模板中直接写 `<a-button>`、`<a-menu>` 等）与图标
  - 类型声明自动生成在 `src/types/auto-imports.d.ts`、`src/types/components.d.ts`

> `vue-i18n` 的 `useI18n` 未加入自动导入，需手动 `import { useI18n } from 'vue-i18n'`。

### 新增 / 修改文案（i18n）

1. 到 `src/i18n/locales/zh-CN.json` 与 `en-US.json` 中按相同层级添加 key（两份文件需保持一致）。
2. 组件内使用：

```vue
<script setup lang="ts">
import { useI18n } from 'vue-i18n'
const { t } = useI18n()
</script>

<template>
  <span>{{ t('menu.dashboard') }}</span>
</template>
```

新增语言：在 `src/i18n/locales/` 添加 JSON，并在 `src/i18n/index.ts` 的 `messages` 与 `languageOptions` 中注册。

### 新增侧边栏菜单项与页面

1. 在 `src/views/` 新建页面组件。
2. 在 `src/router/router.ts` 的 `Layout` 子路由中注册（路径与菜单 `key` 一致）。
3. 在 `src/layout/aside/Aside.vue` 的 `menuItems` 中追加一项（`key` = 路由路径）。
4. 在语言包中补充对应 `menu.*` 文案。

### 明暗主题

- 主题状态在 `src/stores/app.ts`（`isDark` / `toggleDark`），由 `useDark` 在 `<html>` 上切换 `.dark` 类名并持久化。
- Tailwind 使用 class 策略：在 `src/assets/main.css` 中已声明 `@custom-variant dark`。
- Ant Design Vue 主题在 `src/App.vue` 中通过 `a-config-provider` 的 `theme.algorithm` 联动。

### 磨砂玻璃样式

`src/assets/main.css` 中的 `.glass-panel`（面板）与 `.glass-card`（卡片）为通用毛玻璃样式，直接加类名即可（已含暗色适配）。

### Markdown 渲染

更新日志、说明类富文本统一使用 `src/components/MarkdownBlock.vue`（内部调用 `src/utils/markdown.ts` 的 `renderMarkdown`）。渲染关闭了原始 HTML，并把链接点击接管到系统浏览器打开。

### 窗口控制与权限

- 前端封装：`src/utils/window.ts`（`minimizeWindow` / `toggleMaximizeWindow` / `closeWindow` / `startDragging` 等），在非 Tauri 环境（浏览器）下自动空转。
- 自定义标题栏依赖无边框窗口（`src-tauri/tauri.conf.json` 中 `decorations: false`），拖拽区域通过 `data-tauri-drag-region` 实现。
- **窗口圆角（8px）**：Windows 11 由系统 DWM 自动裁切；macOS 的无边框窗口系统不会裁圆角，因此：
  - `src-tauri/tauri.macos.conf.json` 中把窗口设为 `transparent: true`（**Tauri 2.12.1+ 无需再开启 `macOSPrivateApi`**）；
  - `src/assets/main.css` 中在 `html.is-macos` 下把背景从 `<html>` 移到 `#app`，由 `#app` 裁出 8px 圆角，并让 `html`/`body` 透明以透出桌面；
  - `src/main.ts` 在挂载前根据 UA 给 `<html>` 打上 `is-macos` 标记。
  - ⚠️ 平台配置文件按 **JSON Merge Patch** 合并，且**数组整体替换**，因此 `tauri.macos.conf.json` 里必须完整重复 `app.windows` 的字段；**修改基础配置的窗口参数时，记得同步这份文件**。
- 相关权限在 `src-tauri/capabilities/default.json` 中声明（`core:window:*`、`updater:default`、`process:default` 等）。**新增窗口 / 插件 API 调用时需同步补充对应权限**，否则会报 `not allowed` 错误。

### 发布与自动更新

- 版本号取自 `package.json` 的 `version`（`src-tauri/tauri.conf.json` 引用之），`src-tauri/Cargo.toml` 保持一致。
- 推送 `v*` tag 会触发 `.github/workflows/release.yml`：构建 **Windows x64 / macOS Apple Silicon / Linux x64** 产物，签名生成更新产物与 `latest.json`，并自动从 `docs/CHANGELOG.md` 提取对应版本说明作为 Release 正文。
- 发布前请在 `docs/CHANGELOG.md` 中补上 `## v<版本号>` 段落。

---

## 🗺️ 路线图

- frpc 配置文件的导入 / 导出与校验
- frps 服务端信息的集中管理与状态检测
- 日志持久化与导出
- 系统托盘 / 开机自启 / 关闭到托盘
- 主题跟随系统
- 更多语言与自定义背景

---

## 📚 参考链接

- [Tauri 2 文档](https://v2.tauri.app/)
- [Vue 3 文档](https://vuejs.org/)
- [Ant Design Vue 4](https://antdv.com/)
- [Tailwind CSS 4](https://tailwindcss.com/)
- [Vue I18n](https://vue-i18n.intlify.dev/)
- [VueUse](https://vueuse.org/)
- [frp 官方仓库](https://github.com/fatedier/frp)

---

## 📄 License

本项目基于 [Apache License 2.0](LICENSE) 开源。
