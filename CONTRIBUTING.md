# 贡献指南

感谢您考虑为 Antigravity Tools Enhanced Edition 做出贡献！

## 🤝 如何贡献

### 报告 Bug

1. 在 [Issues](../../issues) 中搜索，确保问题未被报告
2. 创建新 Issue 时，请使用 Bug Report 模板
3. 提供详细的信息：
   - 操作系统和版本
   - 应用版本号
   - 复现步骤
   - 期望行为 vs 实际行为
   - 相关日志或截图

### 提出新功能

1. 先在 [Discussions](../../discussions) 中讨论您的想法
2. 创建 Feature Request Issue
3. 说明功能的使用场景和预期效果

### 提交代码

#### 1. Fork 仓库

如果您还没有 fork，请先 fork 本仓库。

#### 2. 克隆并设置开发环境

```bash
# 克隆您的 fork
git clone https://github.com/YOUR_USERNAME/Antigravity-Manager-3.3.33.git
cd Antigravity-Manager-3.3.33

# 添加上游仓库
git remote add upstream https://github.com/ORIGINAL_OWNER/Antigravity-Manager-3.3.33.git

# 安装依赖
npm install
```

#### 3. 创建功能分支

```bash
git checkout -b feature/your-feature-name
# 或者修复 bug
git checkout -b fix/bug-description
```

#### 4. 进行开发

```bash
# 启动开发模式
npm run tauri dev

# 运行测试
npm run tauri test

# Rust 代码检查
cd src-tauri
cargo clippy
cargo fmt
```

#### 5. 提交更改

```bash
git add .
git commit -m "feat: add Copilot token auto-refresh"
```

**提交消息格式**:
- `feat:` - 新功能
- `fix:` - Bug 修复
- `docs:` - 文档更新
- `style:` - 代码格式调整
- `refactor:` - 重构
- `test:` - 测试相关
- `chore:` - 构建/工具相关

#### 6. 推送并创建 Pull Request

```bash
git push origin feature/your-feature-name
```

然后在 GitHub 上创建 Pull Request。

## 📋 代码规范

### Rust 代码

- 使用 `cargo fmt` 格式化代码
- 通过 `cargo clippy` 检查
- 添加必要的文档注释
- 编写单元测试

### TypeScript/React 代码

- 使用 ESLint 和 Prettier
- 遵循 React Hooks 最佳实践
- 组件使用函数式组件
- 添加必要的注释

### UI/UX

- 保持与现有 UI 风格一致
- 支持深色/浅色主题
- 添加国际化翻译（zh.json, en.json）
- 确保响应式布局

## 🧪 测试

在提交 PR 前，请确保：

1. **功能测试**
   - [ ] 应用能正常启动
   - [ ] 新功能在主要平台上都能正常工作
   - [ ] 没有控制台错误

2. **Rust 测试**
   ```bash
   cd src-tauri
   cargo test
   ```

3. **构建测试**
   ```bash
   npm run tauri build
   ```

## 📝 文档

如果您的更改影响了用户使用，请更新相关文档：

- README.md
- docs/ 目录下的相关文档
- 国际化翻译文件

## 🎯 Copilot 集成开发指南

如果您要为 Copilot 集成添加新功能：

### 相关文件

- `src-tauri/src/proxy/providers/copilot.rs` - Copilot 提供商核心逻辑
- `src-tauri/src/proxy/handlers/claude.rs` - Claude 协议中的 Copilot 分发
- `src-tauri/src/proxy/handlers/openai.rs` - OpenAI 协议中的 Copilot 分发
- `src/pages/ApiProxy.tsx` - Copilot 配置 UI
- `src/locales/zh.json` - 中文翻译
- `src/locales/en.json` - 英文翻译

### 测试 Copilot 功能

1. 确保 GitHub Token 有效
2. 测试 OAuth 设备授权流程
3. 验证 Token 自动刷新
4. 测试不同分发模式
5. 检查模型映射是否正确

## 📧 联系方式

如有任何问题，请通过以下方式联系：

- GitHub Issues
- GitHub Discussions

## 📜 许可证

通过贡献代码，您同意您的贡献将采用与项目相同的 **CC-BY-NC-SA-4.0** 许可证。

---

再次感谢您的贡献！🎉
