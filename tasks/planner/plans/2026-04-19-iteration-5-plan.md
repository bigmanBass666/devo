# 任务计划 — Iteration 5

> 日期: 2026-04-19
> 触发: 用户唤醒 Planner
> 状态: 执行中

---

## 当前项目状态总结

### 健康
- ✅ 编译通过 (`cargo check --workspace --all-targets`)
- ✅ 全部测试通过 (247 tests, 0 failures)
- ✅ 工作树干净 (nothing to commit)

### 问题
- ❌ **upstream 远程仓库未配置** — `git remote -v` 只有 origin，没有 upstream
- ⚠️ 6个远程分支待评估清理
- ⚠️ 代码中有 10 个 TODO/FIXME 标记

### 分支状态
| 分支 | 最新commit | 评估 |
|------|-----------|------|
| `feat/clippy-fixes` | `5626cde` chore: fix clippy warnings | 可能已过时，需检查是否已合并到上游 |
| `feat/fix-log-level-prompt-mode` | `e56948c` fix: make --log-level work in prompt mode | 有效修复，可能需要提PR |
| `feat/fix-windows-unc-path` | `7025296` 含UNC修复+杂项commit | 不干净，需重新提取 |
| `dev/tools0412` | `bc54e57` 开发分支 | 旧开发分支，可能可清理 |
| `dev/wang` | `78d6357` 旧开发分支 | 已合并，可清理 |
| `test-mcp-branch` | `3200938` 测试分支 | 已合并，可清理 |

---

## 识别的问题/机会

1. **upstream 未配置** — ValveOS 工作流的基础，无法创建干净分支
2. **远程分支堆积** — 6个分支中至少3个已合并可清理
3. **feat/fix-log-level-prompt-mode** — 看起来是一个有效的 bug fix，值得提 PR
4. **feat/fix-windows-unc-path** — 包含有效修复但分支不干净，需要重新提取
5. **TODO 标记** — 10个 TODO 需要评估哪些值得处理

---

## 任务列表

### TASK-005: 配置 upstream 远程仓库
- **优先级**: P0
- **描述**: 添加 upstream 远程仓库指向 `https://github.com/claw-cli/claw-code-rust.git`，并 fetch 上游最新代码
- **期望结果**: `git remote -v` 显示 upstream，`git fetch upstream` 成功
- **值得提 PR**: 否（本地配置）
- **依赖**: 无

### TASK-006: 评估并清理远程分支
- **优先级**: P1
- **描述**: 检查6个远程分支的合并状态，删除已合并/过时的分支。具体：
  - `dev/wang` — 已合并，删除
  - `test-mcp-branch` — 已合并，删除
  - `dev/tools0412` — 评估是否已合并，如是则删除
  - `feat/clippy-fixes` — 评估是否已合并到上游，如是则删除
  - `feat/fix-log-level-prompt-mode` — 保留，准备提PR
  - `feat/fix-windows-unc-path` — 保留，需重新提取
- **期望结果**: 只保留有价值的分支，仓库整洁
- **值得提 PR**: 否（分支清理）
- **依赖**: TASK-005（需要 upstream 来判断合并状态）

### TASK-007: 提交 feat/fix-log-level-prompt-mode PR
- **优先级**: P1
- **描述**: 将 `feat/fix-log-level-prompt-mode` 分支的修复提交为 PR 到上游。这是一个有效的 bug fix：`--log-level` 在 prompt mode 下不工作
- **期望结果**: PR 已提交到上游，描述清晰
- **值得提 PR**: 是
- **依赖**: TASK-005（需要 upstream 来创建/推送分支）
- **注意**: 需要先检查上游是否已有相关 issue

### TASK-008: 重新提取 Windows UNC path 修复
- **优先级**: P2
- **描述**: `feat/fix-windows-unc-path` 分支包含有效修复（strip Windows UNC prefix），但分支不干净（混入了其他commit）。需要基于 upstream/main 创建新的干净分支，cherry-pick 仅 UNC path 修复的 commit
- **期望结果**: 新的干净 feat/ 分支，只包含 UNC path 修复
- **值得提 PR**: 是
- **依赖**: TASK-005

---

## 任务依赖关系

```
TASK-005 (配置upstream) ← 无依赖，最先执行
    ↓
TASK-006 (清理分支) ← 依赖 TASK-005
TASK-007 (log-level PR) ← 依赖 TASK-005
TASK-008 (UNC path 提取) ← 依赖 TASK-005
```

TASK-006、007、008 可并行执行（互不依赖）

---

## 执行策略

1. **先做 TASK-005**（配置 upstream）— 这是所有后续任务的基础
2. **并行做 TASK-006 + TASK-007 + TASK-008**：
   - TASK-006（分支清理）和 TASK-007（log-level PR）可由不同 Worker 并行
   - TASK-008（UNC path 提取）需要 cherry-pick 操作，独立进行

---

## PR 可行性评估

| 任务 | 对上游有价值 | 符合贡献规范 | PR大小合理 | 有相关issue | 评估 |
|------|------------|------------|-----------|------------|------|
| TASK-007 | ✅ bug fix | ✅ 小而专注 | ✅ 1 commit | ❓ 需检查 | **推荐提PR** |
| TASK-008 | ✅ Windows修复 | ✅ 小而专注 | ✅ 1-2 files | ❓ 需检查 | **推荐提PR** |
| TASK-005 | N/A 本地配置 | N/A | N/A | N/A | 不提PR |
| TASK-006 | N/A 分支清理 | N/A | N/A | N/A | 不提PR |

---

## 预期结果

1. upstream 远程仓库配置完成
2. 无用分支清理完成
3. 1-2个有效 PR 提交到上游
4. 项目基础设施就绪，后续迭代可顺畅运行
