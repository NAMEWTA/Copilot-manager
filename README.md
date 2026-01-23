# Antigravity Tools - Enhanced Edition 🚀

> 专业的 AI 账号管理与协议反代系统 (v3.3.33)
>
> 本项目基于 [lbjlaq/Antigravity-Manager](https://github.com/lbjlaq/Antigravity-Manager) 开发，新增 **GitHub Copilot 作为上游提供商** 支持

<div align="center">
  <img src="public/icon.png" alt="Antigravity Logo" width="120" height="120" style="border-radius: 24px; box-shadow: 0 10px 30px rgba(0,0,0,0.15);">

  <h3>您的个人高性能 AI 调度网关</h3>
  <p>在原有强大功能基础上，新增 <strong>GitHub Copilot 集成</strong>，扩展您的 AI 选择</p>

  <p>
    <a href="https://github.com/YOUR_USERNAME/Antigravity-Manager-3.3.33">
      <img src="https://img.shields.io/badge/Version-3.3.33-blue?style=flat-square" alt="Version">
    </a>
    <img src="https://img.shields.io/badge/Tauri-v2-orange?style=flat-square" alt="Tauri">
    <img src="https://img.shields.io/badge/Backend-Rust-red?style=flat-square" alt="Rust">
    <img src="https://img.shields.io/badge/Frontend-React-61DAFB?style=flat-square" alt="React">
    <img src="https://img.shields.io/badge/License-CC--BY--NC--SA--4.0-lightgrey?style=flat-square" alt="License">
  </p>

  <p>
    <a href="#-核心特性">核心特性</a> •
    <a href="#-copilot-集成详解">Copilot 集成</a> •
    <a href="#-与原项目对比">与原项目对比</a> •
    <a href="#-安装指南">安装指南</a> •
    <a href="#-快速接入">快速接入</a>
  </p>

  <p>
    <strong>简体中文</strong> |
    <a href="./README_EN.md">English</a>
  </p>
</div>

---

## 🌟 核心特性

本项目在保留 [Antigravity Manager](https://github.com/lbjlaq/Antigravity-Manager) 所有核心功能的基础上，新增了以下独特功能：

本版本在原项目 [lbjlaq/Antigravity-Manager](https://github.com/lbjlaq/Antigravity-Manager) 的基础上，新增了以下功能：

### 1. 🤖 GitHub Copilot 作为上游提供商

- **完整的 Copilot API 支持**: 将 GitHub Copilot 集成为可选的上游 AI 提供商
- **OAuth 设备授权流程**: 提供一键式设备授权登录，无需手动复制 token
- **自动 Token 刷新**: Token 即将过期时自动刷新，无需手动干预
- **智能模型映射**: 自动将客户端请求的模型映射到 Copilot 支持的模型
- **多种分发模式**: 支持 `off`、`exclusive`、`pooled`、`fallback` 四种分发策略

### 2. 🔐 Copilot OAuth 设备授权

- **一键授权**: 点击按钮即可启动 GitHub 设备授权流程
- **自动轮询**: 后台自动轮询 OAuth token，无需手动刷新
- **可视化反馈**: 实时显示授权状态和进度
- **错误处理**: 完善的错误提示和重试机制

### 3. 🔄 智能分发模式

Copilot 提供商支持四种分发模式：

- **Off (关闭)**: 不使用 Copilot
- **Exclusive (独占)**: 所有 Anthropic/OpenAI 协议请求都使用 Copilot
- **Pooled (池化)**: 将 Copilot 视为共享池中的一个额外槽位
- **Fallback (兜底)**: 仅在 Google 池不可用时使用 Copilot

### 4. 🎯 模型映射支持

Copilot 支持以下模型的自动映射：

| 原始模型 | 映射到 | 说明 |
|---------|--------|------|
| `claude-opus-4.5` | `claude-opus-4.5` | Anthropic 最强模型 |
| `claude-sonnet-4.5` | `claude-sonnet-4.5` | Claude Sonnet 4.5 |
| `gpt-4.1` | `gpt-4.1` | GPT-4.1 |
| `gpt-4o-mini` | `gpt-4o-mini` | GPT-4o Mini |
| `o1-mini` | `o1-mini` | OpenAI o1-mini |
| `o1-preview` | `o1-preview` | OpenAI o1-preview |

---

## 📊 与原项目对比

| 特性 | 原项目 | 本项目 |
|------|--------|--------|
| Google 账号池管理 | ✅ | ✅ |
| OpenAI 协议支持 | ✅ | ✅ |
| Anthropic 协议支持 | ✅ | ✅ |
| Gemini 协议支持 | ✅ | ✅ |
| z.ai 集成 | ✅ | ✅ |
| **GitHub Copilot 集成** | ❌ | ✅ **新增** |
| **Copilot OAuth 设备授权** | ❌ | ✅ **新增** |
| **自动 Token 刷新** | ❌ | ✅ **新增** |
| **Copilot 独占/池化/兜底模式** | ❌ | ✅ **新增** |

### 为什么选择本项目？

1. **扩展的 AI 选择**: 除了 Google/Anthropic/z.ai，现在还可以使用 GitHub Copilot
2. **一键授权**: 简单的 OAuth 设备授权流程，无需手动操作 token
3. **智能路由**: Copilot 可作为主提供商、补充提供商或备用提供商
4. **完全兼容**: 保留原项目的所有功能，无缝升级

---

## ⚠️ Copilot 集成详解

### 架构说明

```
客户端请求 → Antigravity Gateway → Copilot Provider → GitHub Copilot API
                  (调度器)            (Token 管理)       (api.githubcopilot.com)
```

### 认证流程

本版本支持两种 Copilot 认证方式：

1. **GitHub OAuth Token (推荐)**: 通过设备授权流程获取
2. **GitHub Personal Access Token**: 手动输入 classic PAT
3. **直接使用 Copilot Token**: 从浏览器 DevTools 复制的 token

### 自动 Token 刷新

- Token 过期前 60 秒自动刷新
- 支持并发请求的 token 缓存
- 失败自动重试机制

### 设备授权流程

```
1. 点击 "OAuth 登录" 按钮
   ↓
2. 获取设备码和验证链接
   ↓
3. 用户在浏览器中完成授权
   ↓
4. 后台轮询获取 OAuth token
   ↓
5. 使用 OAuth token 获取 Copilot token
   ↓
6. 自动保存并启用 Copilot
```

---

## 📋 配置说明

### Copilot 配置项

| 配置项 | 类型 | 说明 |
|-------|------|------|
| `enabled` | boolean | 是否启用 Copilot 提供商 |
| `github_token` | string | GitHub Token 或 Copilot Token |
| `dispatch_mode` | enum | 分发模式：off/exclusive/pooled/fallback |
| `default_model` | string | 默认模型（如 `claude-opus-4.5`） |

### 配置文件位置

- **Windows**: `C:\Users\{username}\.antigravity_tools\gui_config.json`
- **macOS**: `~/.antigravity_tools/gui_config.json`
- **Linux**: `~/.antigravity_tools/gui_config.json`

### 示例配置

```json
{
  "proxy": {
    "copilot": {
      "enabled": true,
      "github_token": "your-copilot-token-here",
      "dispatch_mode": "exclusive",
      "default_model": "claude-opus-4.5"
    }
  }
}
```

---

## 🚀 安装指南

### 方式 1: 从源码构建

```bash
# 克隆仓库
git clone https://github.com/YOUR_USERNAME/Antigravity-Manager-3.3.33.git
cd Antigravity-Manager-3.3.33

# 安装依赖
npm install

# 启动开发模式
npm run tauri dev

# 构建生产版本
npm run tauri build
```

### 方式 2: 下载预编译版本

前往 [GitHub Releases](../../releases) 下载对应系统的包：

- **Windows**: `.msi` 或便携版 `.zip`
- **macOS**: `.dmg` (支持 Apple Silicon & Intel)
- **Linux**: `.deb` 或 `AppImage`

---

## 🔌 快速接入

### 接入 Claude Code CLI

1. 启动 Antigravity，在"API 反代"页面启用 Copilot
2. 选择分发模式为 `exclusive` 或 `pooled`
3. 在终端执行：

```bash
export ANTHROPIC_API_KEY="sk-antigravity"
export ANTHROPIC_BASE_URL="http://127.0.0.1:8045"
claude
```

### 接入 OpenAI 协议应用

```python
import openai

client = openai.OpenAI(
    api_key="sk-antigravity",
    base_url="http://127.0.0.1:8045/v1"
)

response = client.chat.completions.create(
    model="claude-opus-4.5",
    messages=[{"role": "user", "content": "你好，请自我介绍"}]
)
print(response.choices[0].message.content)
```

### Copilot 模式配置

1. 打开"API 反代"页面
2. 找到"Copilot Provider"卡片
3. 点击"开始 OAuth"按钮完成授权
4. 选择分发模式：
   - **独占模式**: 所有请求都通过 Copilot
   - **池化模式**: Copilot 作为额外账号池
   - **兜底模式**: 仅在 Google 账号不可用时使用

---

## 🏗️ 技术实现

### 核心文件

| 文件 | 说明 |
|------|------|
| `src-tauri/src/proxy/providers/copilot.rs` | Copilot 提供商核心实现 |
| `src-tauri/src/proxy/handlers/claude.rs` | Claude 协议中的 Copilot 分发逻辑 |
| `src-tauri/src/proxy/handlers/openai.rs` | OpenAI 协议中的 Copilot 分发逻辑 |
| `src-tauri/src/proxy/config.rs` | Copilot 配置结构定义 |
| `src/pages/ApiProxy.tsx` | Copilot 配置前端界面 |
| `src/locales/zh.json` | 中文翻译 |
| `src/locales/en.json` | 英文翻译 |

### API 端点

Copilot 提供商使用以下 GitHub 端点：

- `GET https://api.github.com/copilot_internal/v2/token` - 获取 Copilot token
- `POST https://github.com/login/device/code` - 设备授权
- `POST https://github.com/login/oauth/access_token` - OAuth token
- `https://api.githubcopilot.com` - Copilot API 基础 URL

---

## ⚠️ 注意事项

### 使用风险

- Copilot API 为非官方接口，使用风险自负
- GitHub 可能随时更改接口或封禁账号
- 建议仅用于个人学习和测试目的
- 请遵守 GitHub 的服务条款

### 限制

- Copilot 仅支持 OpenAI 协议，不支持 Anthropic 原生 `/v1/messages` 端点
- 模型映射有限，不支持所有 Claude/OpenAI 模型
- Token 有效期较短（通常 1-2 小时），但会自动刷新

---

## 🤝 贡献

欢迎提交 Issue 和 Pull Request！

### 开发命令

```bash
# 前端开发
npm run dev

# Rust 后端开发
cd src-tauri
cargo build
cargo test
cargo clippy

# 完整开发
npm run tauri dev
```

---

## 📝 继承的核心功能

除了新增的 Copilot 集成，本项目完整保留了原项目的所有强大功能：

### 账号管理
- ✅ **智能账号仪表盘** - 全局实时监控所有账号健康状况
- ✅ **OAuth 2.0 授权** - 自动/手动授权，支持多种导入方式
- ✅ **多维度导入** - 单条录入、JSON 批量导入、V1 数据库热迁移

### 协议转换
- ✅ **OpenAI 格式** - `/v1/chat/completions` 端点，兼容 99% 的 AI 应用
- ✅ **Anthropic 格式** - 原生 `/v1/messages` 接口，支持 Claude Code CLI
- ✅ **Gemini 格式** - Google 官方 SDK 直接调用支持

### 智能路由
- ✅ **模型映射** - 正则表达式级自定义映射
- ✅ **智能分级路由** - 根据账号类型和配额重置频率自动排序
- ✅ **后台任务静默降级** - 自动识别并重定向后台请求

### 高级功能
- ✅ **多模态支持** - Imagen 3 高级绘图，100MB Payload 支持
- ✅ **配额保护** - 模型级配额监控与自动保护
- ✅ **智能调度** - 自动重试与静默轮换，确保业务不中断

> 📖 **了解更多**: 查看原项目文档了解更多功能详情

---

## 🛣️ 项目路线图

### 已完成 ✅
- [x] GitHub Copilot 基础集成
- [x] OAuth 设备授权流程
- [x] 自动 Token 刷新机制
- [x] 多种分发模式支持
- [x] 模型映射功能

### 计划中 🚧
- [ ] Copilot Token 浏览器扩展自动提取
- [ ] 支持更多 Copilot 模型
- [ ] 性能优化与错误处理增强
- [ ] 详细的使用文档和示例

---

## 🤝 贡献指南

欢迎提交 Issue 和 Pull Request！

### 开发环境设置

```bash
# 1. 克隆仓库
git clone https://github.com/YOUR_USERNAME/Antigravity-Manager-3.3.33.git
cd Antigravity-Manager-3.3.33

# 2. 安装依赖
npm install

# 3. 启动开发模式
npm run tauri dev
```

### 代码规范

- Rust 代码请遵循 `cargo clippy` 建议
- 前端代码使用 ESLint 和 Prettier
- 提交前请确保测试通过：`npm run tauri test`

---

## 📜 许可证

本项目采用 **CC-BY-NC-SA-4.0** (知识共享署名-非商业性使用-相同方式共享) 许可证。

**重要说明**:
- 本项目基于 [lbjlaq/Antigravity-Manager](https://github.com/lbjlaq/Antigravity-Manager) (v3.3.33) 开发
- 新增的 Copilot 集成代码同样遵循 CC-BY-NC-SA-4.0 许可证
- 使用时请保留原作者署名
- **禁止商业用途**

---

## 📜 许可证

CC-BY-NC-SA-4.0 (知识共享署名-非商业性使用-相同方式共享)

**原项目**: Copyright © 2024-2026 [lbjlaq](https://github.com/lbjlaq)

---

## 🙏 致谢

### 原项目
- **[lbjlaq](https://github.com/lbjlaq)** - Antigravity Manager 原项目作者，构建了强大的基础架构

### Copilot 相关参考
- **[ericc-ch/copilot-api](https://github.com/ericc-ch/copilot-api)** - Copilot API 参考实现
- **[Alorse/copilot-to-api](https://github.com/Alorse/copilot-to-api)** - Copilot 协议转换参考

### 技术栈
本项目离不开以下优秀开源项目的支持：
- [Tauri](https://tauri.app/) - 跨平台桌面应用框架
- [React](https://react.dev/) - UI 框架
- [Axum](https://github.com/tokio-rs/axum) - Rust Web 框架
- [Tailwind CSS](https://tailwindcss.com/) - CSS 框架

---

## 📮 联系方式

- **GitHub Issues**: [提交问题](../../issues)
- **GitHub Discussions**: [参与讨论](../../discussions)

---

<div align="center">

### ⭐ 如果这个项目对你有帮助，请给个 Star！

### 🔗 相关链接

- [原项目](https://github.com/lbjlaq/Antigravity-Manager)
- [更新日志](CHANGELOG.md)
- [贡献指南](CONTRIBUTING.md)

---

**Made with ❤️ by [King]**

*Powered by [Tauri](https://tauri.app/) + [React](https://react.dev/)*

</div>
