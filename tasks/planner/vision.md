# 项目愿景

本文件记录 Planner Agent 对项目的深入理解，作为决策的参考基准。

---

## 项目是什么

**claw-code-rust** — 一个开源的 coding agent，类比 Claude Code / Codex，用 Rust 构建。

核心目标：创建一个 provider-agnostic 的 AI 编程助手，支持 Claude、OpenAI、z.ai、Qwen、Deepseek 或本地模型。

---

## 核心特性

1. **完全开源** — 不绑定任何特定 provider
2. **多模型支持** — 可接入各种 LLM
3. **原生 LSP 支持** — 开箱即用
4. **TUI 支持** — 已实现终端界面
5. **客户端/服务器架构** — 核心可本地运行，支持远程控制

---

## 技术架构

### 分层结构
- Session（会话）→ Turn（回合）→ Item（条目）
- 核心层：conversation、model integration、context management
- 工具层：Tools、Safety、Context Management、MCP、Skills

### 目标 Crate 划分
| Crate | 职责 |
|-------|------|
| `clawcr-core` | Session、Turn、Item、Model、Context |
| `clawcr-tools` | 工具合约、注册、执行 |
| `clawcr-safety` | 沙箱、权限、审批 |
| `clawcr-mcp` | MCP 服务器连接 |
| `clawcr-server` | API 服务器、传输层 |
| `clawcr-cli` | 本地引导、TUI |
| `clawcr-utils` | 共享工具函数 |

---

## 当前阶段

项目处于 **早期开发阶段（Early-stage）**，尚未生产就绪。

根据 `progress.txt`：
- Iteration 1 已完成基础功能
- 有 PR #38, #39 待审核
- 主要工作：修复、改进、清理

---

## 质量标准

### 提交前检查
1. `cargo fmt --all -- --check` — 格式
2. `cargo clippy --workspace --all-targets` — 代码检查
3. `cargo test --workspace` — 测试通过

### 规范遵循
- 遵循 `CONTRIBUTING.md` 的贡献指南
- 先开 issue 讨论大改动
- 保持 PR 小而专注
- 明确描述改什么、为什么

---

## 成功指标

### 短期（当前迭代）
- 修复待审核 PR 中的问题
- 消除 clippy 警告
- 测试全部通过

### 中期
- 实现核心功能完整
- 文档完善
- 社区参与

### 长期
- 成为一个可用的开源 coding agent
- 获得社区贡献
- 与上游保持同步

---

## 决策参考

在做任务决策时，考虑：
1. **是否推动项目接近"可生产就绪"？**
2. **是否遵循开源社区规范？**
3. **是否考虑了多 provider 支持？**
4. **是否保持了代码质量？**

---

## 备注

本愿景文件应随项目发展而更新。
