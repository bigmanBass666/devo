# Git 分支记录

此文件记录每个 Worker Agent 工作的 Git 分支，防止分支冲突。

## 分支分配规则

- 每个 Worker 使用独立的分支
- 分支命名格式: `agent/<worker-id>/<简短描述>`
- 避免与他人分支冲突

## 分支列表

| Agent ID | 分支名 | 任务 ID | 创建时间 | 状态 |
|----------|--------|---------|----------|------|
| | | | | |

---

## 合并规则

1. Worker 完成任务后，通知 Coordinator
2. Coordinator 审查后决定是否合并
3. 合并前确保没有冲突
4. 合并使用 `git merge --no-ff` 保留分支历史
5. 已合并的分支可以删除

---

## 使用说明

1. Worker 开始任务前，在此表中登记分支信息
2. 分支创建命令: `git checkout -b agent/xxx/description upstream/main`
3. 任务完成后，通知 Coordinator 进行代码审查和合并
4. 已合并的分支更新状态为 `merged`，可以删除本地分支

---

## 冲突检测

<!-- 记录任何分支冲突或合并问题 -->

