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
| Coordinator | 💓 Heartbeat | 7 | 2026-04-23 12:23:39 | 无新消息 |
| Worker | 💤 Standby | 4 | 2026-04-23 12:15:22 | 无新消息 |
| Planner | 💓 Heartbeat | 0 | — | 等待第一轮心跳 |
| PR Manager | 🌙 Dormant | 0 | — | — |
| Maintainer | 🌙 Dormant | 0 | — | — |
| Housekeeper | 🌙 Dormant | 0 | — | — |
| COO | 🌙 Dormant | 0 | — | — |
