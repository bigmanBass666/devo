# 任务队列

此文件由 **Planner** 下发任务，由 **Coordinator** 消费。

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

<!-- Planner 下发的新任务列在这里 -->

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
