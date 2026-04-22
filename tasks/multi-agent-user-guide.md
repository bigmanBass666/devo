# ValveOS 用户指南

> **ValveOS** — 面向 PR 规范化的多 Agent 协作系统。你作为用户控制"阀门"，决定何时唤醒哪个 Agent，系统自主完成代码开发、质量检查、PR 准备的全流程。

---

## 快速开始（3 步）

### 第一步：了解 Agent

| Agent | 一句话职责 |
|-------|-----------|
| **Planner** | 决策者——分析项目现状，决定"做什么" |
| **Coordinator** | 管理员——拆分任务，分配给 Worker |
| **Worker** | 工人——执行具体编码任务 |
| **PR Manager** | PR 管理员——提取干净改动，准备 PR |
| **Maintainer** | 分析师——收集日志，发现改进点 |
| **Housekeeper** | 清理工——清理已合并分支 |
| **COO** | 系统官——维护文档一致性，执行审计 |

### 第二步：唤醒 Agent

1. 打开新会话
2. 告诉 AI："你是 [Agent 名]，读取你的指令"
3. Agent 会自动读取 inbox 并开始工作
4. 完成后 Agent 会告诉你："请唤醒 [下一个 Agent]"

> 你只需要做一件事：**打开正确的会话**。其余由 Agent 自主完成。

### 第三步：查看状态

查看 `tasks/shared/agent-status.md` 了解当前进度：
- 各 Agent 的状态（空闲/工作中）
- 当前任务及进度
- 上次活动时间

---

## 架构概览

```
核心流水线（线性流转）：
  Planner → Coordinator → Worker → PR Manager

横切服务（独立触发）：
  Maintainer   — 数据分析后台
  Housekeeper  — 仓库清理后台
  COO          — 系统维护后台
```

完整元数据（Agent 清单/架构模型/功能索引）→ [SYSTEM-MANIFEST.md](./SYSTEM-MANIFEST.md)

### 核心概念速查

| 概念 | 含义 |
|------|------|
| **沉睡** | Agent 收到消息但未被人唤醒，无法执行 |
| **唤醒** | 你打开特定 Agent 的会话 |
| **睁眼** | 被唤醒的 Agent 主动读取自己的 inbox |
| **声音** | Agent 写入共享文件的消息 |
| **待机** | Agent 完成工作后标记状态，等待下次唤醒时断点续传 |

> 完整定义 → [ARCHITECTURE.md#核心概念](./ARCHITECTURE.md)

---

## 各 Agent 详情

| Agent | 类型 | 一句话职责 | 唤醒时机 |
|-------|------|-----------|----------|
| Planner | 核心流水线 | 分析现状，制定计划，决定下一步 | 项目启动 / 需要决策时 |
| Coordinator | 核心流水线 | 拆分任务，分配 Worker，协调冲突 | 收到 Planner 任务后 |
| Worker | 核心流水线 | 编写代码，运行测试，提交改动 | 收到 Coordinator 分配后 |
| PR Manager | 核心流水线 | 质量检查，生成 PR 描述 | Worker 完成任务后 |
| Maintainer | 横切服务 | 分析日志，发现瓶颈，提出改进 | 每3个任务/每24小时 |
| Housekeeper | 横切服务 | 清理已合并/过期分支 | PR 合并后/每24小时 |
| COO | 横切服务 | 文档审计，skill 优化，系统维护 | 文档改动后/收到改进数据时 |

---

## 单会话模式

**是什么**：在 `/spec` 模式下，用 sub-agent 替代 Worker 执行代码编写，在一个会话内完成更多工作。

**原理**：`/spec` 模式支持并行 sub-agent 编码，主 Agent 可直接 spawn sub-agent 替代 Worker。

**适用场景**：
- ✅ 系统维护
- ✅ 文档修改
- ✅ 小规模代码改动

**不适用场景**：
- ❌ 大型功能开发
- ❌ 需要独立工作目录隔离的场景

---

## 待机模式

Agent 完成工作后标记为"待机"，下次被唤醒时从断点续传。**不存在后台轮询**——AI 会话是一次性的。

| 概念 | 说明 |
|------|------|
| 待机 | Agent 在 agent-status.md 中标记为"待机"，不执行任何后台进程 |
| 唤醒 | 用户在新会话中说"唤醒 [Agent名]"，AI 读取 instructions + inbox + status，从断点续传 |
| 轮询 | 不存在。AI 会话没有后台轮询能力。 |

详细说明 → [cli-operations.md#待机模式](../docs/agent-rules/cli-operations.md#待机模式)

---

## 常见操作指引

### 系统重置
说 **"执行系统重置"** 即可触发。清空所有 inbox、状态文件、任务队列，恢复初始状态。
→ 详细说明：[cli-operations.md#系统重置](../docs/agent-rules/cli-operations.md#系统重置)

### 文档审计
COO 在每次文档改动后**自动执行审计**。也可手动触发 `valveos-audit` skill。
→ 审计日志：[audit-log.md](./coo/audit-log.md)

### 提交 PR
PR Manager 负责：
1. 从 Worker 分支提取干净改动
2. 创建基于 `upstream/main` 的 `feat/xxx` 分支
3. 执行质量检查（fmt/clippy/test/diff清洁度）
4. 生成 PR 描述供你审批
→ PR 质量铁律见 [AGENTS.md](../AGENTS.md)

### 查看状态
- **全局状态**：[agent-status.md](./shared/agent-status.md)
- **迭代进度**：[iteration-log.md](./shared/iteration-log.md)
- **各 Agent 日志**：`tasks/logs/*.log`

---

## 文件导航

| 文件 | 用途 |
|------|------|
| [AGENTS.md](../AGENTS.md) | 项目基础规则（安全铁律、提交纪律、社交边界、ValveOS 入口） |
| [valveos-protocol.md](../docs/agent-rules/valveos-protocol.md) | ValveOS 完整协议（按需激活，/valveos 触发后加载） |
| [SYSTEM-MANIFEST.md](./SYSTEM-MANIFEST.md) | 元数据唯一事实来源（**完整索引**） |
| [ARCHITECTURE.md](./ARCHITECTURE.md) | 完整架构文档（通信机制、分支策略等） |
| [agent-status.md](./shared/agent-status.md) | Agent 状态与任务追踪 |
| [iteration-log.md](./shared/iteration-log.md) | 迭代日志（断点续传） |
| [cli-operations.md](../docs/agent-rules/cli-operations.md) | CLI 操作参考（重置/通知/调试） |
| [git-workflow.md](../docs/agent-rules/git-workflow.md) | Git 工作流与上游协作 |
| [rust-conventions.md](../docs/agent-rules/rust-conventions.md) | Rust 编码与测试规范 |

> 📋 **完整文件注册表**（含运行时数据文件）→ [SYSTEM-MANIFEST.md#File Registry](./SYSTEM-MANIFEST.md#File Registry)

---

## FAQ / 注意事项

### Q1: 我需要做什么？
**大部分时间你只需要打开会话唤醒 Agent**。系统自主运转，你只在需要审批或介入时操作。

### Q2: Agent 之间怎么沟通？
通过 `tasks/shared/inbox/` 下的收件箱。Agent A 写完消息后会告诉你"请唤醒 Agent B"。

### Q3: PR 为什么天然干净？
Worker 使用 `git worktree` 在独立目录工作，分支基于 `upstream/main`（不含 AI 协调文件）。PR Manager 只 cherry-pick 相关 commit 到干净的 `feat/xxx` 分支。

### Q4: 可以同时运行多个 Worker 吗？
可以。Coordinator 通过文件锁 (`tasks/workers/locks/`) 防止冲突，多个 Worker 可并行处理不同文件的任务。

### Q5: 如何让系统自我改进？
Maintainer 定期分析日志发现问题 → 写入 COO inbox → COO 评估并实施改进。你也可以直接向 COO 或 Maintainer 发指令加速这个过程。

---

## 关键原则速查

- **你是阀门**：只决定唤醒谁，不传话、不指导细节
- **Agent 自主**：被唤醒后自己读指令、判断、执行
- **PR 要小而专注**：每个 PR 解决一个问题，超过 10 个文件要三思
- **立即提交**：每次更改后 `git add && git commit && git push`，不留未提交工作
- **安全第一**：push 前先 pull，冲突交给 Worker 处理

---

> 📖 **深入阅读**：完整架构文档 → [ARCHITECTURE.md](./ARCHITECTURE.md) | CLI 操作手册 → [cli-operations.md](../docs/agent-rules/cli-operations.md)
