# Agent 状态与任务追踪

> 用户是阀门，只有被唤醒的Agent才能执行
> Agent之间通过 inbox 文件传递消息

## Agent 状态

| Agent | 最近活跃 | 当前状态 | 等待唤醒 |
|-------|----------|----------|----------|
| Planner | 2026-04-19 | 沉睡 | 用户手动 |
| Coordinator | 2026-04-19 | 沉睡(轮询实验结束) | Planner |
| Worker-001 | 2026-04-19 | 沉睡(轮询实验结束) | Coordinator |
| Worker-002 | 2026-04-19 | 沉睡(轮询实验结束) | Coordinator |
| Worker-003 | 2026-04-19 | 待机(等Coordinator消息) | Coordinator |
| PR Manager | - | 未启动 | Worker |
| Maintainer | - | 未启动 | 自动触发(3任务/24h/连续失败) |
| Housekeeper | - | 未启动 | PR合并后/24h安全网 |

## 全局任务看板

> 追踪所有任务的完整生命周期
> 任务状态: pending / in_progress / completed / blocked / failed / stale

### 当前迭代: Iteration 5

| 任务ID | 描述 | 状态 | 负责人 | 优先级 | 创建时间 |
|--------|------|------|--------|--------|----------|
| TASK-005 | 配置 upstream 远程仓库 | pending | 待分配 | P0 | 2026-04-19 |
| TASK-006 | 评估并清理远程分支 | pending | 待分配 | P1 | 2026-04-19 |
| TASK-007 | 提交 feat/fix-log-level-prompt-mode PR | pending | 待分配 | P1 | 2026-04-19 |
| TASK-008 | 重新提取 Windows UNC path 修复 | pending | 待分配 | P2 | 2026-04-19 |

---

## 唤醒历史

| 时间 | 被唤醒者 | 唤醒原因 | 结果 |
|------|----------|----------|------|
| 2026-04-19 | Planner | 用户手动唤醒 | 制定Iteration 5计划，4任务下发 |

## 使用说明

1. Agent被用户唤醒后，更新状态为"活跃"
2. Agent完成后，写入消息到目标Agent的inbox
3. Agent告知用户下一步该唤醒谁
4. 更新"等待唤醒"列和任务状态
5. 新会话醒来时先读此文件了解全局状态
