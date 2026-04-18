# 分支清理队列

此文件记录待 Housekeeper 执行的分支清理任务。

## 队列状态

- `pending` — 待处理
- `auto-clean` — 确认安全，自动清理
- `needs-confirm` — 需要确认
- `done` — 已清理
- `skipped` — 跳过

---

## 待处理任务

<!-- PR Manager 或其他 Agent 添加的任务 -->

---

## 自动清理（已确认安全）

| 分支名 | 原因 | 添加时间 | 状态 |
|--------|------|----------|------|
| | | | |

---

## 需要确认

| 分支名 | 原因 | 添加时间 | 建议动作 |
|--------|------|----------|----------|
| | | | |

---

## 最近清理记录

| 分支名 | 清理时间 | 原因 |
|--------|----------|------|
| feat/null-array-fix-v2 | 2026-04-18 | PR #40 已合并 |

---

## 使用说明

1. **PR Manager** 在 PR 合并后，将 `feat/` 分支添加到"自动清理"区
2. **Planner/其他 Agent** 发现过期分支，添加到"需要确认"区
3. **Housekeeper** 读取此文件，执行清理
4. 清理完成后，从队列移除并添加到"最近清理记录"

---

## 格式模板

```markdown
### [BRANCH-XXX] <分支名>
- **原因**: PR #XX 已合并 / 超过 N 天无更新
- **类型**: auto-clean / needs-confirm
- **添加时间**: YYYY-MM-DD HH:MM
- **添加者**: PR Manager / Planner / Maintainer
- **状态**: pending / done / skipped
```
