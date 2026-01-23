# 更新日志 (Changelog)

本项目的所有重要更改都将记录在此文件中。

格式基于 [Keep a Changelog](https://keepachangelog.com/zh-CN/1.0.0/)，
版本号遵循 [语义化版本](https://semver.org/lang/zh-CN/)。

## [Unreleased]

### 计划中
- Copilot Token 浏览器扩展自动提取
- 支持更多 Copilot 模型
- 性能优化与错误处理增强

## [3.3.33-enhanced] - 2026-01-23

### 新增 ✨
- **GitHub Copilot 集成**: 将 GitHub Copilot 添加为上游 AI 提供商
- **OAuth 设备授权**: 一键式 GitHub 设备授权流程，无需手动复制 token
- **自动 Token 刷新**: Token 即将过期时自动刷新，确保服务不中断
- **多种分发模式**: 支持 `off`、`exclusive`、`pooled`、`fallback` 四种分发策略
- **智能模型映射**: 自动将客户端请求映射到 Copilot 支持的模型
- **Copilot 配置界面**: 完整的 UI 支持，包含授权、配置和状态显示

### 改进 🚀
- 优化了 Copilot token 的缓存机制
- 增强了错误处理和重试逻辑
- 完善了前端 UI 的用户反馈

### 文档 📝
- 添加了详细的 Copilot 集成文档
- 提供了快速接入指南
- 添加了与原项目的对比说明

### 依赖
- 基于 [lbjlaq/Antigravity-Manager](https://github.com/lbjlaq/Antigravity-Manager) v3.3.33

## [原项目 v3.3.33] - 2026-01-15

详见原项目 [CHANGELOG](https://github.com/lbjlaq/Antigravity-Manager/blob/main/CHANGELOG.md)

---

## 版本说明

- **[Unreleased]**: 计划中但尚未发布的功能
- **[3.3.33-enhanced]**: 当前版本（基于原项目 v3.3.33 的增强版）
- **[原项目 v3.3.33]**: 基础版本
