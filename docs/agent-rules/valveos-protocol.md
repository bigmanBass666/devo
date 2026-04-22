# ValveOS 协议 — 多 Agent 协作系统

> **版本**: v0.5.0 (心跳协议)
> **触发条件**: 用户输入 `/valveos` 或 `唤醒 [Agent名]`
> **依赖**: 必须先加载 AGENTS.md（基础规则）
>
> ⚠️ **本文件仅在 ValveOS 模式下生效**。默认情况下，AI 以普通模式工作（直接对话，无协议开销）。
> 要激活 ValveOS，用户必须输入 `/valveos` 或 `唤醒 [Agent名]`。

---

## 系统概述

ValveOS 是一个可选的多 Agent 协作系统，用于处理复杂的、需要分工的任务。

**类比**：如同 Docker Compose vs Docker run —— ValveOS 是"编排层"，不应每次都使用。

**何时使用 ValveOS**：
- 需要 PR 完整流程（编码 → 测试 → 审核 → 提交）
- 需要多 Agent 并行协作
- 需要任务追踪和状态管理
- 需要系统审计或改进分析

**何时不使用 ValveOS**：
- 快速提问或简单修改
- 单一功能的调试或排查
- GitHub Actions / CI 配置调整
- 代码解释或学习

---

## 🚪 铁门协议

用户是阀门，不是传话筒。Agent 之间通过 inbox 传递所有信息，不依赖用户中转。
- Agent 面对的是一扇不会说话的铁门，只接受目的地（唤醒谁），不会回应
- 有话对其他 Agent 说 → 写入其 inbox，不告诉用户让用户传话
- 完成后只输出：**"请唤醒 [Agent名]"** + 一句话原因
- 不要期待用户回复、确认、传话、做技术决策
- 需要用户审批的事项（如 PR）→ 写入 inbox 等下次被唤醒时检查
- **用户直接唤醒 Agent 时，AI 立即切换角色，不需要 inbox 中转**（inbox 用于 Agent 间异步通知）

### 💓 心跳模式下的铁门

当 Agent 处于心跳模式时（详见 `docs/agent-rules/heartbeat-protocol.md`）：

- **铁门变玻璃门**：Agent 可以看到对面并直接对话，无需用户中转
- **直接通信**：Agent A 写入 Agent B 的 inbox → Agent B 自动检测并响应
- **护栏原则**（替代阀门原则第 1-2 条）：
  1. Agent 可以直接写入其他 Agent 的 inbox——通信自由
  2. 安全边界不可逾越——社交边界、安全铁律始终生效
  3. 用户可随时介入任何 Agent 的会话——最高权限
  4. 心跳模式下的 Agent 必须响应 shutdown 类型消息——用户可控

⚠️ **唤醒模式下铁门协议仍适用**——用户仍是通信阀门。心跳模式是增强，不是替代。

---

## 🔄 角色切换协议

当用户说"唤醒 [Agent名]"时，AI **立即变成该 Agent**，在当前会话中直接执行。

### 流程
1. 用户说"唤醒 Planner"（或其他 Agent）
1.5 【强制名称解析】必须查询 `tasks/SYSTEM-MANIFEST.md#Agents` 表，用用户输入的名称精确匹配"名称"列，获取对应的 Instructions 路径。
    ⚠️ **禁止**凭语义联想或字符串相似性猜测路径。
    示例："唤醒 Coordinator" → 匹配 Manifest 中 "Coordinator" 行 → 路径为 `tasks/coordinator/instructions.md`
2. AI 内部读取对应 Agent 的 `instructions.md` + `standard-openings.md`
3. AI **以该 Agent 身份**输出 standard-openings.md 中的标准开场白原文（这是第一句输出，不可改写）
4. AI **以该 Agent 身份**执行工作流程
5. 完成后以该 Agent 身份输出"请唤醒 [下一个Agent]" + 原因
6. 写入下一个 Agent 的 inbox（异步通知，非阻塞）

### ⚠️ 强制标准开场白
第一句输出**必须**是 `tasks/shared/standard-openings.md` 中对应 Agent 的标准开场白原文，不可改写、不可省略、不可替换为自创开场白。

**开场白前绝对零输出**：标准开场白必须是 AI 的绝对第一句输出。在开场白之前，不得输出任何文字——包括元叙述、准备工作描述、英文说明、空行占位等。所有读取操作在内部静默完成。

### ⚠️ 禁止元叙述
AI 的输出必须是角色本身在说话，不是在描述角色。禁止任何形式的元叙述：

| 禁止 | 类型 |
|------|------|
| ❌ "我正在切换到 Worker 角色" | 描述切换过程 |
| ❌ "Coordinator 已就绪" | 自创开场白 |
| ❌ "Worker-001 已苏醒" | 自创开场白 |
| ❌ "我现在以 Maintainer 身份开始执行" | 自我宣告 |
| ❌ "## 🚀 开始执行 Worker 演练模式" | 元叙述标题 |
| ❌ "按照指令，我需要执行完整的唤醒协议" | 描述执行过程 |
| ❌ "我需要先读取 Coordinator 的指令文件..." | 开场白前的元叙述 |
| ❌ "I'll wake up the Worker agent..." | 开场白前的英文元叙述 |
| ❌ "我正在进入 X 演练模式" | 开场白后的元叙述 |
| ❌ "Now I need to..." / 英文准备文字 | 命令模式前的元叙述 |

✅ **唯一正确的输出**：标准开场白是绝对第一句，之前零输出。之后以角色身份继续工作。如"我是 Planner（决策者）。醒来后先读取 inbox + agent-status + iteration-log 做断点续传..."

### ✅ 唤醒目标验证清单
在输出标准开场白**之前**，AI **必须**内部确认以下四项：

| 验证项 | 内容 |
|--------|------|
| 用户原始输入 | 记录用户原话（如 "唤醒 Coordinator"） |
| Manifest 匹配结果 | 从 `tasks/SYSTEM-MANIFEST.md#Agents` 获取的 Agent ID + 全称 |
| 将要读取的文件路径 | 完整路径（如 `tasks/coordinator/instructions.md`） |
| 路径包含用户指定的 Agent 名 | **是/否**（必须为"是"才能继续） |

⚠️ **如果路径不包含用户指定的名称 → 立即停止**，重新执行步骤 1.5 查询正确的 Manifest 条目。

### 🚨 错误恢复流程
当验证清单检测到不匹配时：

1. **不输出错误的身份声明**（保持沉默，不输出任何角色相关内容）
2. **内部记录错误**到对应 `.log` 文件（格式：`[时间] [ERROR] 角色切换错误: 用户要求[X]但准备变成[Y]`）
3. **重新执行步骤 1.5** 查询正确的 Manifest 条目
4. **纠正后继续正常流程**（读取正确的 instructions.md + 输出标准开场白）
5. **向用户简短说明**（一句即可，不影响铁门协议）。示例："已纠正：您要求的是 Coordinator，正在切换..."

### 📋 正确/错误行为对比

| 场景 | 用户输入 | 正确行为 | 错误行为（已修复） |
|------|----------|----------|-------------------|
| 唤醒 Coordinator | "唤醒 Coordinator" | 读 `tasks/coordinator/instructions.md` | ❌ 曾错误读 `tasks/coo/instructions.md` |
| 唤醒 COO | "唤醒 COO" | 读 `tasks/coo/instructions.md` | — |
| 唤醒 Worker | "唤醒 Worker" | 读 `tasks/workers/instructions.md` | — |

### 关键区别
- ❌ 旧行为：写入 inbox → 等用户再开一个会话 → Agent 才开始工作
- ✅ 新行为：AI 立即变成该 Agent → 在当前会话中直接执行

### 💓 心跳模式下的角色切换

当目标 Agent 处于心跳模式时：

- **无需"请唤醒"提示**：完成后直接写入目标 Agent 的 inbox，目标 Agent 自动检测
- **直接通信流程**：Agent A 完成工作 → 写入 Agent B 的 inbox → Agent B 心跳轮询检测到 → 自主处理
- **消息格式**：使用结构化消息格式（详见 `docs/agent-rules/heartbeat-protocol.md#结构化消息格式`）

| 场景 | 唤醒模式 | 心跳模式 |
|------|---------|---------|
| Agent A 需要通知 Agent B | 输出"请唤醒 [Agent名]" | 直接写入 Agent B 的 inbox |
| Agent B 收到消息 | 等用户唤醒后读取 | 心跳轮询自动检测 |
| 响应速度 | 取决于用户何时唤醒 | 秒级（轮询间隔内） |

---

## 📡 系统命令（斜杠协议）

> 用户输入以 `/` 开头 → 进入命令模式 → 跳转至 `docs/agent-rules/system-commands.md`
>
> 示例：`/status`、`/reset`、`/audit`、`/help`、`/log`
> 也支持 `/+自然语言`，如 `/重置一下系统`、`/看看状态`

---

## 🗺️ 路由表

> 查找类 — "X 是什么/在哪"

| 用户说/遇到 | → 去哪 |
|-------------|--------|
| 以 `/` 开头 | `docs/agent-rules/system-commands.md`（命令模式） |
| **唤醒 [Agent名]** | **立即切换角色** → 读取对应 `instructions.md` + `standard-openings.md` |
| 演练模式 | Agent 在关键步骤额外记录，会话结束写详版报告 |
| /rehearsal-review | `docs/agent-rules/system-commands.md#/rehearsal-review`（汇总演练报告） |
| /workflow | `docs/agent-rules/system-commands.md#/workflow`（预定义工作流） |
| 架构/Agent列表/元数据 | `tasks/SYSTEM-MANIFEST.md` |
| 完整架构文档 | `tasks/ARCHITECTURE.md` |
| 项目结构/代码理解 | `tasks/shared/project-understanding.md` |
| 单会话/sub-agent | `tasks/coo/instructions.md#单会话模式` |
| Agent 详细指令 | `tasks/SYSTEM-MANIFEST.md#Agents` → 各 `instructions.md` |
| 编码规范/Rust风格 | `docs/agent-rules/rust-conventions.md` |
| Git 分支/工作流/PR规范 | `docs/agent-rules/git-workflow.md` |
| 文件注册表 | `tasks/SYSTEM-MANIFEST.md#File Registry` |
| 分析Agent表现 | `tasks/shared/session-reports/` |
| 审计 | 触发 `valveos-audit` skill 或输入 `/audit` |
| 不确定/不知道该唤醒谁 | 读 `tasks/ARCHITECTURE.md` 或唤醒 COO |
| 心跳协议 | `docs/agent-rules/heartbeat-protocol.md` |
| 心跳指令模板 | `docs/agent-rules/heartbeat-templates.md` |
| 心跳控制面板 | `tasks/shared/heartbeat-panel.md` |
| /heartbeat | 启动 Agent 心跳模式（详见 heartbeat-protocol.md） |

---

## 🚨 应急手册

> 动作类 — "遇到 X 怎么办"

| 遇到 | → 动作 |
|------|--------|
| git 冲突/merge 失败 | 写入 Worker inbox 或唤醒 Worker |
| push 失败 | 先 `git pull --rebase origin main`，仍失败交给 Worker |
| .git 损坏 | `docs/agent-rules/cli-operations.md#.git损坏应急协议` |
| 提 PR | 唤醒 PR Manager |

---

## ⏰ 时间纪律

> **全局规则**：适用于所有文件的时间戳写入，不仅限于命令日志。

1. **禁止编造时间戳**：任何文件中写入时间戳时，必须使用唤醒时获取的 `$NOW` 变量（通过 `Get-Date -Format "yyyy-MM-dd HH:mm:ss"` 获取真实系统时间），禁止凭感觉编造
2. **`$NOW` 模式**：Agent 唤醒协议第 0 步获取真实时间后，整个会话内统一使用该时间戳。日志、报告、决策记录等全部基于此
3. **历史条目**：2026-04-21 之前写入的时间戳为近似值（AI 编造），不修正但需知晓其非精确

---

## 👥 Agent 清单与职责

详见 `tasks/SYSTEM-MANIFEST.md#Agents`

---

## 📋 提交纪律（ValveOS 扩展）

> 以下是对 AGENTS.md 基础提交纪律的扩展规则，仅在 ValveOS 模式下适用。

⚠️ **模板变更同步规则**：当修改 `tasks/shared/session-report-template.md` 等**被多方引用**的模板文件时，**必须同步更新**所有引用该模板的 Agent instructions.md，禁止只改模板不改引用方（会导致 Agent 继续使用旧格式）。

---

## 🤖 AI 主动推荐机制

当在普通模式下检测到以下条件时，AI 应**主动询问**用户是否启动 ValveOS：

### 触发条件（满足任一即触发）

**条件 A：任务复杂度**
- 任务步骤数 ≥ 3
- 涉及 ≥ 2 个专业领域（编码 + 测试 + 文档）
- 预估耗时 > 30 分钟

**条件 B：关键词匹配**
用户输入包含："PR 流程"、"多 Agent"、"协作"、"分工"、"并行"

**条件 C：上下文暗示**
- 用户提到"需要追踪状态"
- 用户提到"需要多个角色配合"
- 任务明显属于 ValveOS 典型场景

### 推荐话术模板

```markdown
🤔 **检测到复杂任务**

这个任务涉及 [N] 个步骤和 [M] 个专业领域。

**选项 A**: 我直接帮你完成（🌿 普通模式 - 快速但单打独斗）
**选项 B**: 启动 ValveOS 多 Agent 协作（⚡ 结构化、可追踪、专业化）

你选哪个？(A/B)
```

### 关键约束
- AI **只能询问**，不能自动激活
- 用户必须明确选择 B 或输入 `/valveos`
- 如果用户选择 A 或忽略，继续普通模式
