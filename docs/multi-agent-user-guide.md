# 多 Agent 协作系统 — 用户使用指南

> 让多个 AI 同时为你工作，自主协作、自动管理。

---

## 🎯 这个系统是什么？

一个**六层 AI 协作系统**，让多个 AI Agent 自动分工合作，完成项目任务并提交干净的 PR。

**你只需要做旁观者，必要时介入。**

---

## 🏗️ 六层架构

```
┌─────────────────────────────────────────────────────────┐
│                     你（最高领导人）                      │
│              旁观者，必要时介入审批                        │
└─────────────────────────────────────────────────────────┘
                           ↓
┌─────────────────────────────────────────────────────────┐
│  Planner（决策者）— 决定做什么                           │
│  观察项目、分析需求、制定计划                             │
└─────────────────────────────────────────────────────────┘
                           ↓
┌─────────────────────────────────────────────────────────┐
│  Coordinator（管理员）— 协调怎么做                      │
│  分配任务、管理冲突、分配 Worker                         │
└─────────────────────────────────────────────────────────┘
                           ↓
┌─────────────────────────────────────────────────────────┐
│  Worker（工人）— 具体执行                               │
│  编写代码、测试、提交                                   │
└─────────────────────────────────────────────────────────┘
                           ↓
┌─────────────────────────────────────────────────────────┐
│  PR Manager（PR 管理员）— 产出干净 PR                   │
│  提取代码、质量检查、准备 PR                            │
└─────────────────────────────────────────────────────────┘
                           ↓
┌─────────────────────────────────────────────────────────┐
│  Maintainer（维护者）— 持续改进系统                     │
│  分析日志、发现问题、提出改进                            │
└─────────────────────────────────────────────────────────┘
                           ↓
┌─────────────────────────────────────────────────────────┐
│  Housekeeper（仓库守护）— 清理分支                       │
│  自动删除已合并的分支、保持仓库整洁                      │
└─────────────────────────────────────────────────────────┘
```

---

## 🚀 快速开始

### 方式一：开一个"总指挥"会话

在 Trae IDE 新开一个会话，告诉它：

> "你是总指挥。请读取 `tasks/ARCHITECTURE.md` 和 `AGENTS.md`，然后开始协调项目工作。"

系统会自动：
1. Planner 分析项目状态
2. Coordinator 分配任务
3. Worker 执行
4. PR Manager 准备 PR
5. 你审批

### 方式二：让 Planner 自主观察

> "请分析当前项目状态，制定下一步计划。"

---

## 💬 常用指令

### 作为旁观者

| 你说 | 系统做 |
|------|--------|
| "开始工作吧" | Planner 开始观察并制定计划 |
| "现在做到哪了？" | Planner 汇报当前状态 |
| "有什么可以改进的？" | Maintainer 分析并提出建议 |
| "系统有什么问题？" | Maintainer 诊断问题 |

### 介入指挥

| 你说 | 系统做 |
|------|--------|
| "暂停当前任务" | Coordinator 停止分配 |
| "优先做 X" | Planner 调整计划 |
| "这个 PR 可以提交了" | PR Manager 提交 PR |
| "批准改进 #3" | Maintainer 实施改进 |

---

## 📁 协调文件结构

```
tasks/
├── ARCHITECTURE.md       # 系统架构（所有人读）
├── planner/              # Planner 的工作区
│   ├── observations.md   # 当前观察
│   ├── plans/           # 任务计划
│   └── backlog.md       # 长期待办
├── coordinator/          # Coordinator 的工作区
│   ├── queue.md         # 任务队列
│   └── assignments.md   # 任务分配
├── workers/             # Worker 的工作区
│   ├── status.md       # Worker 状态
│   └── branches.md     # 分支记录
├── pr-manager/          # PR Manager 的工作区
│   ├── pr-queue.md     # 待处理 PR
│   └── pr-history.md   # PR 历史
├── maintainer/          # Maintainer 的工作区
│   ├── improvements.md # 改进队列
│   └── reports/        # 分析报告
├── housekeeper/         # Housekeeper 的工作区
│   └── cleanup-queue.md # 分支清理队列
└── logs/               # 日志
    ├── planner.log
    ├── coordinator.log
    └── ...
```

---

## 🔄 工作流程

### 1. 日常迭代

```
你："开始今天的工作"
    ↓
Planner 观察项目状态
    ↓
Planner 制定计划
    ↓
Coordinator 分配任务给 Worker
    ↓
Worker 执行任务
    ↓
PR Manager 准备 PR
    ↓
你审批 PR
    ↓
PR 提交到上游
    ↓
Housekeeper 清理分支
```

### 2. 自我改进

```
Maintainer 分析日志
    ↓
发现问题 → 提出改进建议
    ↓
你批准改进
    ↓
实施改进
    ↓
系统变得更好
```

---

## 🧹 分支管理

### 分支命名规则

| 类型 | 格式 | 基于 |
|------|------|------|
| 功能开发 | `agent/worker-001/fix-xxx` | upstream/main |
| PR | `feat/42-fix-bug` | upstream/main |
| 协调系统 | `agent/planner/xxx` | main |

### 自动清理

- **PR 合并后**：Housekeeper 自动删除对应的 feat/ 分支
- **过期分支**：超过 7 天的 dev/、14 天的 agent/ 分支会标记清理
- **永不删除**：main、upstream/*

---

## 📊 查看状态

### 查看当前任务

```bash
# 看任务队列
cat tasks/coordinator/queue.md

# 看 Worker 状态
cat tasks/workers/status.md

# 看 PR 进度
cat tasks/pr-manager/pr-queue.md
```

### 查看日志

```bash
# 看最近活动
cat tasks/logs/planner.log
cat tasks/logs/pr-manager.log
```

---

## 🔧 重置系统

如果需要重新开始：

1. 清理运行数据：
```bash
# 清空通知
echo '[]' > notifications/github-activity.jsonl
echo '{"last_notification_timestamp":"1970-01-01T00:00:00Z","last_read_timestamp":"1970-01-01T00:00:00Z","unread_count":0,"collected_at":"1970-01-01T00:00:00Z","summary":"No new activity"}' > notifications/github-meta.json

# 清空 tasks/ 运行数据（保留模板）
git checkout -- tasks/planner/observations.md
git checkout -- tasks/coordinator/queue.md
git checkout -- tasks/workers/status.md
# ... 其他运行文件
```

2. 提交：
```bash
git add -A
git commit -m "chore: reset system for new iteration"
git push
```

---

## ❓ 常见问题

**Q: AI 之间会冲突吗？**
A: 不会。有文件锁机制防止同时修改同一文件。

**Q: PR 会包含 AI 专用文件吗？**
A: 不会。PR 分支基于 upstream/main，天然干净。

**Q: 我需要做什么？**
A: 主要做旁观者，审批重要的 PR 和改进建议。

**Q: 系统出问题怎么办？**
A: 告诉 Maintainer "分析一下系统有什么问题"。

---

## 📚 相关文档

| 文档 | 说明 |
|------|------|
| `AGENTS.md` | 系统宪法，完整架构说明 |
| `tasks/ARCHITECTURE.md` | 详细架构文档 |
| `docs/agent-rules/` | 开发规范（Git、编码、CLI） |
| `docs/plans/` | 设计文档 |

---

**版本**：v1.0
**更新**：2026-04-18
