# Git 分支记录

此文件记录每个 Worker Agent 工作的 Git 分支，防止分支冲突。

## 分支分配规则

- 每个 Worker 使用独立的分支
- 分支命名格式: `agent/<worker-id>/<简短描述>`
- **必须基于 `upstream/main` 创建**（保证 diff 天然干净）
- 避免与他人分支冲突

## 分支列表

| Agent ID | 分支名 | 任务 ID | 创建时间 | 状态 |
|----------|--------|---------|----------|------|
| | | | | |

---

## ValveOS 工作流（当前）

Worker 完成任务后的交接流程：

1. Worker 完成编码 + 测试 + commit + push 到 origin
2. Worker 写消息到 **PR Manager 的 inbox**（通知完成）
3. **PR Manager** 被唤醒后：
   - 从 agent/ 分支 cherry-pick 相关 commit 到新的 feat/ 分支（基于 upstream/main）
   - 执行质量检查（fmt / clippy / test）
   - 生成 PR 描述，等待用户审批
4. 用户批准 → PR 提交到上游
5. PR 合并后 → PR Manager 通知 Housekeeper 清理分支

> ⚠️ **Coordinator 不再做代码审查或 merge**。代码质量由 PR Manager 通过自动化检查保障。

## 使用说明

1. Worker 开始任务前，在此表中登记分支信息
2. 分支创建命令: `git checkout -b agent/xxx/description upstream/main`
3. 任务完成后 push 到 origin，然后通过 inbox 通知 PR Manager
4. 分支清理由 Housekeeper 在 PR 合并后执行

---

## 冲突检测

<!-- 记录任何分支冲突或问题 -->
