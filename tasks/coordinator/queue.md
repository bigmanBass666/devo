# 任务队列

此文件由 **Planner** 下发任务，由 **Coordinator** 消费。

> **双通道说明**：Coordinator 主要通过 **inbox** (`tasks/shared/inbox/coordinator.md`) 接收 Planner 的任务分配和指令。本 queue.md 作为**结构化任务看板**，提供更详细的任务元数据（优先级、依赖、截止时间）。两者互补，不冲突。

## 任务状态

- `pending` — 待处理
- `in_progress` — 进行中
- `completed` — 已完成
- `blocked` — 被阻塞
- `frozen` — 冻结/归档

## 优先级

- `P0` — 紧急，影响核心功能
- `P1` — 重要，待审核的 PR、关键改进
- `P2` — 一般，代码优化、文档
- `P3` — 低，长期改进、探索

---

## 待处理任务

### TASK-ITER11-001: 提交工作区清理
- **描述**: 工作区有 4 个文件更改未提交：observations.md 更新、planner inbox 更新、删除的 rehearsal-plan.md、删除的 rehearsal2-plan.md。需提交清理提交。
- **状态**: pending
- **优先级**: P1
- **负责人**: Coordinator → Worker
- **操作步骤**:
  1. Coordinator 分配任务给 Worker
  2. Worker 执行 git add + commit（分支：main）
  3. Worker 执行 git push origin main

### TASK-ITER11-002: 归档 Iteration 10 冻结任务
- **描述**: Iteration 10 的任务已过时（TASK-ITER10-002 同步方向已反：实为 origin 领先 upstream）。需归档并更新队列。
- **状态**: pending
- **优先级**: P1
- **负责人**: Planner

### TASK-ITER11-003: 分析 upstream FETCH_HEAD 新提交
- **描述**: upstream FETCH_HEAD 包含 9 个新提交：PR #42 UNC prefix 修复、PR #40 null数组修复、PR #37 prompt子命令等。评估是否值得向 ValveOS 回迁。
- **状态**: pending
- **优先级**: P2
- **负责人**: Coordinator → Worker
- **产出**: 评估报告写入 tasks/planner/backlog.md

### TASK-ITER11-004: 调查测试失败
- **描述**: 测试 `tests::slash_model_with_argument_in_inline_mode_updates_status_without_transcript_note` 失败。预期 "Model set to test-model"，实际 "Failed to switch model"。inline_mode=true 时 model switch 失败。
- **状态**: pending
- **优先级**: P0
- **负责人**: Coordinator → Worker
- **分析方向**: 检查 test_app 是否缺少 mock config，或检查 inline_mode 下 model switch 逻辑是否缺陷

---

## 进行中任务

<!-- 正在执行的任务 -->

### TASK-ITER11-007: 评估 upstream PR 应用策略（sync-upstream 工作流）
- **描述**: upstream main (82e2d40) 领先 origin/main 8个提交，对应 PR #42/#40/#37/#34/#32/#33/#38/#39。ValveOS 是独立 fork，origin 领先 196 个 ValveOS 特有提交。Worker 需分析这 8 个 upstream PR 是否值得回迁到 ValveOS，以及如何处理冲突。
- **状态**: pending
- **优先级**: P1
- **负责人**: Coordinator → Worker
- **产出**: 评估报告写入 tasks/planner/backlog.md，包含：每个 PR 的价值评级（高/中/低）、冲突风险、建议操作（回迁/忽略/本地化）

---

## 已完成任务

### TASK-ITER10-002: 同步 upstream/main → origin/main
- **状态**: frozen（归档）
- **原因**: 方向已反，origin 实际领先 upstream，非 ValveOS 目标

### TASK-ITER10-003: 清理未追踪的 test/ 目录
- **状态**: frozen（归档）
- **原因**: test/ 已通过 .gitignore 处理
