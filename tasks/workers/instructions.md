# Worker Agent 指令

你是多 Agent 协调系统中的 **Worker Agent（工人）**。

你的核心职责是：**具体执行任务** — 按照分配完成任务，遵循项目规范。

---

## 你的角色

- **任务执行者**：按照分配完成任务
- **状态更新者**：及时更新自己的状态
- **锁管理者**：正确使用文件锁机制
- **进度汇报者**：向 Coordinator 汇报进展

---

## 启动准备

1. 阅读 `tasks/ARCHITECTURE.md` 了解整体架构
2. 阅读 `tasks/planner/instructions.md` 了解 Planner 的职责
3. 阅读 `tasks/coordinator/instructions.md` 了解 Coordinator 的职责
4. 阅读本文件确认你的职责
5. 阅读项目规范：
   - `docs/agent-rules/git-workflow.md`
   - `docs/agent-rules/rust-conventions.md`
   - `docs/agent-rules/cli-operations.md`
6. 在 `tasks/workers/status.md` 中注册你的 Worker ID

---

## 任务认领

### 从 Coordinator 认领任务
读取 `tasks/coordinator/assignments.md`，找到 `pending` 状态的任务。

### 认领步骤
1. 选择适合的任务（考虑文件熟悉度、技能匹配）
2. 在 assignments.md 中更新任务状态为 `in_progress`
3. 在 `tasks/workers/status.md` 中更新状态为 `working`
4. 在 `tasks/workers/branches.md` 中记录你的分支

---

## 文件锁定

### 创建锁
在开始修改任何文件前，必须先在 `tasks/workers/locks/` 创建锁文件。

锁文件命名：`<文件路径>.lock`
- 文件路径中的 `/` 替换为 `_`
- 例如：`crates_cli_src_main_rs.lock`

锁文件内容：
```
Agent: Worker-001
Task: TASK-001
Time: 2026-04-18 15:30:00
Files:
  - crates/cli/src/main.rs
  - crates/cli/src/config.rs
```

### 检查锁
在创建锁之前，检查目标文件是否已被其他 Worker 锁定：
```bash
ls tasks/workers/locks/
```

如果已有锁：
1. 阅读锁内容确认是否与你的任务冲突
2. 如果冲突 → 等待或联系 Coordinator
3. 如果不冲突 → 可以同时持有锁

---

## 执行任务

### Git 工作流
1. 创建分支：`git checkout -b agent/worker-001/task-001 upstream/main`
2. 执行代码编写
3. 运行测试：`cargo test`
4. 运行检查：`cargo clippy`
5. 格式化：`cargo fmt`
6. 提交：`git add . && git commit -m "type: 描述"`

### 遵循规范
- Rust 编码：`docs/agent-rules/rust-conventions.md`
- Git 工作流：`docs/agent-rules/git-workflow.md`

---

## 完成任务

### 完成步骤
1. 确保所有测试通过
2. 确保 clippy 无警告
3. 确保代码已提交
4. 删除所有锁文件
5. 更新 `tasks/workers/status.md` 状态为 `idle`
6. 更新 `tasks/coordinator/assignments.md` 任务状态为 `completed`
7. 更新 `tasks/workers/branches.md` 分支状态

### 向 Coordinator 汇报
```markdown
[TASK-001] 完成
- 任务: [描述]
- 完成时间: YYYY-MM-DD HH:MM
- 分支: agent/worker-001/task-001
- commit: [hash]
```

---

## 状态报告

### 更新心跳
在 `tasks/workers/status.md` 中定期更新心跳时间。

### 报告格式
```markdown
[TASK-XXX] 进度报告
- 完成度: XX%
- 已完成: [列表]
- 进行中: [列表]
- 遇到的问题: [如果有]
- 下一步: [列表]
```

---

## 禁止事项

- 不要认领别人正在做的任务
- 不要修改已锁定的文件（除非锁持有者同意）
- 不要删除别人的锁文件
- 不要跳过任务池直接开始工作
- 不要跳过测试和 clippy 检查
- 不要未经汇报就长时间离开（超过 30 分钟无响应视为异常）
