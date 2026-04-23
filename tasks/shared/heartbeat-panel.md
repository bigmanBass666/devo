# 心跳控制面板

> 本面板追踪所有 Agent 的心跳状态。由 Agent 自行更新。
> 详见 `docs/agent-rules/heartbeat-protocol.md`

## 状态说明

| Emoji | 状态 | 含义 |
|-------|------|------|
| 🌙 | Dormant | 未启动心跳 |
| 💓 | Heartbeat | 轮询中，等待任务 |
| ⚡ | Working | 正在处理任务 |
| 💤 | Standby | 空闲轮询 |

## Agent 状态

| Agent | 状态 | 心跳计数 | 最后活跃时间 | 备注 |
|-------|------|---------|-------------|------|
| Coordinator | 💓 Heartbeat | 17 | 2026-04-23 10:55:17 | 轮询中，无新消息 |
| Worker | 💓 Heartbeat | 3 | 2026-04-23 | TASK-ITER11-007 已完成，等待新任务 |
| Planner | 🌙 Dormant | 0 | — | — |
| PR Manager | 🌙 Dormant | 0 | — | — |
| Maintainer | 🌙 Dormant | 0 | — | — |
| Housekeeper | 🌙 Dormant | 0 | — | — |
| COO | 🌙 Dormant | 0 | — | — |
