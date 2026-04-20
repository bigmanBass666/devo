# ValveOS System Manifest

> **本文件是 ValveOS 所有元数据的唯一事实来源。**
> 其他文件需要 Agent 列表/架构模型/概念/文件映射等信息时，应引用本文件而非自行声明。
> 修改元数据时，只需改这里，然后运行审计确认同步。
>
> **审计 skill 以此文件为动态基线的主来源。**

---

## Agents（Agent 清单）

| ID | 名称 | 类型 | 层级 | Inbox 路径 | Instructions 路径 |
|----|------|------|------|-----------|------------------|
| planner | Planner | core-pipeline | 1 | `tasks/shared/inbox/planner.md` | `tasks/planner/instructions.md` |
| coordinator | Coordinator | core-pipeline | 2 | `tasks/shared/inbox/coordinator.md` | `tasks/coordinator/instructions.md` |
| worker | Worker | core-pipeline | 3 | `tasks/shared/inbox/worker.md` | `tasks/workers/instructions.md` |
| pr-manager | PR Manager | core-pipeline | 4 | `tasks/shared/inbox/pr-manager.md` | `tasks/pr-manager/instructions.md` |
| maintainer | Maintainer | cross-cutting | — | `tasks/shared/inbox/maintainer.md` | `tasks/maintainer/instructions.md` |
| housekeeper | Housekeeper | cross-cutting | — | `tasks/shared/inbox/housekeeper.md` | `tasks/housekeeper/instructions.md` |
| coo | COO | cross-cutting | — | `tasks/shared/inbox/coo.md` | `tasks/coo/instructions.md` |

**总计**: 7 个 Agent（核心流水线 4 个 + 横切服务 3 个）

---

## Architecture Model（架构模型）

```
名称: 核心流水线 + 横切服务
类型: pipeline + cross-cutting

核心流水线（线性流转）:
  Planner → Coordinator → Worker → PR Manager

横切服务（独立触发，覆盖所有层）:
  Maintainer   — 数据分析后台
  Housekeeper  — 仓库清理后台
  COO          — 系统维护后台
```

**关键特征**:
- 单会话模式: 在 /spec 模式下可用 sub-agent 替代 Worker 执行代码编写任务
- 待机模式: 支持 3 种（Coordinator inbox / Worker dispatch / PR Manager inbox）
- 工作隔离: Worker 必须使用 git worktree 创建独立工作目录

---

## Core Concepts（核心概念）

> 定义详情见 `tasks/ARCHITECTURE.md#核心概念`

| 概念 | 定义 |
|------|------|
| 沉睡 | Agent 收到消息但未被人唤醒，无法执行 |
| 唤醒 | 用户打开特定 Agent 的会话 |
| 睁眼 | 被唤醒的 Agent 主动读取自己的 inbox 消息 |
| 声音 | Agent 写入共享文件的消息 |
| 待机 | Agent 被唤醒但未收到消息，轮询等待中 |

---

## File Registry（文件注册表）

### 核心文件

| 路径 | 追踪 | 用途 |
|------|------|------|
| `AGENTS.md` | ✅ | 宪法/总纲（原则、协议、纪律） |
| `tasks/SYSTEM-MANIFEST.md` | ✅ | 本文件（元数据唯一事实来源） |
| `tasks/ARCHITECTURE.md` | ✅ | 完整架构文档（Agent 详情、通信机制、分支策略等） |
| `tasks/shared/agent-status.md` | ✅ | Agent 状态与任务追踪 |
| `tasks/shared/iteration-log.md` | ✅ | 迭代日志（断点续传） |
| `tasks/coo/audit-log.md` | ✅ | COO 审计日志 |
| `tasks/shared/decisions.md` | ✅ | 技术决策记录（为什么选X不选Y） |
| `tasks/shared/project-understanding.md` | ✅ | 上游代码结构理解（crate 映射/编译命令/修改模式） |
| `tasks/shared/session-reports/*.md` | ✅ | 7 个 Agent 的会话摘要（观察/异常/建议） |
| `docs/agent-rules/cli-operations.md` | ✅ | CLI 操作参考 |
| `docs/agent-rules/git-workflow.md` | ✅ | Git 工作流与上游协作 |
| `docs/agent-rules/rust-conventions.md` | ✅ | Rust 编码与测试规范 |
| `tasks/logs/README.md` | ✅ | 日志系统说明 |

### 运行时数据文件

| 路径模式 | 追踪 | 说明 |
|----------|------|------|
| `tasks/shared/inbox/*.md` | ✅ | Agent 消息收件箱（7 个，见 Agents 表） |
| `tasks/coordinator/queue.md` | ✅ | 任务队列 |
| `tasks/workers/status.md` | ✅ | Worker 状态表 |
| `tasks/workers/branches.md` | ✅ | 分支记录 |
| `tasks/workers/locks/*.lock` | ❌ | 文件锁（运行时） |
| `tasks/housekeeper/cleanup-queue.md` | ✅ | 分支清理队列 |
| `tasks/maintainer/improvements.md` | ✅ | 改进队列 |
| `tasks/maintainer/reports/*.md` | ✅ | 分析报告输出 |
| `tasks/pr-manager/pr-checklist.md` | ✅ | PR 质量检查模板 |
| `tasks/pr-manager/pr-queue.md` | ✅ | PR 队列 |
| `tasks/pr-manager/pr-history.md` | ✅ | PR 历史 |
| `tasks/planner/plans/*.md` | ✅ | 任务计划 |
| `tasks/planner/backlog.md` | ✅ | 长期待办 |
| `notifications/*.json` | ✅ 可选 | GitHub 日志 |
| `tasks/logs/system-commands.log` | ❌ | 系统命令日志（不纳入 Git） |
| `tasks/logs/*.log` | ❌ | 运行时日志（不纳入 Git） |
| `.trae/*` | ❌ | AI 状态数据 |

### PR 中不应出现的文件

以下内容**永远不要**出现在给上游的 PR 中：
- `tasks/` 目录
- `notifications/` 目录
- `.trae/` 目录
- `AGENTS.md`
- `SYSTEM-MANIFEST.md`

---

## Feature Index（功能索引）

| 功能 | 触发条件 | 详情位置 |
|------|----------|----------|
| ⚡ 系统命令 | 用户输入以 `/` 开头 | `system-commands.md`（斜杠命令协议） |
| 🔄 系统重置 | `/reset` 或 "执行系统重置" | `cli-operations.md#系统重置` |
| 🔔 通知消费 | 检查 GitHub 动态时 | `cli-operations.md#通知系统` |
| 📝 Inbox 读写 | 向其他 Agent 发消息时 | `cli-operations.md#Agent协作操作` |
| 🐛 调试方法 | 遇到 bug 时 | `cli-operations.md#调试方法论` |
| 📂 Git 工作流 | 创建分支 / 提 PR 时 | `git-workflow.md` |
| 🚨 Git 损坏 | git 命令报错时 | `cli-operations.md#.git损坏应急协议` |
| 💤 待机模式 | Agent 待机轮询 inbox 时 | `cli-operations.md#待机模式` |
| 🔧 COO 审计 | 每次文档改动后 | `valveos-audit skill` |

---

## 所有权约定（信息声明规则）

> **每类元数据只有一个"声明处"，其他地方只引用不声明。违反此约定的不一致由审计 skill P1 #12 检测。**

| 信息类型 | 🔑 唯一声明处 | 其他文件的处理方式 |
|---------|-------------|-------------------|
| Agent 清单（ID/名称/类型/层级） | **本文件 → Agents 表** | 各 `instructions.md` 用角色标签行引用；ARCHITECTURE.md 架构图保留可视化展示但标注派生 |
| 架构模型（名称/结构） | **本文件 → Architecture Model** | AGENTS.md 用一行概要引用；ARCHITECTURE.md 标题引用 |
| 核心概念（定义） | **ARCHITECTURE.md → 核心概念章节** | 本文件只存索引；instructions.md 使用术语但不重新定义 |
| 功能索引 | **本文件 → Feature Index** | AGENTS.md 引用（内联简版或"详见 Manifest"） |
| 文件追踪规则 | **本文件 → File Registry** | AGENTS.md 不再包含追踪表（完全迁移到此处） |
| inbox 文件列表 | **本文件 → Agents 表（Inbox 路径列）** | ARCHITECTURE.md 通信机制图从本表派生，不再独立维护 |
| 日志文件列表 | **本 File Registry 或 ARCHITECTURE.md 日志系统图** | 两处应一致，以本文件为准 |
| Agent 职责详情 | **各自 `instructions.md`** | 本文件只有一句话摘要；ARCHITECTURE.md 有完整版 |
| 标准开场白 | **ARCHITECTURE.md → 标准开场白章节** ⚠️ | 本文件引用其位置（注：该章节尚待创建，见 audit-log 评估记录） |
| Git 安全规则 | **AGENTS.md → 提交纪律章节** | 各 instructions.md 可引用但不重复完整规则 |
| PR 质量铁律 | **AGENTS.md → PR 质量铁律章节** | pr-manager/instructions.md 引用具体条目 |
| AGENTS.md 路由条目 | **AGENTS.md → 路由表章节** | 其他文件不重复定义路由；SYSTEM-MANIFEST.md Feature Index 与路由表保持覆盖一致 |

---

> **路由覆盖原则**: AGENTS.md 路由表应覆盖系统中所有"用户可能问到的概念"。当新增重要概念时，应同步在路由表添加对应条目。路由条目的触发词应包含用户最可能使用的关键词变体。

> **去品牌化原则**: 品牌名 "ValveOS" 只在 AGENTS.md（标题+元数据行）和本文件（标题行）中硬编码，共约 4 处。其他文件（shared/*.md、instructions.md 等）只描述功能，不硬编码品牌名。需要引用系统名时使用"本系统"或引用 AGENTS.md 标题行。改名时只需修改 AGENTS.md + 本文件。

> **闭环分形原则**: 闭环没有固定的范围边界。当前解决的是项目级闭环（同一项目跨会话），但未来可能需要跨项目级闭环。不要硬编码闭环的范围——设计时应保持闭环机制的可扩展性。

## AGENTS.md 章节分类原则

AGENTS.md 的章节分为两类，遵循不同的设计模式：

### 可路由化章节（信息查找类）

用户问"X 是什么"→ 路由告诉 AI 去哪找答案。

特征：描述事实、指向外部文件、可被触发词匹配。

| 章节名 | 路由目标 |
|--------|----------|
| 架构速查 | SYSTEM-MANIFEST.md#Agents |
| 单会话模式 | coo/instructions.md#单会话模式 |
| 待机模式 | cli-operations.md#待机模式 |
| 操作指引 | 多目标子路由表 |
| 错误恢复 | 按错误类型分发 |
| 审计触发 | valveos-audit skill |

### 不可路由化章节（行为约束类）

告诉 AI "必须/禁止做 X" → 这本身就是答案。

特征：声明规则、约束行为、不需要外部查找。

| 章节名 | 约束类型 |
|--------|----------|
| 核心原则 | Agent 身份宣言 |
| 铁门协议 | 通信规则 |
| 社交边界 | 权限边界 |
| 提交纪律 | Git 安全规则 + PR 质量 |
| 上游规范 | 外部协作规则 |

### 判断标准

如果该章节的内容是"AI 遇到某场景时需要查找的信息"，则可路由化；如果是"AI 必须遵守的规则"，则不可路由化。

## 变更历史

| 日期 | 变更内容 | 操作者 |
|------|----------|--------|
| 2026-04-19 21:37 | 初始创建：从 AGENTS.md + ARCHITECTURE.md 提取全部元数据 | COO (skill-creator 评估后) |
