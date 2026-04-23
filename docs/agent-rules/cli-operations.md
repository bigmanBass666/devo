# CLI 操作、通知系统与调试

## PowerShell 注意事项
- `&&` 不可用，用 `;` 连接命令
- `curl` 是 PowerShell 别名，需要用 `curl.exe`
- 文件读写只在项目目录内进行，不用系统 `%TEMP%`
- `Out-File` 写系统临时目录会被安全工具拦截，避免使用

## Git 操作
- 优先用 Git MCP 工具（add、commit、status、diff、log、branch）
- push/pull 等 MCP 不支持的才用命令行
- 有未提交更改时先 `git stash` 再 pull

## 通知系统
- 通知文件：`notifications/github-meta.json`（元数据）+ `github-activity.jsonl`（事件日志）
- Actions 每 30 分钟采集：上游 commits、PR 活动、issue 更新、评论
- Agent 消费行为：分析含义 → 汇报用户 → 社交类只建议不行动 → 技术类自主处理
- 读取后更新 `last_read_timestamp`

## 调试方法论
- 遇到 bug 先复现，再定位，最后修复
- GitHub Actions 调试：将日志写入仓库文件（会被提交推送），运行后读取
- API 调试：先在本地用 `curl.exe` 或 `mcp_fetch_fetch` 验证端点可用
- 变量展开问题：shell heredoc 不可靠，用 `jq -n` 构建 JSON
- 权限问题：`GITHUB_TOKEN` 只能访问当前仓库，上游公开仓库用 `curl` 无认证 API

## Fork 维护意识

这是 fork 仓库，提 PR 时需注意：Agent 专用文件不应出现在给上游的 PR diff 中。创建文件时先思考：这个文件是给上游用的吗？

---

## Agent 协作操作

### 读写 Inbox（消息收件箱）

**位置**：`tasks/shared/inbox/[角色].md`（planner / coordinator / worker / pr-manager / maintainer / housekeeper / coo）

**读取**：用 Read 工具读取自己的 inbox，检查"待处理消息"区
**写入**：用 SearchReplace 向目标 Agent 的 inbox 添加消息行：
```markdown
| 时间 | 来源 | 内容摘要 | 状态 |
|------|------|----------|------|
| YYYY-MM-DDTHH:MM:SSZ | 你的Agent名 | [消息内容] | 未读 |
```
**处理后**：将消息从"待处理"移到"已处理"区

### 更新 Agent Status

**位置**：`tasks/shared/agent-status.md`

更新自己的状态和等待唤醒的 Agent：
```markdown
| Agent | 最近活跃 | 当前状态 | 等待唤醒 |
|-------|----------|----------|----------|
| 你的Agent名 | 当前时间 | 沉睡 | - |
| 下一个Agent | - | 未启动 | 你的Agent名 |
```

### 日志记录

**位置**：`tasks/logs/[角色].log`

**格式**：
```
[YYYY-MM-DD HH:MM:SS] [角色] [级别] MESSAGE
  - detail: ...
  - data: { ... }
```

**级别**：INFO / WARN / ERROR / DECISION

**ValveOS 特有事件**（必须按需记录）：
| 事件 | 触发场景 | 使用Agent |
|------|----------|-----------|
| WAKEUP | 被用户唤醒，执行醒来协议 | 所有Agent |
| RESUME | 断点续传，发现上次进度 | 主要Planner |
| MESSAGE | Inbox消息读写 | 所有Agent |
| RESET | 系统重置（完全/选择性） | 主要Housekeeper |
| ITERATION | 迭代生命周期变更 | 主要Planner |
| LOOKUP | 查阅功能索引/文档 | 主要Maintainer |

详细格式示例见 `tasks/logs/README.md` 和各 Agent 的 instructions.md

### 完成后的标准输出

每个 Agent 完成工作后必须输出：

```markdown
请唤醒 [下一个Agent名称]。
```

所有上下文、策略、细节必须已写入目标 Agent 的 inbox。用户是阀门，不传话。

---

## 待机模式

Agent 完成工作后标记为"待机"，下次被唤醒时从断点续传。**不存在后台轮询**——AI 会话是一次性的。

### 定义

- **待机** = Agent 在 agent-status.md 中标记为"待机"，不执行任何后台进程
- **唤醒** = 用户在新会话中说"唤醒 [Agent名]"，AI 读取 instructions + inbox + status，从断点续传
- **轮询** = 不存在。AI 会话没有后台轮询能力。

### 工作流

1. 完成当前工作后，更新 `tasks/shared/agent-status.md` 状态为"待机"
2. 输出"请唤醒 [下一个Agent]" + 原因
3. 会话结束。Agent 不执行任何后台操作。
4. 下次用户唤醒该 Agent 时，AI 读取 inbox + agent-status → 从断点续传

### ⚠️ 已废弃：轮询待机

以下轮询方式已被证明不可行（AI 会话不是持久进程，Start-Sleep 结束后不会自动醒来）：
- ~~Start-Sleep 轮询~~
- ~~while 循环轮询~~
- ~~前台/后台轮询~~

**不要使用任何形式的轮询。**

---

## 系统重置

当用户想要从头开始时，告诉任意Agent **"执行系统重置"**。

### 精确触发词白名单

只有以下**精确短语**才触发系统重置协议。非白名单内的模糊表述不直接触发。

| 触发词 | 操作 |
|--------|------|
| `执行系统重置` | 完全重置（默认） |
| `完全重置系统` | 完全重置（同上） |
| `只重置任务看板` | 只重置 agent-status 任务区 |
| `只归档当前迭代` | 只标记当前迭代为已废弃 |
| `只清空inbox` | 只清空所有收件箱 |

> ⚠️ **精确匹配要求**：只有上述白名单内的精确短语才触发重置。对于模糊表述（如「重置状态」「重置一下」「清理一下」「reset」「清理状态」），Agent **必须先确认意图，不得直接执行**。

> ⚠️ **不可跳步**：以下步骤必须按顺序逐个执行，每完成一步标记 `[x]`。跳步会导致状态不一致（如 iteration-log 与 agent-status 迭代号不匹配）。

### 执行前确认（必须）

> ⚠️ **破坏性操作二次确认**：即使触发了精确匹配，Agent 在**实际修改任何文件之前**必须输出以下确认清单并等待用户回复。

**确认清单模板**：
```
⚠️ 即将执行[完全/选择性]重置，将执行以下操作：
  1. 清空所有 inbox（7个文件）
  2. 重置 agent-status.md 所有Agent为未启动
  3. 归档 Iteration X → 新建 Iteration X+1
  4. [根据实际类型列出其他受影响操作...]

确认执行？回复"确认"/"是"/"yes"/"执行"开始。
```

- 用户回复「确认」「是」「yes」「执行」→ **才开始执行**下方步骤
- 用户回复其他内容或无回复 → **不执行**，等待进一步指示

### 重置操作

Agent会执行以下操作：

- [ ] **清空所有 inbox**（`tasks/shared/inbox/*.md`）→ 恢复为空模板
- [ ] **重置 agent-status.md** → 所有Agent回到"未启动"
- [ ] **归档当前 iteration-log 条目** → 标记为"已废弃"，新建递增迭代号条目（与 agent-status.md 迭代号一致）
- [ ] **新建空白迭代条目**
- [ ] **处理运行数据文件**：
   - **归档保留**（添加重置分隔线，不清空）：
     - `tasks/coo/audit-log.md`
     - `tasks/shared/session-reports/*.md`（所有 7 个 Agent 文件）
   - **清空恢复模板**：
     - `tasks/planner/observations.md`
     - `tasks/coordinator/queue.md` + `assignments.md`
     - `tasks/workers/status.md` + `branches.md`（清空分支记录）
     - `tasks/pr-manager/pr-queue.md`
     - `tasks/maintainer/improvements.md`（改进状态改为 proposed 或删除已完成项）
     - `tasks/housekeeper/cleanup-queue.md`（保留清理历史）
- [ ] **不触碰制度文件**：instructions.md（所有 Agent）、ARCHITECTURE.md、AGENTS.md、SYSTEM-MANIFEST.md、decisions.md、SKILL.md 等跨迭代制度文件保持不变
- [ ] 输出："✅ 系统已重置，可以重新唤醒 Planner 开始新迭代"

### 完成后校验（必须执行）

重置完成后，逐项校验以下内容，确保无遗漏：

- [ ] iteration-log.md 当前迭代号 = agent-status.md 当前迭代号
- [ ] 所有 inbox 已清空（7 个文件）
- [ ] observations.md 已恢复模板
- [ ] queue.md + assignments.md 已恢复模板
- [ ] audit-log.md 有重置分隔线且历史保留
- [ ] session-reports/*.md 有重置分隔线且历史保留
- [ ] 制度文件（instructions.md/ARCHITECTURE.md/AGENTS.md/SYSTEM-MANIFEST.md/decisions.md）未被修改
- [ ] git commit 成功

如有任何项未通过，立即补执行对应步骤。

### 选择性重置

> ⚠️ 以上选择性命令同样属于白名单，精确匹配才触发，同样需要二次确认。

| 命令 | 操作 |
|------|------|
| "执行系统重置" | 完全重置（默认） |
| "只重置任务看板" | 只重置 agent-status 的任务区 |
| "只归档当前迭代" | 只标记当前迭代为已废弃 |
| "只清空inbox" | 只清空所有收件箱 |

### 安全规则

1. **永远不删除Git历史** — 重置只是恢复文件内容到模板状态
2. **保留cleanup-queue的清理历史**
3. **保留logs/目录的日志文件**
4. **重置前必须告知用户将要做什么**
5. **保留audit-log的制度记忆** — 审计日志是跨迭代的制度记忆，重置时只添加分隔线标注，不清空
6. **不修改制度文件** — instructions.md、ARCHITECTURE.md、AGENTS.md、SYSTEM-MANIFEST.md、decisions.md 等是跨迭代的制度记忆，重置时不应修改

---

## .git 损坏应急协议

### 预防措施：使用 Worktree 隔离

多个 Worker 同时操作同一个 git 仓库会导致 .git 损坏。预防方法：

- Worker 使用 `git worktree add` 创建独立工作目录
- 主仓库永远保持在 main 分支
- 详见 `git-workflow.md#Worker Worktree 工作流`

当 `git` 命令报错（如 `fatal: not a git repository`、`corrupt`、`index.lock` 等）时：

### 修复步骤

1. **停止所有写操作** — 不要继续执行任务
2. **诊断损坏程度**：
   - `git status` — 是否能读？
   - `git log --oneline -1` — 历史是否完整？
   - `git fsck` — 检查损坏详情
3. **尝试修复**：
   - `index.lock` 残留 → `rm .git/index.lock`
   - 轻微损坏 → `git fsck --full` 按提示修复
   - HEAD 损坏 → `git reset --soft HEAD~1` 撤销提交（不丢失工作区）
4. **严重损坏（无法修复）** → 从远程 clone 恢复：
   - `git clone https://github.com/bigmanBass666/claw-code-rust.git [临时目录]`
   - 将临时目录的 `.git` 复制到原仓库：`Copy-Item [临时目录]/.git [原仓库]/.git`
   - 回到原仓库：`git reset --hard HEAD`
5. **修复后验证** → `git status` + `git log --oneline -3` 确认正常

### 安全规则

- **不要尝试 `git push --force`** — 可能覆盖远程数据
- **不要删除 .git 目录** — 除非用户明确指示
- **修复后立即 commit + push** — 确保当前工作不再次丢失
