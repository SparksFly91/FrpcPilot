# Frpilot

> 一个基于 **Tauri 2 + Vue 3 + TypeScript** 的 [frp](https://github.com/fatedier/frp) 可视化客户端，帮助用户快速配置 frpc 版本、服务器与隧道。

Frpilot 目前处于早期开发阶段，已完成应用骨架（自定义标题栏、多语言、明暗主题、磨砂玻璃布局）。

---

## ✨ 功能特性

### 已实现

- **i18n 国际化**：内置简体中文 / English，语言选择持久化，Ant Design Vue 组件库语言包联动
- **自定义标题栏**：按操作系统自动切换
  - Windows：左侧 `Frpilot` 标题，右侧「语言 / 明暗 / 设置 / 最小化 / 关闭」
  - macOS：左侧红绿灯（关闭 / 最小化 / 最大化），右侧同上一组按钮
- **明暗主题**：基于 `@vueuse/core` 的 `useDark`，Tailwind `dark:` 变体 + Ant Design Vue 暗色算法联动，选择持久化
- **悬浮磨砂玻璃布局**：侧边栏（Aside）与内容区（Content）均为圆角 + `backdrop-filter` 毛玻璃面板
- **侧边栏菜单**：仪表盘 / 服务器 / 隧道列表 / Frpc 下载
- **底部系统信息**：CPU 占用、内存占用（百分比 + 进度条）、运行状态指示点
  - 🟢 运行正常 · 🔴 运行异常 · 🟡 未启动

### 规划中

- frpc 版本管理与下载
- 服务器（frps）连接配置管理
- 隧道配置的增删改查与启停
- 配置文件的导入 / 导出与校验

---

## 🧱 技术栈

| 分类       | 选型                                                             |
| ---------- | ---------------------------------------------------------------- |
| 桌面框架   | [Tauri 2](https://v2.tauri.app/)（Rust 后端）                     |
| 前端框架   | Vue 3（`<script setup>`）+ TypeScript                            |
| 构建工具   | Vite 8                                                           |
| UI 组件库  | Ant Design Vue 4 + `@ant-design/icons-vue`                       |
| 样式       | Tailwind CSS 4（原子化）+ SCSS                                   |
| 状态管理   | Pinia                                                            |
| 路由       | Vue Router                                                       |
| 国际化     | Vue I18n 11                                                       |
| 工具库     | `@vueuse/core`                                                   |
| 自动化     | `unplugin-auto-import`、`unplugin-vue-components`（Antd 按需自动导入） |

---

## 📦 环境要求

- **Node.js** ≥ 18（推荐 20 / 22 LTS）
- **pnpm** ≥ 9（项目使用 pnpm，含 `pnpm-lock.yaml`）
- **Rust** 稳定版工具链（`rustup`）
- **系统依赖**：请参考 [Tauri 前置依赖](https://v2.tauri.app/start/prerequisites/)（Windows 需 WebView2 与 MSVC 构建工具；macOS 需 Xcode Command Line Tools；Linux 需 `webkit2gtk` 等）

---

## 🚀 快速开始

```bash
# 1. 安装依赖
pnpm install

# 2. 仅启动前端（浏览器调试，窗口控制类操作会安全空转）
pnpm dev

# 3. 启动完整桌面应用（推荐，含 Rust 后端）
pnpm tauri dev
```

### 常用脚本

| 命令             | 说明                                                     |
| ---------------- | -------------------------------------------------------- |
| `pnpm dev`       | 启动 Vite 开发服务器（端口 `1420`）                       |
| `pnpm tauri dev` | 启动 Tauri 桌面开发环境（自动拉起前端）                   |
| `pnpm build`     | 类型检查（`vue-tsc --noEmit`）+ 构建前端产物到 `dist/`    |
| `pnpm preview`   | 预览前端构建产物                                         |
| `pnpm tauri build` | 打包生成各平台安装包                                   |

> ⚠️ `pnpm dev` 的端口固定为 `1420` 且 `strictPort: true`，占用时会直接报错退出。

---

## 📁 项目结构

```
Frpilot/
├── src/
│   ├── api/                     # 接口封装（预留）
│   ├── assets/
│   │   └── main.css             # Tailwind 入口 + 全局样式（.glass-panel / dark 变体）
│   ├── components/              # 通用组件
│   │   ├── TitleBarActions.vue  # 标题栏右侧按钮组（语言/主题/设置/最小化/关闭）
│   │   ├── WindowsTitleBar.vue  # Windows 自定义标题栏
│   │   └── MacTitleBar.vue      # macOS 自定义标题栏（红绿灯）
│   ├── composables/
│   │   ├── usePlatform.ts       # 操作系统类型判断
│   │   └── useSystemStats.ts    # CPU / 内存占用采集
│   ├── i18n/
│   │   ├── index.ts             # createI18n 初始化
│   │   └── locales/
│   │       ├── zh-CN.json       # 中文语言包
│   │       └── en-US.json       # 英文语言包
│   ├── layout/
│   │   ├── Layout.vue           # 整体布局（背景 + Header + Aside + Content）
│   │   ├── aside/Aside.vue      # 侧边栏（菜单 + 系统信息）
│   │   └── header/Header.vue    # 顶部栏（按系统加载 TitleBar）
│   ├── router/router.ts         # 路由表
│   ├── stores/app.ts            # 全局状态：语言 / 主题 / 运行状态
│   ├── types/                   # 自动生成的类型声明
│   ├── utils/window.ts          # Tauri 窗口控制封装
│   ├── views/                   # 页面（仪表盘 / 服务器 / 隧道 / 下载）
│   ├── App.vue                  # ConfigProvider（语言 + 主题）
│   └── main.ts                  # 应用入口
├── src-tauri/                   # Rust 后端
│   ├── src/lib.rs               # 命令注册入口
│   ├── capabilities/default.json# 权限声明（窗口控制等）
│   └── tauri.conf.json          # Tauri 配置（无边框窗口 1100×720）
├── docs/
├── index.html
├── vite.config.ts               # 别名 / 自动导入 / Tailwind 插件
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
2. 在 `src/router/router.ts` 的 `Layout` 子路由中注册（路径需以 `/` 开头，与菜单 `key` 一致）。
3. 在 `src/layout/aside/Aside.vue` 的 `menuItems` 中追加一项（`key` = 路由路径）。
4. 在语言包中补充对应 `menu.*` 文案。

### 明暗主题

- 主题状态在 `src/stores/app.ts`（`isDark` / `toggleDark`），由 `useDark` 在 `<html>` 上切换 `.dark` 类名并持久化。
- Tailwind 使用 class 策略：在 `src/assets/main.css` 中已声明 `@custom-variant dark`。
- Ant Design Vue 主题在 `src/App.vue` 中通过 `a-config-provider` 的 `theme.algorithm` 联动。

### 玻璃拟态样式

`src/assets/main.css` 中的 `.glass-panel` 为通用毛玻璃面板，直接给元素加 `class="glass-panel"` 即可（已含暗色适配）。

### 接入真实的 CPU / 内存占用

`src/composables/useSystemStats.ts` 会优先调用 Tauri 命令 `get_system_info`，在命令不存在时回退到**模拟数据**。接入真实数据只需在 Rust 端实现该命令：

```rust
// src-tauri/src/lib.rs
#[tauri::command]
fn get_system_info() -> serde_json::Value {
    // 返回 { "cpu": f64, "memory": f64 }，单位为百分比
    serde_json::json!({ "cpu": 12.0, "memory": 45.0 })
}

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![get_system_info])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

前端会自动切换为真实数据（`isMock` 会变为 `false`）。

### 运行状态指示点

运行状态由 `src/stores/app.ts` 的 `runStatus` 控制，取值：

```ts
type RunStatus = 'running' | 'error' | 'stopped' // 🟢 | 🔴 | 🟡
```

后续接入 frpc 进程状态时，更新该字段即可。

### 窗口控制与权限

- 前端封装：`src/utils/window.ts`（`minimizeWindow` / `toggleMaximizeWindow` / `closeWindow` / `startDragging`），在非 Tauri 环境（浏览器）下自动空转。
- 自定义标题栏依赖无边框窗口（`src-tauri/tauri.conf.json` 中 `decorations: false`），拖拽区域通过 `data-tauri-drag-region` 实现。
- 相关权限在 `src-tauri/capabilities/default.json` 中声明（`core:window:allow-minimize`、`allow-close`、`allow-toggle-maximize`、`allow-start-dragging` 等）。**新增窗口 API 调用时需同步补充对应权限**，否则会报 `not allowed` 错误。

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

License 待定（待补充）。
