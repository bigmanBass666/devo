# Agent 状态与任务追踪

> 用户是阀门，只有被唤醒的Agent才能执行
> Agent之间通过 inbox 文件传递消息

## Agent 状态

| Agent | 最近活跃 | 当前状态 | 等待唤醒 |
|-------|----------|----------|----------|
| Planner | - | 未启动 | 用户手动 |
| Coordinator | - | 未启动 | 用户 |
| Worker-001 | - | 未启动 | Coordinator |
| Worker-002 | - | 未启动 | Coordinator |
| Worker-003 | - | 未启动 | Coordinator |
| PR Manager | - | 未启动 | Worker |
| Maintainer | - | 未启动 | 自动触发(3任务/24h/连续失败) |
| Housekeeper | - | 未启动 | PR合并后/24h安全网 |
| COO | - | 未启动 | - |

### 审批类任务格式

> 当任务需要用户审批时（如关闭 PR/Issue、回复评论），负责人列必须使用完整格式：

| 格式 | 示例 |
|------|------|
| `需用户审批（原因：XXX；操作：YYY）` | `需用户审批（原因：评论属社交边界；操作：在PR#42评论请求关闭）` |
| `需用户审批（原因：XXX；操作：YYY）` | `需用户审批（原因：关闭Issue需社区确认；操作：在Issue#36/35评论确认已解决）` |

⚠️ 禁止只写 `需用户审批` —— 必须包含原因和操作指引。

## 全局任务看板

> 追踪所有任务的完整生命周期
> 任务状态: pending / in_progress / completed / blocked / failed / stale

### 当前迭代: Iteration 10

| 任务ID | 描述 | 状态 | 负责人 | 优先级 | 创建时间 |
|--------|------|------|--------|--------|----------|

### 已废弃迭代

#### Iteration 9 (已废弃 — 2026-04-20 系统重置)

| 任务ID | 描述 | 状态 | 负责人 | 优先级 |
|--------|------|------|--------|--------|
| TASK-016 | 同步 local main 到 upstream/main | pending | Coordinator→Worker | P0 |
| TASK-017 | 关闭 PR #42、Issue #36、Issue #35 | pending | 需用户审批 | P1 |
| TASK-018 | 清理 origin/feat/fix-windows-unc-path 旧分支 | pending | Worker | P2 |
| TASK-019 | 评估上游 Issue 贡献可行性 | pending | Worker | P2 |

#### Iteration 8 (已废弃 — 2026-04-20 系统重置)

| 任务ID | 描述 | 状态 | 负责人 | 优先级 |
|--------|------|------|--------|--------|
| - | - | - | - | - |

#### Iteration 7 (已废弃 — 2026-04-20 系统重置)

| 任务ID | 描述 | 状态 | 负责人 | 优先级 |
|--------|------|------|--------|--------|
| TASK-013 | 同步本地 main 到 upstream/main | completed | Planner | P0 |
| TASK-014 | 关闭 PR #42 和 Issue #36 | pending | 需用户审批 | P1 |
| TASK-015 | 清理 origin/feat/fix-windows-unc-path 旧分支 | pending | Worker | P2 |

#### Iteration 6 (已废弃 — 2026-04-20 系统重置)

| 任务ID | 描述 | 状态 | 负责人 | 优先级 |
|--------|------|------|--------|--------|
| TASK-009 | 配置 upstream 远程仓库 | completed | Coordinator | P0 |
| TASK-010 | 修复 CJK 文本 panic（Issue #36） | blocked | Worker-001 | P0 |
| TASK-011 | 重新提取 Windows Unc path 修复为干净分支 | blocked | Worker-002 | P1 |
| TASK-012 | 清理远程分支 | completed | Worker-003 | P2 |

---

## 唤醒历史

| 时间 | 被唤醒者 | 唤醒原因 | 结果 |
|------|----------|----------|------|
| 2026-04-20 | 系统 | 系统重置 | 全部Agent回到未启动，Iteration 10 就绪 |

## 使用说明

1. Agent被用户唤醒后，更新状态为"活跃"
2. Agent完成后，写入消息到目标Agent的inbox
3. Agent告知用户下一步该唤醒谁
4. 更新"等待唤醒"列和任务状态
5. 新会话醒来时先读此文件了解全局状态
