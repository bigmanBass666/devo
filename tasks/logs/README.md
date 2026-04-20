# ValveOS 运行日志系统

> 记录多 Agent 协作系统的所有运行日志

## 日志文件

| 文件 | 记录内容 |
|------|----------|
| `system-commands.log` | 用户触发的系统命令记录（INPUT/RESPONSE） |
| `system.log` | 系统级事件（启动、重置、配置变更） |
| `planner.log` | Planner 的决策和观察记录 |
| `coordinator.log` | Coordinator 的协调和分配记录 |
| `workers.log` | 所有 Worker 的执行记录 |
| `pr-manager.log` | PR Manager 的 PR 处理记录 |
| `maintainer.log` | Maintainer 的改进建议和执行记录 |
| `housekeeper.log` | Housekeeper 的分支清理记录 |
| `coo.log` | COO 的审计和系统维护记录 |

## 日志格式

### 统一格式
```
[YYYY-MM-DD HH:MM:SS] [AGENT_ID] [LEVEL] MESSAGE
  - detail: ...
  - data: { ... }
```

### LEVEL 说明

**基础级别**：
- **INFO** — 正常操作
- **WARN** — 警告（如冲突、等待）
- **ERROR** — 错误（如任务失败）
- **DECISION** — 决策点（Planner/Maintainer）

**系统命令日志级别**：
- **INPUT** — 用户输入触发系统命令
- **RESPONSE** — Agent 处理完成并响应

**ValveOS 特有事件**：
- **WAKEUP** — Agent被唤醒（含醒来后读取的文件列表）
- **RESUME** — 断点续传（发现上次进度、任务有效性判断）
- **MESSAGE** — Inbox消息读写（发送者/接收者/内容摘要）
- **RESET** — 系统重置（完全/选择性）
- **ITERATION** — 迭代生命周期（开始/暂停/完成/废弃）
- **LOOKUP** — 功能索引跳板（查了什么、从哪查的）

### 事件类型使用示例

```markdown
# 唤醒协议
[2026-04-19 10:00:00] [Planner] [WAKEUP] 被用户唤醒
  - detail: 开始醒来协议，读取inbox+agent-status+iteration-log
  - data: { "files_read": ["inbox/planner.md", "agent-status.md", "iteration-log.md"] }

# 系统命令日志
[2026-04-20 09:15:00] [INPUT] "看看状态" → 触发 查看状态 命令
[2026-04-20 09:15:00] [RESPONSE] COO 处理，输出 7 个 Agent 状态
[2026-04-20 14:30:00] [INPUT] "从头开始" → 触发 系统重置 命令
[2026-04-20 14:30:01] [RESPONSE] 完全重置，清空 6 个 inbox

# 断点续传
[2026-04-19 10:01:00] [Planner] [RESUME] 发现上次迭代进度
  - detail: Iteration 2 暂停，4个任务pending，评估后标记为stale
  - data: { "iteration": 2, "tasks_found": 4, "valid": 0, "action": "mark_stale" }

# Inbox通信
[2026-04-19 10:15:00] [Planner] [MESSAGE] 写入Coordinator inbox
  - detail: 任务下发：4个任务待分配
  - data: { "target": "coordinator", "task_count": 4, "plan": "2026-04-18-iteration-2" }

# 功能跳板
[2026-04-19 11:00:00] [Worker] [LOOKUP] 查阅功能索引
  - detail: 需要了解Inbox写入格式，跳转到cli-operations.md#Agent协作操作
  - data: { "lookup": "Inbox读写格式", "source": "cli-operations.md" }

# 系统重置
[2026-04-19 12:00:00] [Coordinator] [RESET] 执行系统重置
  - detail: 完全重置，清空所有inbox+归档Iteration3
  - data: { "mode": "full", "inboxes_cleared": 6, "iteration_archived": 3 }

# 迭代管理
[2026-04-19 10:30:00] [Planner] [ITERATION] 开始新迭代
  - detail: Iteration 4 开始，目标待制定
  - data: { "iteration": 4, "status": "started", "trigger": "user" }
```

---

## 使用说明

1. 每个 Agent 在执行重要操作时，追加到对应的日志文件
2. 日志是**追加式**的，不删除历史记录
3. Maintainer 定期分析这些日志，提出改进建议
4. 日志文件**不纳入Git追踪**（`.gitignore` 中已排除），避免仓库膨胀
5. 保留本 README.md 以说明日志格式

## Maintainer 分析维度

Maintainer 分析日志时应关注：

| 维度 | 关注点 |
|------|--------|
| 唤醒效率 | WAKEUP到首次操作的耗时 |
| 消息流转 | MESSAGE的发送/接收是否匹配 |
| 断点续传 | RESUME中任务有效性判断准确率 |
| 重置频率 | RESET触发的频次和原因 |
| 迭代健康 | ITERATION完成率 vs 废弃率 |
| 跳板命中率 | LOOKUP后的操作成功率 |
