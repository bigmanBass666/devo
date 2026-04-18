# 任务队列

此文件由 **Planner** 下发任务，由 **Coordinator** 消费。

> **双通道说明**：Coordinator 主要通过 **inbox** (`tasks/shared/inbox/coordinator.md`) 接收 Planner 的任务分配和指令。本 queue.md 作为**结构化任务看板**，提供更详细的任务元数据（优先级、依赖、截止时间）。两者互补，不冲突。

## 任务状态

- `pending` — 待处理
- `in_progress` — 进行中
- `completed` — 已完成
- `blocked` — 被阻塞

## 优先级

- `P0` — 紧急，影响核心功能
- `P1` — 重要，待审核的 PR、关键改进
- `P2` — 一般，代码优化、文档
- `P3` — 低，长期改进、探索

---

## 待处理任务

### TASK-005: 配置 upstream 远程仓库
- **优先级**: P0
- **描述**: 添加 upstream 远程仓库指向 `https://github.com/claw-cli/claw-code-rust.git`，并 fetch 上游最新代码。当前 `git remote -v` 只有 origin，没有 upstream。这是 ValveOS 工作流的基础——没有 upstream 就无法创建基于 upstream/main 的干净分支。
- **期望结果**: `git remote -v` 显示 upstream，`git fetch upstream` 成功
- **值得提 PR**: 否（本地配置）
- **依赖**: 无
- **状态**: pending
- **分配给**: Worker-001
- **创建时间**: 2026-04-19
- **更新时间**: 2026-04-19

### TASK-006: 评估并清理远程分支
- **优先级**: P1
- **描述**: 检查6个远程分支的合并状态，删除已合并/过时的分支。具体评估：
  - `dev/wang` — 已合并到上游，删除
  - `test-mcp-branch` — 已合并，删除
  - `dev/tools0412` — 评估是否已合并，如是则删除
  - `feat/clippy-fixes` — 评估是否已合并到上游，如是则删除
  - `feat/fix-log-level-prompt-mode` — **保留**，准备提PR
  - `feat/fix-windows-unc-path` — **保留**，需重新提取
- **期望结果**: 只保留有价值的分支，仓库整洁
- **值得提 PR**: 否（分支清理）
- **依赖**: TASK-005（需要 upstream 来判断合并状态）
- **状态**: pending
- **分配给**: Worker-001
- **创建时间**: 2026-04-19
- **更新时间**: 2026-04-19

### TASK-007: 提交 feat/fix-log-level-prompt-mode PR
- **优先级**: P1
- **描述**: 将 `feat/fix-log-level-prompt-mode` 分支的修复提交为 PR 到上游。这是一个有效的 bug fix：`--log-level` 在 prompt mode 下不工作。分支只有1个有效commit（`e56948c fix: make --log-level work in prompt mode`），非常干净。
- **期望结果**: PR 已提交到上游，描述清晰
- **值得提 PR**: 是
- **依赖**: TASK-005（需要 upstream 来创建/推送分支）
- **注意**: 需要先检查上游是否已有相关 issue
- **状态**: pending
- **分配给**: Worker-001
- **创建时间**: 2026-04-19
- **更新时间**: 2026-04-19

### TASK-008: 重新提取 Windows UNC path 修复
- **优先级**: P2
- **描述**: `feat/fix-windows-unc-path` 分支包含有效修复（strip Windows UNC prefix from canonicalized CLAWCR_HOME path），但分支不干净（混入了其他commit如 author attitude tracking、notification system 等）。需要基于 upstream/main 创建新的干净分支，cherry-pick 仅 UNC path 修复的 commit（`35dab7b` 或 `3eac63d`）。
- **期望结果**: 新的干净 feat/ 分支，只包含 UNC path 修复
- **值得提 PR**: 是
- **依赖**: TASK-005
- **状态**: pending
- **分配给**: Worker-001
- **创建时间**: 2026-04-19
- **更新时间**: 2026-04-19

---

## 进行中任务

<!-- 正在执行的任务 -->

---

## 已完成任务

<!-- 已完成的任务 -->

---

## 任务记录格式

```markdown
### [TASK-ID] 任务标题
- **优先级**: P0/P1/P2/P3
- **描述**: 详细描述
- **期望结果**: 完成标准
- **截止时间**: YYYY-MM-DD（可选）
- **依赖**: TASK-XXX（如果有）
- **状态**: pending/in_progress/completed/blocked
- **分配给**: Coordinator/Worker-XXX
- **创建时间**: YYYY-MM-DD HH:MM
- **更新时间**: YYYY-MM-DD HH:MM
```

---

## 使用说明

1. **Planner** 将新任务写入此文件，放在"待处理任务"区
2. **Coordinator** 从此文件读取任务，消费后移动到"进行中"
3. 任务完成后，**Coordinator** 更新状态并移到"已完成"
4. 如果阻塞，更新状态为 `blocked` 并说明原因
