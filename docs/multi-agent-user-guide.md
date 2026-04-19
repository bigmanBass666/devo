# 多 Agent 协作系统 — 用户使用指南

> 让多个 AI 同时为你工作，自主协作、自动管理。

---

## 🎯 这个系统是什么？

一个**七层 AI 协作系统**，让多个 AI Agent 自动分工合作，完成项目任务并提交干净的 PR。

**你只需要做旁观者，必要时介入。**

---

## 🏗️ 七层架构

```
┌─────────────────────────────────────────────────────────┐
│                     你（最高领导人）                      │
│              旁观者，必要时介入审批                        │
└─────────────────────────────────────────────────────────┘
                           ↓
┌─────────────────────────────────────────────────────────┐
│  Planner（决策者）— 决定做什么                           │
│  观察项目、分析需求、制定计划                             │
└─────────────────────────────────────────────────────────┘
                           ↓
┌─────────────────────────────────────────────────────────┐
│  Coordinator（管理员）— 协调怎么做                      │
│  分配任务、管理冲突、分配 Worker                         │
└─────────────────────────────────────────────────────────┘
                           ↓
┌─────────────────────────────────────────────────────────┐
│  Worker（工人）— 具体执行                               │
│  编写代码、测试、提交                                   │
└─────────────────────────────────────────────────────────┘
                           ↓
┌─────────────────────────────────────────────────────────┐
│  PR Manager（PR 管理员）— 产出干净 PR                   │
│  提取代码、质量检查、准备 PR                            │
└─────────────────────────────────────────────────────────┘
                           ↓
┌─────────────────────────────────────────────────────────┐
│  Maintainer（维护者）— 发现问题                          │
│  分析日志、发现趋势、写入 COO inbox                      │
└─────────────────────────────────────────────────────────┘
                           ↓
┌─────────────────────────────────────────────────────────┐
│  Housekeeper（仓库守护）— 清理分支                       │
│  自动删除已合并的分支、保持仓库整洁                      │
└─────────────────────────────────────────────────────────┘
                           ↓
┌─────────────────────────────────────────────────────────┐
│  COO（首席系统官）— 系统维护与改进                       │
│  文档维护、一致性审计、skill 优化                        │
└─────────────────────────────────────────────────────────┘
```

---

## 🚀 快速开始

### 方式一：开一个"总指挥"会话

在 Trae IDE 新开一个会话，告诉它：

> "你是总指挥。请读取 `tasks/ARCHITECTURE.md` 和 `AGENTS.md`，然后开始协调项目工作。"

系统会自动：
1. Planner 分析项目状态
2. Coordinator 分配任务
3. Worker 执行
4. PR Manager 准备 PR
5. 你审批

### 方式二：让 Planner 自主观察

> "请分析当前项目状态，制定下一步计划。"

---

## 🤖 多会话并行模式（推荐）

由于 Trae 单个会话没有并行功能，需要**开多个会话**实现真正并行。

### 会话分配

| 会话 | 角色 | 开场白 |
|------|------|--------|
| 会话 1 | Planner（决策者） | "你是 **Planner Agent**。请读取 `tasks/ARCHITECTURE.md` 和 `tasks/planner/instructions.md`，然后检查你的 inbox（`tasks/shared/inbox/planner.md`）并开始观察项目状态。" |
| 会话 2 | Coordinator（管理员） | "你是 **Coordinator Agent**。请读取 `tasks/ARCHITECTURE.md` 和 `tasks/coordinator/instructions.md`，然后检查你的 inbox 并等待 Planner 下发任务。" |
| 会话 3 | Worker-001（工人） | "你是 **Worker-001 Agent**。请读取 `tasks/ARCHITECTURE.md` 和 `tasks/workers/instructions.md`，然后检查你的 inbox 并等待任务分配。" |
| 会话 4 | Worker-002（工人） | "你是 **Worker-002 Agent**。请读取 `tasks/ARCHITECTURE.md` 和 `tasks/workers/instructions.md`，然后检查你的 inbox 并等待任务分配。" |
| 会话 5 | PR Manager（PR 管理员） | "你是 **PR Manager Agent**。请读取 `tasks/ARCHITECTURE.md` 和 `tasks/pr-manager/instructions.md`，然后检查你的 inbox 并等待 Worker 完成任务。" |
| 会话 6 | Maintainer + Housekeeper | "你是 **Maintainer Agent** 和 **Housekeeper Agent**。请读取 `tasks/ARCHITECTURE.md`、`tasks/maintainer/instructions.md` 和 `tasks/housekeeper/instructions.md`，然后检查各自的 inbox。" |

### 协作流程

```
Planner 制定计划
    ↓ 写入 Coordinator 的 inbox
Coordinator 读取 inbox，分配任务
    ↓ 写入 Worker 的 inbox
Worker 读取 inbox，执行任务
    ↓ 完成通知到 PR Manager 的 inbox
PR Manager 准备 PR
    ↓ 合并后通知 Housekeeper 的 inbox
Housekeeper 清理分支
    ↓
Maintainer 分析日志，改进系统
```

### 注意事项

1. **每个会话只扮演一个角色** — 不要让一个会话同时扮演多个 Agent
2. **通过文件协作** — 不要在会话里问另一个 Agent，直接读写 `tasks/` 下的文件
3. **日志自动记录** — 每个 Agent 会自动写日志到 `tasks/logs/`
4. **你是旁观者** — 只需要审批重要决策，其他让 Agent 自己协调

---

## � 阀门操作指南（核心机制）

### 为什么需要"阀门"？

理想状态：Agent像人类团队，想跟谁说就跟谁说，对方自动收到。
现实：Trae没有全自主Agent功能，Agent无法主动唤醒其他Agent。

**解决方案：你（用户）是"阀门"，控制哪个Agent能"听到"。**

### 核心概念

| 概念 | 含义 |
|------|------|
| 沉睡 | Agent收到消息但未被人唤醒，无法执行 |
| 唤醒 | 你打开特定Agent的会话（唯一人工操作） |
| 睁眼 | 被唤醒的Agent主动读取自己的inbox消息 |
| 声音 | Agent写入共享文件的消息 |
| 待机 | Agent被唤醒但未收到消息，轮询等待中（可选） |

### 你的唯一工作

**你只需要做一件事：打开指定Agent的会话。**

Agent会告诉你：
1. 下一步该唤醒谁
2. 为什么需要唤醒
3. 该Agent需要什么准备

### 待机模式（可选）

你可以让 Agent 在唤醒后进入**待机轮询**，自动监听 inbox 或分配表而非等待你手动唤醒。

**两种待机类型**：

| 类型 | 用法 | 适用 |
|------|------|------|
| inbox 待机 | "待机模式，等 [来源Agent] 消息" | Coordinator, PR Manager |
| 分配表待机 | "待机模式，等分配表" | Worker |

**效果**：Agent 会每 5 分钟检查一次，收到消息/任务后自动开始工作，无需你再次手动唤醒。

**大规模待机**：你可以一次性开多个 Worker 分配表待机，Coordinator 分配完任务后 Worker 自动对号入座，没分到任务的 Worker 自主结束。详见 `docs/agent-rules/cli-operations.md#待机模式`。

### 完整操作流程

```
┌─────────────────────────────────────────┐
│  1. Planner完成工作                      │
│     → 写入消息到 Coordinator的inbox      │
│     → 告知你："请唤醒 Coordinator"       │
└─────────────────────────────────────────┘
                    │
                    ▼ 你操作（打开新会话）
┌─────────────────────────────────────────┐
│  2. Coordinator被唤醒                    │
│     → 读取自己的inbox                    │
│     → 自主判断读什么、做什么              │
│     → 完成后写入Worker的inbox            │
│     → 告知你："请唤醒 Worker"            │
└─────────────────────────────────────────┘
                    │
                    ▼ 你操作（打开新会话）
┌─────────────────────────────────────────┐
│  3. Worker被唤醒                         │
│     → 读取自己的inbox                    │
│     → 执行任务                           │
│     → 完成后告知下一步                   │
└─────────────────────────────────────────┘
```

### 阀门操作示例

#### 场景：Planner完成后

Planner会输出：

```markdown
---

## 用户操作指引

### 本次完成
- 观察了项目状态
- 制定了4个任务计划
- 已写入任务队列

### 下一步操作

**请唤醒**：Coordinator
**原因**：有4个任务需要分配给Worker
**该Agent需读取**：
  - tasks/shared/inbox/coordinator.md（你的消息）
  - tasks/coordinator/queue.md（任务队列）

### 人工介入点
- 无（全自动流转）
```

**你只需要做**：开一个新会话，告诉它"你是Coordinator..."

#### 场景：需要人工介入时

如果Agent遇到必须人工处理的情况：

```markdown
### 人工介入点
- ⚠️ **PR审批**：需要你查看PR草稿并决定是否提交
```

### 消息收件箱位置

```
tasks/shared/inbox/
├── planner.md      # Planner的收件箱
├── coordinator.md   # Coordinator的收件箱
├── worker.md       # Worker的收件箱
├── pr-manager.md   # PR Manager的收件箱
├── maintainer.md   # Maintainer的收件箱
└── housekeeper.md  # Housekeeper的收件箱

tasks/shared/agent-status.md  # 所有Agent的状态
```

### 阀门原则总结

1. **最小人工介入** — 你只需打开会话
2. **最大自动化** — Agent自主判断读什么、做什么
3. **清晰指引** — 每个Agent明确告诉你下一步
4. **可追溯** — 所有消息在inbox中记录

---

## �💬 常用指令

### 作为旁观者

| 你说 | 系统做 |
|------|--------|
| "开始工作吧" | Planner 开始观察并制定计划 |
| "现在做到哪了？" | Planner 汇报当前状态 |
| "有什么可以改进的？" | Maintainer 分析并提出建议 |
| "系统有什么问题？" | Maintainer 诊断问题 |

### 介入指挥

| 你说 | 系统做 |
|------|--------|
| "暂停当前任务" | Coordinator 停止分配 |
| "优先做 X" | Planner 调整计划 |
| "这个 PR 可以提交了" | PR Manager 提交 PR |
| "批准改进 #3" | Maintainer 实施改进 |

---

## 📁 协调文件结构

```
tasks/
├── ARCHITECTURE.md       # 系统架构（所有人读）
├── planner/              # Planner 的工作区
│   ├── observations.md   # 当前观察
│   ├── plans/           # 任务计划
│   └── backlog.md       # 长期待办
├── coordinator/          # Coordinator 的工作区
│   ├── queue.md         # 任务队列
│   └── assignments.md   # 任务分配
├── workers/             # Worker 的工作区
│   ├── status.md       # Worker 状态
│   └── branches.md     # 分支记录
├── pr-manager/          # PR Manager 的工作区
│   ├── pr-queue.md     # 待处理 PR
│   └── pr-history.md   # PR 历史
├── maintainer/          # Maintainer 的工作区
│   ├── improvements.md # 改进队列
│   └── reports/        # 分析报告
├── housekeeper/         # Housekeeper 的工作区
│   └── cleanup-queue.md # 分支清理队列
└── logs/               # 日志
    ├── planner.log
    ├── coordinator.log
    └── ...
```

---

## 🔄 工作流程

### 1. 日常迭代

```
你："开始今天的工作"
    ↓
Planner 观察项目状态
    ↓
Planner 制定计划
    ↓
Coordinator 分配任务给 Worker
    ↓
Worker 执行任务
    ↓
PR Manager 准备 PR
    ↓
你审批 PR
    ↓
PR 提交到上游
    ↓
Housekeeper 清理分支
```

### 2. 自我改进

```
Maintainer 分析日志
    ↓
发现问题 → 提出改进建议
    ↓
你批准改进
    ↓
实施改进
    ↓
系统变得更好
```

---

## 🧹 分支管理

### 分支命名规则

| 类型 | 格式 | 基于 |
|------|------|------|
| 功能开发 | `agent/worker-001/fix-xxx` | upstream/main |
| PR | `feat/42-fix-bug` | upstream/main |
| 协调系统 | `agent/planner/xxx` | main |

### 自动清理

- **PR 合并后**：Housekeeper 自动删除对应的 feat/ 分支
- **过期分支**：超过 7 天的 dev/、14 天的 agent/ 分支会标记清理
- **永不删除**：main、upstream/*

---

## 📊 查看状态

### 查看当前任务

```bash
# 看任务队列
cat tasks/coordinator/queue.md

# 看 Worker 状态
cat tasks/workers/status.md

# 看 PR 进度
cat tasks/pr-manager/pr-queue.md
```

### 查看日志

```bash
# 看最近活动
cat tasks/logs/planner.log
cat tasks/logs/pr-manager.log
```

---

## 🔧 重置系统

如果需要重新开始：

1. 清理运行数据：
```bash
# 清空通知
echo '[]' > notifications/github-activity.jsonl
echo '{"last_notification_timestamp":"1970-01-01T00:00:00Z","last_read_timestamp":"1970-01-01T00:00:00Z","unread_count":0,"collected_at":"1970-01-01T00:00:00Z","summary":"No new activity"}' > notifications/github-meta.json

# 清空 tasks/ 运行数据（保留模板）
git checkout -- tasks/planner/observations.md
git checkout -- tasks/coordinator/queue.md
git checkout -- tasks/workers/status.md
# ... 其他运行文件
```

2. 提交：
```bash
git add -A
git commit -m "chore: reset system for new iteration"
git push
```

---

## ❓ 常见问题

**Q: AI 之间会冲突吗？**
A: 不会。有文件锁机制防止同时修改同一文件。

**Q: PR 会包含 AI 专用文件吗？**
A: 不会。PR 分支基于 upstream/main，天然干净。

**Q: 我需要做什么？**
A: 主要做旁观者，审批重要的 PR 和改进建议。

**Q: 系统出问题怎么办？**
A: 告诉 Maintainer "分析一下系统有什么问题"。

---

## 📚 相关文档

| 文档 | 说明 |
|------|------|
| `AGENTS.md` | 系统宪法，完整架构说明 |
| `tasks/ARCHITECTURE.md` | 详细架构文档 |
| `docs/agent-rules/` | 开发规范（Git、编码、CLI） |
| `docs/plans/` | 设计文档 |

---

**版本**：v1.0
**更新**：2026-04-18
