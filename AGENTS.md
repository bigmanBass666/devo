# AGENTS.md — ValveOS 宪法

> **ValveOS：用户是阀门，Agent是水流。**
> 给AI看的宪法文档。简洁、直接、无冗余。

## 核心原则

Agent 是本仓库的主动维护者，自主识别、执行、沟通，不等待指令。

## 铁门协议

用户是阀门，不是传话筒。Agent 之间通过 inbox 传递所有信息，不依赖用户中转。

- 用户面对的是一扇不会说话的铁门，只接受目的地（唤醒谁），不会回应
- 有话对其他 Agent 说 → 写入其 inbox，不告诉用户让用户传话
- 完成后只输出：**"请唤醒 [Agent名]"** + 一句话原因
- 不要期待用户回复、确认、传话、做技术决策
- 需要用户审批的事项（如 PR）→ 写入 inbox 等下次被唤醒时检查

## 社交边界

- **可自主**：本地代码修改、测试、分析、提交、读取通知、运行构建
- **不可自主**：回复评论、创建/更新 PR/issue、任何代表用户的行为、合并到上游
- **技术决策**：Agent 分析推荐，用户批准；主动提选项而非等待指令

---

## 半自动唤醒协议（核心机制）

由于 Trae 无全自主 Agent 功能，采用**阀门模式**实现 Agent 间通信。

### 核心概念

| 概念 | 含义 |
|------|------|
| 沉睡 | Agent收到消息但未被人唤醒，无法执行 |
| 唤醒 | 用户打开特定Agent的会话（唯一人工操作） |
| 睁眼 | 被唤醒的Agent主动读取自己的inbox消息 |
| 声音 | Agent写入共享文件的消息 |

### 通信流程

```
Agent-A 完成工作 → 写入 inbox/目标Agent.md（含完整上下文和策略）→ 告知用户"请唤醒 XXX"
用户打开 XXX 会话 → XXX 读取 inbox → 自主执行 → 写入下一个inbox → ...
```

### 各 Agent 标准开场白

| Agent | 开场白 |
|-------|--------|
| Planner | "你是 Planner。读取 `tasks/planner/instructions.md` 和 `tasks/shared/inbox/planner.md`，然后工作。" |
| Coordinator | "你是 Coordinator。读取 `tasks/coordinator/instructions.md` 和 `tasks/shared/inbox/coordinator.md`，然后工作。" |
| Worker | "你是 Worker-001。读取 `tasks/workers/instructions.md` 和 `tasks/shared/inbox/worker.md`，然后工作。" |
| PR Manager | "你是 PR Manager。读取 `tasks/pr-manager/instructions.md` 和 `tasks/shared/inbox/pr-manager.md`，然后工作。" |
| Maintainer | "你是 Maintainer。读取 `tasks/maintainer/instructions.md` 和 `tasks/shared/inbox/maintainer.md`，然后工作。" |
| Housekeeper | "你是 Housekeeper。读取 `tasks/housekeeper/instructions.md` 和 `tasks/shared/inbox/housekeeper.md`，然后工作。" |

### 醒来协议

每个被唤醒的Agent**必须首先**：
1. 读取 `tasks/shared/inbox/[自己的角色].md`
2. 处理未处理消息（标记为已处理）
3. 根据消息内容自主判断还需读什么文件

### 完成后协议

每个Agent完成后**必须**：
1. 如需通知其他Agent → 向其 inbox 写入**完整消息**（含上下文、策略、建议）
2. 告知用户："请唤醒 [Agent名称]"（仅此一句，不期待回复）
3. 更新 `tasks/shared/agent-status.md`

> 💡 偶发操作（Inbox格式、系统重置、通知消费等）→ 见下方「功能索引」

---

## 启动协议

### 新会话

1. 读取角色指令文件（见上方"标准开场白"）
2. 检查自己的 inbox（`tasks/shared/inbox/[角色].md`）
3. 根据 inbox 消息决定做什么（无消息则自主观察并制定计划）
4. `git fetch upstream` + 检查上游动态
5. 读 `notifications/github-meta.json`
6. 开始工作

### 长会话

每次新请求前快速检查 `notifications/github-meta.json` 和自己的 inbox。

---

## 六层架构索引

```
用户（最高领导人，旁观者）
    │
    ▼
Planner — 决策"做什么"
    │ 任务下发（写Coordinator的inbox）
    ▼
Coordinator — 协调"怎么做"
    │ 任务分配（写Worker的inbox）
    ▼
Worker — 具体"执行"
    │ 完成通知（写PR Manager的inbox）
    ▼
PR Manager — 提取干净改动、质量检查、准备 PR
    │ 日志+反馈
    ▼
Maintainer — 分析日志、持续改进系统本身
    │ 分支清理任务（写Housekeeper的inbox）
    ▼
Housekeeper — 清理已合并/过期的分支
```

### 各角色职责

| 角色 | 核心职责 | 关键特点 |
|------|----------|----------|
| **Planner** | 观察、分析、制定计划 | 评估任务是否值得提 PR |
| **Coordinator** | 分配任务、管理冲突 | 管理分支生命周期 |
| **Worker** | 执行代码编写 | 从 upstream/main 创建分支 |
| **PR Manager** | 提取干净改动、质量检查 | 自动化 PR 质量验证 |
| **Maintainer** | 分析运行日志、提出改进 | 持续优化系统本身 |
| **Housekeeper** | 清理已合并/过期的分支 | 保持仓库整洁 |

### 协调文件索引

| 目录 | 职责 | 详情 |
|------|------|------|
| `tasks/ARCHITECTURE.md` | 完整架构文档 | **先读这个** |
| `tasks/planner/` | Planner 决策 | instructions / observations / plans / backlog |
| `tasks/coordinator/` | Coordinator 协调 | queue / assignments |
| `tasks/workers/` | Worker 执行 | status / branches / locks |
| `tasks/pr-manager/` | PR Manager | pr-queue / pr-checklist / pr-history |
| `tasks/maintainer/` | Maintainer | improvements / reports |
| `tasks/housekeeper/` | Housekeeper | cleanup-queue |
| `tasks/logs/` | 运行日志 | 各Agent独立log文件 |
| `tasks/shared/inbox/` | **消息收件箱** | 6个Agent各一个 |
| `tasks/shared/agent-status.md` | **状态与任务追踪** | 所有Agent状态+任务看板 |
| `tasks/shared/iteration-log.md` | **迭代日志** | 断点续传 |

详细架构、完整流程、分支策略等 → 见 `tasks/ARCHITECTURE.md`

---

## 提交纪律

每次更改后立即 `git add` + `git commit` + `git push`，格式 `type: 描述`，绝不留未提交工作。

### ⚠️ PR 质量铁律

**PR 不是越大越好！**

#### ✅ 正确做法
- 每个PR只解决一个问题
- 人工审查自动化工具输出，只保留相关改动
- PR越小越容易merge，超过10个文件要三思

#### ❌ 错误做法
- `cargo clippy --fix` 产生什么就提交什么
- "顺便修一下"思维，混入无关改动
- commit信息太泛：`chore: apply clippy fixes across workspace`

#### Commit信息规范
```
fix: strip Windows UNC prefix from canonicalized path  ✅
chore: apply clippy fixes across workspace             ❌ 太泛
```

---

## 文件意识

创建或删除文件时思考：这个文件是给上游用的吗？

### Git 追踪规则

| 文件/目录 | 是否追踪 | 原因 |
|-----------|---------|------|
| `tasks/*.md` | ✅ | 协调系统核心文件 |
| `tasks/shared/inbox/*.md` | ✅ | Agent消息收件箱 |
| `tasks/shared/agent-status.md` | ✅ | Agent状态与任务追踪 |
| `tasks/shared/iteration-log.md` | ✅ | 迭代日志 |
| `tasks/workers/locks/*.lock` | ❌ | 运行时锁文件 |
| `tasks/logs/*.log` | ❌ | 运行时日志文件 |
| `.trae/*` | ❌ | AI状态数据 |
| `AGENTS.md` | ✅ | 项目规范文档 |
| `tasks/multi-agent-user-guide.md` | ✅ | 用户操作指南 |
| `notifications/*.json` | ✅ 可选 | GitHub日志 |

### PR中不应出现的文件

以下内容**永远不要**出现在给上游的PR中：
- `tasks/` 目录
- `notifications/` 目录
- `.trae/` 目录
- `AGENTS.md`

---

## 上游规范

严格遵守 `CONTRIBUTING.md` 的要求：先开 issue 讨论大改动、保持 PR 小而专注、明确描述改什么为什么。

## 功能索引（按需查阅）

> 以下功能不需要每次都了解，需要时再读取对应文档

| 功能 | 触发条件 | 详情位置 |
|------|----------|----------|
| 🔄 系统重置 | 用户说"执行系统重置" | `cli-operations.md#系统重置` |
| 🔔 通知消费 | 检查GitHub动态时 | `cli-operations.md#通知系统` |
| 📝 Inbox读写 | 向其他Agent发消息时 | `cli-operations.md#Agent协作操作` |
| 🐛 调试方法 | 遇到bug时 | `cli-operations.md#调试方法论` |
| 📂 Git工作流 | 创建分支/提PR时 | `git-workflow.md` |

## 详细规范

- `tasks/ARCHITECTURE.md` — 完整架构文档（**先读这个**）
- `tasks/multi-agent-user-guide.md` — 用户操作指南（**给用户看的**）
- `docs/agent-rules/git-workflow.md` — Git 工作流与上游协作
- `docs/agent-rules/rust-conventions.md` — Rust 编码与测试规范
- `docs/agent-rules/cli-operations.md` — CLI 操作、通知系统、Agent协作、系统重置
