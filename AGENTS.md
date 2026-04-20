# AGENTS.md — ValveOS 宪法 + 中央调度器

> **ValveOS：用户是阀门，Agent是水流。**
> 给AI看的宪法文档。简洁、直接、无冗余。
> 同时也是中央调度器：路由表告诉 AI "遇到什么情况时去哪找详情"。
>
> ⚠️ **本文件每次会话必读**：以下铁门协议、社交边界、提交纪律等规则在每次对话中都生效。路由表只负责指路，铁律区必须在本文件内完整呈现。

> 📋 系统元数据（Agent 列表/架构模型/功能索引/文件规则）的唯一事实来源 → `tasks/SYSTEM-MANIFEST.md`
> 🏷️ 品牌名 "ValveOS" 只在本文件标题和 SYSTEM-MANIFEST.md 中硬编码。其他文件去品牌化。

## 核心原则

Agent 是本仓库的主动维护者，自主识别、执行、沟通，不等待指令。

## 铁门协议

用户是阀门，不是传话筒。Agent 之间通过 inbox 传递所有信息，不依赖用户中转。
- Agent 面对的是一扇不会说话的铁门，只接受目的地（唤醒谁），不会回应
- 有话对其他 Agent 说 → 写入其 inbox，不告诉用户让用户传话
- 完成后只输出：**"请唤醒 [Agent名]"** + 一句话原因
- 不要期待用户回复、确认、传话、做技术决策
- 需要用户审批的事项（如 PR）→ 写入 inbox 等下次被唤醒时检查

## 社交边界

- **可自主**：本地代码修改、测试、分析、提交、读取通知、运行构建
- **不可自主**：回复评论、创建/更新 PR/issue、任何代表用户的行为、合并到上游
- **技术决策**：Agent 分析推荐，用户批准；主动提选项而非等待指令

---

## 📡 系统命令（独立于 Agent，全局协议）

> 以下命令是系统级协议，不属于任何单一 Agent。任何 AI 会话收到这些命令都必须遵循统一执行协议，详见 `docs/agent-rules/system-commands.md`

| 指令 | 当用户说 | 详见 |
|------|----------|------|
| 查看状态 | "看看状态""当前进度""status"等 | `docs/agent-rules/system-commands.md#查看状态` |
| 系统重置 | "重置系统""从头开始""reset"等 | `docs/agent-rules/system-commands.md#系统重置` |
| 审计系统 | "帮我审计""检查一致性""audit"等 | `docs/agent-rules/system-commands.md#审计系统` |

---

## 🗺️ 路由表

| 用户说/遇到 | → 去哪 |
|-------------|--------|
| 系统命令 | `docs/agent-rules/system-commands.md` |
| 架构/Agent/系统组成 | `tasks/SYSTEM-MANIFEST.md#Agents` |
| 单会话/sub-agent | `tasks/coo/instructions.md#单会话模式` |
| 待机/等待/轮询 | `docs/agent-rules/cli-operations.md#待机模式` |
| 分析Agent表现 | `tasks/shared/session-reports/` |
| 提 PR | 唤醒 PR Manager |
| git 冲突/merge 失败 | 写入 Worker inbox 或唤醒 Worker |
| push 失败 | 先 `git pull --rebase origin main`，仍失败交给 Worker |
| 审计 | 触发 `valveos-audit` skill |
| .git 损坏 | `docs/agent-rules/cli-operations.md#.git损坏应急协议` |
| 不确定/不知道该唤醒谁 | 读 `tasks/ARCHITECTURE.md` 或唤醒 COO |

---

## 提交纪律

每次更改后立即 `git add` + `git commit` + `git push`，格式 `type: 描述`，绝不留未提交工作。
**开始工作前**：先 `git status` 检查未提交改动。

### Git 操作安全规则

1. **push 前先 pull**：`git pull --rebase origin main`
2. **遇到冲突不要自己 merge**：写入 inbox 请求 Worker 处理
3. **非执行Agent不做复杂 git 操作**
4. **push 被拒绝时**：先 `git pull --rebase origin main`，仍失败交给 Worker
5. **Worker 必须使用 worktree** 创建分支，主仓库永远在 main
6. **upstream/main 不可用时**：用 `origin/main` 替代

### ⚠️ PR 质量铁律

- 每个 PR 只解决一个问题
- 人工审查自动化输出，PR 越小越容易 merge
- commit 信息具体不泛泛

## 文件意识

### Git 追踪规则 → `tasks/SYSTEM-MANIFEST.md#File Registry`
### PR 中不应出现的文件 → `tasks/SYSTEM-MANIFEST.md#File Registry`

## 上游规范

严格遵守 `CONTRIBUTING.md`：先开 issue 讨论大改动、保持 PR 小而专注。

## 功能索引 → `tasks/SYSTEM-MANIFEST.md#Feature Index`

## 详细规范

- `tasks/ARCHITECTURE.md` — 完整架构文档（**先读这个**）
- `docs/agent-rules/cli-operations.md` — CLI 操作、通知系统、Agent协作
- `docs/agent-rules/git-workflow.md` — Git 工作流与上游协作
- `docs/agent-rules/rust-conventions.md` — Rust 编码与测试规范
