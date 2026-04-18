# 运行日志系统

此目录记录多 Agent 协调系统的所有运行日志。

## 日志文件

| 文件 | 记录内容 |
|------|----------|
| `system.log` | 系统级事件（启动、关闭、配置变更） |
| `planner.log` | Planner 的决策和观察记录 |
| `coordinator.log` | Coordinator 的协调和分配记录 |
| `workers.log` | 所有 Worker 的执行记录 |
| `pr-manager.log` | PR Manager 的 PR 处理记录 |
| `maintainer.log` | Maintainer 的改进建议和执行记录 |

## 日志格式

### 统一格式
```
[YYYY-MM-DD HH:MM:SS] [AGENT_ID] [LEVEL] MESSAGE
  - detail: ...
  - data: { ... }
```

### LEVEL 说明
- **INFO** — 正常操作
- **WARN** — 警告（如冲突、等待）
- **ERROR** — 错误（如任务失败）
- **DECISION** — 决策点（Planner/Maintainer）

---

## 使用说明

1. 每个 Agent 在执行重要操作时，追加到对应的日志文件
2. 日志是**追加式**的，不删除历史记录
3. Maintainer 定期分析这些日志，提出改进建议
4. 日志文件纳入 Git 追踪，用于长期分析
