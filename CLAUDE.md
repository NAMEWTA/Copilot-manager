# CLAUDE.md

本文件为 Claude Code (claude.ai/code) 提供此代码库的工作指南。

## 项目概述

Antigravity Tools 是一个专业的 AI 账号管理与协议反代系统 (v3.3.33)。它作为本地 AI 中转站，提供以下功能：
- 管理多个 AI 服务账号 (Google/Anthropic)
- 将 Web 端 Session 转换为标准化 API 接口
- 提供不同 AI 厂商间的协议转换 (OpenAI/Anthropic/Gemini)
- 智能请求调度和代理服务

## 技术栈

- **前端**: React 19.1.0 + TypeScript, Vite 构建
- **UI**: Tailwind CSS + DaisyUI + Framer Motion
- **状态管理**: Zustand
- **路由**: React Router DOM v7
- **桌面应用**: Tauri v2 (Rust 后端)
- **国际化**: i18next

## 开发命令

```bash
# 前端开发
npm run dev              # 启动 Vite 开发服务器 (运行在 http://localhost:1420)
npm run build           # 构建前端 (tsc && vite build)
npm run preview         # 预览生产构建

# Tauri (桌面应用)
npm run tauri dev       # 启动 Tauri 开发模式 (构建 Rust + 前端)
npm run tauri build     # 构建发布版本

# 仅 Rust 后端
cd src-tauri
cargo build             # Debug 构建
cargo build --release   # Release 构建
cargo test              # 运行测试
cargo clippy            # 代码检查
```

## 架构

### 前端结构
- `src/App.tsx` - 主入口，路由设置，托盘事件监听
- `src/pages/` - 路由页面: Dashboard, Accounts, ApiProxy, Monitor, Settings
- `src/components/` - UI 组件 (layout/, common/)
- `src/stores/` - Zustand 状态存储 (useAccountStore, useConfigStore, networkMonitorStore)
- `src/types/` - TypeScript 类型定义
- `src/utils/` - 工具函数
- `src/locales/` - i18next 翻译文件 (en.json, zh.json)

### 后端结构 (Rust)

**核心模块** (`src-tauri/src/`):
- `lib.rs` - 主入口，Tauri 初始化，插件加载，命令注册
- `commands/` - Tauri 命令处理器 (前后端通信桥)
- `modules/` - 核心模块: account (账号), config (配置), scheduler (调度器), tray (托盘), http_api
- `models/` - 数据模型
- `proxy/` - API 反代服务 (见下方)

**反代模块** (`src-tauri/src/proxy/`):
基于 Axum 的 HTTP 服务器，处理 API 协议转换：

```
src-tauri/src/proxy/
├── config.rs           # ProxyConfig, ProxyAuthMode, ZaiConfig
├── server.rs           # AxumServer - 主 HTTP 服务器
├── security.rs         # ProxySecurityConfig, 鉴权策略解析
├── handlers/           # API 端点处理器
│   ├── claude.rs       # /v1/messages, Anthropic 协议
│   ├── openai.rs       # /v1/chat/completions, OpenAI 协议
│   ├── gemini.rs       # Gemini 协议端点
│   ├── mcp.rs          # MCP 端点 (z.ai 集成)
│   └── audio.rs        # 音频输入端点
├── mappers/            # 协议转换器 (Claude<->Gemini, OpenAI<->Gemini)
├── middleware/         # Axum 中间件 (auth, cors, logging, monitor)
├── providers/          # 额外上游提供商 (z.ai)
├── upstream/           # 上游请求 HTTP 客户端
├── token_manager.rs    # 账号令牌管理
├── session_manager.rs  # 会话指纹 (用于粘性路由)
├── sticky_config.rs    # 粘性调度配置
└── monitor.rs          # 请求/响应监控和日志
```

### 请求流程

1. **客户端请求** → Axum 服务器 (端口配置在 `proxy.port`)
2. **中间件层** → 鉴权检查、CORS、日志
3. **处理器** → 路由到对应的 handler (claude/openai/gemini)
4. **分发决策** → 选择上游 (z.ai 或 Google 账号池)
5. **映射器** → 转换请求格式 (如 Claude → Gemini)
6. **上游** → 发送到提供商 API
7. **响应映射器** → 转换响应回客户端格式
8. **监控** → 记录请求/响应

## 核心功能与配置

### 代理模式
- `proxy.auth_mode`: `off`, `strict`, `all_except_health`, `auto`
- `proxy.allow_lan_access`: 启用局域网访问
- `proxy.api_key`: 启用鉴权时必需

### 上游分发
- **z.ai 提供商** (`proxy.zai.*`): Anthropic 协议的可选上游
  - `dispatch_mode`: `off`, `exclusive`, `pooled`, `fallback`
  - 模型映射: claude-* 模型 → glm-* 模型
- **Google 账号池**: 使用 OAuth 令牌的主要上游

### 模型映射
位于 `src-tauri/src/proxy/common/model_mapping.rs`
- 支持基于正则的模型 ID 映射
- 示例: 将所有 `gpt-4*` 请求映射到 `gemini-3-pro-high`

### 会话管理
- `session_manager.rs`: 用于粘性路由的指纹识别
- 确保来自同一客户端的连续请求使用相同的上游账号

## 重要实现细节

### 热更新
配置变更 (鉴权、z.ai、模型映射) 无需重启即可生效:
- `save_config` 命令触发 `axum_server.update_security()` 和 `axum_server.update_zai()`

### 错误处理
- 遇到 429/401 时自动重试并轮换账号
- OAuth `invalid_grant` 错误时自动标记账号无效
- 详情见 `docs/proxy/accounts.md` 账号生命周期

### MCP 集成
- z.ai MCP Search/Reader: 反代端点 `/mcp/web_search_prime/mcp`, `/mcp/web_reader/mcp`
- Vision MCP: 内置服务器 `/mcp/zai-mcp-server/mcp`
- 详情见 `docs/zai/` 实现文档

## 测试

运行 Rust 测试:
```bash
cd src-tauri
cargo test
```

手动测试示例:
1. 启用代理鉴权并启动代理
2. `GET http://127.0.0.1:<port>/healthz` - 健康检查
3. `POST http://127.0.0.1:<port>/v1/messages` - 带 auth header 的 Anthropic 协议请求

## 文档

- `docs/README.md` - 开发者文档索引
- `docs/proxy/auth.md` - 鉴权模式和实现
- `docs/proxy/accounts.md` - 账号生命周期和自动禁用
- `docs/zai/` - z.ai 提供商和 MCP 集成文档

## 许可证

CC-BY-NC-SA-4.0 (知识共享署名-非商业性使用-相同方式共享)
