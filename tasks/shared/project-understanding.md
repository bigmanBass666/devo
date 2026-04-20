# 项目理解 — claw-code-rust 上游代码结构

> 供 Worker Agent 和 Planner 快速理解项目结构，避免盲目搜索。

## 仓库信息

| 项 | 值 |
|---|---|
| 名称 | claw-code-rust（Claw CR） |
| 语言 | Rust (edition 2024, rust-version 1.85) |
| 上游 | https://github.com/7df-lab/claw-code-rust |
| 你的 fork | https://github.com/bigmanBass666/claw-code-rust |
| 许可证 | MIT |
| 构建命令 | `cargo build --release` → 输出到 `target/release/clawcr` |
| 测试命令 | `cargo test --workspace` |
| 格式化 | `cargo fmt --all -- --check` |
| Lint | `cargo clippy --workspace --all-targets` |

## Workspace 结构（11 个 crate）

```
claw-code-rust/
├── crates/
│   ├── core/        ← 核心引擎：消息模型、会话状态、Agent 主循环
│   ├── server/      ← 服务端：传输层、运行时、会话管理、二进制入口
│   ├── client/      ← 客户端：stdio 通信
│   ├── protocol/    ← 协议定义：client/server 共享的类型系统
│   ├── provider/    ← 模型提供商：Anthropic / OpenAI 统一接口
│   ├── tools/       ← 工具系统：Tool trait、注册表、内置工具
│   ├── safety/      ← 安全层：权限控制
│   ├── tasks/       ← 任务抽象（ValveOS 协调文件也在此）
│   ├── mcp/         ← MCP（Model Context Protocol）支持
│   ├── tui/         ← 终端 UI（尚未实现）
│   └── utils/       ← 工具函数（尚未实现）
```

## 各 Crate 职责详解

### core（核心引擎）
- **职责**：消息模型、会话状态管理、Agent 主循环、配置加载、日志
- **关键模块**：
  - `config/` — 应用配置（app.rs, provider.rs, server.rs, skills.rs, safety.rs）
  - `context.rs` — 上下文管理
  - `session.rs` — 会话生命周期
  - `conversation/` — 对话记录
  - `query.rs` — 查询构建
  - `model_catalog.rs` / `model_preset.rs` — 模型目录和预设

### server（服务端）
- **职责**：运行时协议、连接管理、事件处理、turn 执行、持久化
- **关键模块**：
  - `main.rs` — 二进制入口
  - `bootstrap.rs` — 启动引导
  - `session.rs` — 会话管理
  - `execution.rs` / `turn.rs` — 执行引擎
  - `connection.rs` / `transport.ts` — 连接与传输
  - `persistence.rs` — 状态持久化
  - `runtime/skills.rs` — 运行时 skill 管理
  - `titles.rs` — 会话标题生成

### protocol（协议层）
- **职责**：client/server 共享的纯数据类型，零依赖业务逻辑
- **关键模块**：
  - `protocol.rs` — 主协议类型
  - `message.rs` / `role.rs` — 消息与角色
  - `conversation.rs` — 对话协议
  - `skill.rs` — Skill 定义
  - `thinking.rs` — 思考过程
  - `truncation.rs` — 截断策略
  - `approval.rs` — 审批机制

### provider（模型提供商）
- **职责**：统一的 LLM 调用接口，支持流式输出
- **关键模块**：
  - `anthropic/` — Claude API 实现
  - `openai/` — OpenAI 兼容 API（含 chat_completions, responses, streaming）
  - `provider.rs` — 统一 provider trait
  - `request.rs` — 请求构建
  - `text_normalization.rs` — 文本标准化

### tools（工具系统）
- **职责**：Agent 可用的操作工具（读文件、写文件、搜索、执行命令等）
- **关键模块**：
  - `registry.rs` — 工具注册表
  - `orchestrator.rs` — 工具编排
  - `runtime/` — 运行时（executor, assembly, builtins, shell_command）
  - 内置工具：read, write/edit, grep, glob, bash, file_upload, lsp, plan, question, skill, spec, apply_patch

### client（客户端）
- **职责**：通过 stdio 与 server 通信
- **关键模块**：`lib.rs`, `stdio.rs`

### safety（安全层）
- **职责**：权限控制和安全策略
- **关键模块**：`lib.rs`, `legacy_permissions.rs`

### mcp（MCP 支持）
- **职责**：Model Context Protocol 集成

## 常见修改模式

| 我想改... | 去哪找 |
|----------|--------|
| 配置项/新增配置 | `crates/core/src/config/` |
| 新增工具 | `crates/tools/src/` + 在 `runtime/builtins.rs` 注册 |
| 改协议格式 | `crates/protocol/src/` |
| 新增模型提供商 | `crates/provider/src/` 新建目录 |
| 改服务端行为 | `crates/server/src/`（execution.rs / turn.rs / session.rs） |
| 改 CLI 参数 | `crates/cli/src/` 或 `crates/server/src/bootstrap.rs` |
| 安全规则 | `crates/safety/src/` |
| 测试 | `crates/server/tests/`（端到端测试） |

## 编译与测试

```bash
# 完整编译
cargo build --release

# 运行所有测试
cargo test --workspace

# 格式检查
cargo fmt --all -- --check

# Lint 检查
cargo clippy --workspace --all-targets

# 启动 onboard 流程
cargo run --release -- onboard
```
