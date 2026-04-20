# Agent 状态与任务追踪

> 用户是阀门，只有被唤醒的Agent才能执行
> Agent之间通过 inbox 文件传递消息
>
> **ValveOS v0.2.0** — 最后更新: 2026-04-21

## Agent 状态

| Agent | 最近活跃 | 当前状态 | 等待唤醒 |
|-------|----------|----------|----------|
| Planner | 2026-04-21T03:50:00Z | 沉睡 | — |
| Coordinator | 2026-04-21T04:50:43Z | 待机 | Worker |
| Worker-001 | 2026-04-21T04:54:56Z | 活跃 | Coordinator |
| Worker-002 | - | 进行中 | — |
| Worker-003 | - | 未启动 | — |
| PR Manager | - | 未启动 | Worker |
| Maintainer | - | 未启动 | 自动触发(3任务/24h/连续失败) |
| Housekeeper | - | 未启动 | PR合并后/24h安全网 |
| COO | - | 未启动 | — |

### 审批类任务格式

> 当任务需要用户审批时（如关闭 PR/Issue、回复评论），负责人列必须使用完整格式：

| 格式 | 示例 |
|------|------|
| `需用户审批（原因：XXX；操作：YYY）` | `需用户审批（原因：评论属社交边界；操作：在PR#42评论请求关闭）` |

⚠️ 禁止只写 `需用户审批` —— 必须包含原因和操作指引。

## 全局任务看板

> 追踪所有任务的完整生命周期
> 任务状态: pending / in_progress / completed / blocked / failed / stale / frozen

### 🚀 Iteration 11: 2026-04-21 ~ 进行中

> Planner 演练模式启动

| 任务ID | 描述 | 状态 | 负责人 | 优先级 |
|--------|------|------|--------|--------|
| TASK-ITER11-001 | 提交工作区清理 | completed | Worker-001 | P1 |
| TASK-ITER11-002 | 归档 Iteration 10 冻结任务 | pending | Planner | P1 |
| TASK-ITER11-003 | 分析 upstream FETCH_HEAD 新提交 | in_progress | Worker-002 | P2 |
| TASK-ITER11-004 | 调查测试失败: slash_model_with_argument_in_inline_mode_updates_status_without_transcript_note | completed | Worker-001 | P0 |

### 历史迭代（已结束）

#### Iteration 10: 2026-04-20 ~ 冻结

> 被 Iteration 11 替代，任务已归档

| 任务ID | 描述 | 状态 | 负责人 | 优先级 |
|--------|------|------|--------|--------|
| TASK-ITER10-001 | 验证 upstream/main 同步状态 | completed | Planner | P0 |
| TASK-ITER10-002 | 同步 upstream/main → origin/main | frozen | Coordinator→Worker | P0 |
| TASK-ITER10-003 | 清理未追踪的 test/ 目录 | frozen | （.gitignore 处理） | P1 |
| TASK-ITER10-004 | 评估 query.rs TODO 并形成改进建议 | frozen | Worker | P2 |

#### Iteration 1-9: 已废弃

> 详见 iteration-log.md 历史记录

---

## 唤醒历史

| 时间 | 被唤醒者 | 唤醒原因 | 结果 |
|------|----------|----------|------|
| 2026-04-20 12:30 | 系统 | 系统重置 | Iteration 10 启动 |
| 2026-04-20 15:23 | 系统 | ValveOS 基础建设 spec | v0.2.0 就绪 |
| 2026-04-20 21:00 | 系统 | 运行时基础设施加固 | 进行中 |
| 2026-04-21 04:04 | Coordinator | 演练模式 | 任务已分配给 Worker |
| 2026-04-21 04:50 | Coordinator | 演练模式 | TASK-ITER11-004 已分配给 Worker-001 |

## 使用说明

1. Agent被用户唤醒后，更新状态为"活跃"
2. Agent完成后，写入消息到目标Agent的inbox
3. Agent告知用户下一步该唤醒谁
4. 更新"等待唤醒"列和任务状态
5. 新会话醒来时先读此文件了解全局状态
