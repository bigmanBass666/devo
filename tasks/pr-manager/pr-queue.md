# PR 待处理队列

此文件记录等待 PR Manager 处理的任务。

## 队列状态

- `pending` — 待处理
- `checking` — 正在检查
- `ready` — 检查通过，等待用户审批
- `needs_fix` — 需要修复
- `submitted` — 已提交 PR

## 待处理任务

<!-- 新任务追加到这里 -->

## 进行中的任务

<!-- 正在处理的任务 -->

## 已完成任务

<!-- 已完成的任务 -->

---

## 任务记录格式

```markdown
### [TASK-ID] <任务标题>
- **Worker**: Worker-XXX
- **分支**: agent/worker-xxx/task-yyy
- **优先级**: P0/P1/P2/P3
- **状态**: pending/checking/ready/needs_fix/submitted
- **创建时间**: YYYY-MM-DD HH:MM
- **处理时间**: YYYY-MM-DD HH:MM
- **PR 分支**: feat/xxx (如果有)
- **PR 编号**: #XXX (如果有)
```

---

## 使用说明

1. Coordinator 通知任务完成后，在此添加记录
2. PR Manager 开始处理后，更新状态为 `checking`
3. 检查通过后，更新状态为 `ready`
4. 用户批准后，更新状态为 `submitted`
5. 定期清理已完成的历史记录

