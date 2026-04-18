# 自主多 Agent 协调系统 — 重新设计

## 背景与目标

### 问题
用户开启多个 Trae AI 会话并行工作，但没有分工、没有协调。需要一个可以**自主判断"做什么"**的系统，让 AI 能够自动运转，用户只需要做旁观者，必要时介入。

### 目标
建立三层架构的自主闭环系统：
- **Planner（决策者）** — 判断做什么，决定项目下一步
- **Coordinator（管理员）** — 分配任务、协调冲突
- **Worker（工人）** — 执行具体任务

用户是最高领导人，一般情况下做旁观者，可以随时介入。

---

## 项目理解

### 项目类型
**claw-code-rust** — 开源的 coding agent（类 Claude Code），用 Rust 构建。

### 现有架构层次
- Session（会话）→ Turn（回合）→ Item（条目）
- Tools / Safety / Context Management / MCP / Skills / Server API

### 项目当前状态（来自 progress.txt）
- Iteration 1 已完成多个功能
- 有 PR #38, #39 待审核
- 处于早期开发阶段

### 项目规范文档
- `AGENTS.md` — Agent 工作宪法
- `docs/agent-rules/` — 详细规则（Git、Rust、CLI）
- `docs/spec-*.md` — 详细设计规范
- `progress.txt` — 当前进度记录
- `notifications/` — GitHub 活动通知

---

## 三层 Agent 架构

```
┌─────────────────────────────────────────────────────────┐
│                    用户（最高领导人）                     │
│              一般旁观者，必要时介入                        │
└─────────────────────────────────────────────────────────┘
                           ▲
                           │ 指令/介入
                           ▼
┌─────────────────────────────────────────────────────────┐
│              Planner Agent（决策者）                      │
│                                                          │
│  - 理解项目现状和目标                                    │
│  - 分析 GitHub 动态、issues、PR、代码质量                 │
│  - 决定"做什么" — 生成任务计划                           │
│  - 评估优先级和依赖                                      │
│  - 向 Coordinator 下发任务                               │
│  - 监督整体进度                                          │
│                                                          │
│  位置：tasks/planner/                                    │
└─────────────────────────────────────────────────────────┘
                           ▲
                           │ 任务
                           ▼
┌─────────────────────────────────────────────────────────┐
│            Coordinator Agent（管理员）                    │
│                                                          │
│  - 接收 Planner 下发的任务                               │
│  - 将大任务拆分为子任务                                  │
│  - 分配给空闲的 Worker                                  │
│  - 协调冲突、处理文件锁                                  │
│  - 监控 Worker 进度                                      │
│  - 向 Planner 汇报状态                                   │
│                                                          │
│  位置：tasks/coordinator/                               │
└─────────────────────────────────────────────────────────┘
                           ▲
                           │ 任务
                           ▼
┌─────────────────────────────────────────────────────────┐
│              Worker Agent（工人）                        │
│                                                          │
│  - 认领 Coordinator 分配的任务                           │
│  - 创建文件锁，开始工作                                  │
│  - 执行代码编写、测试、提交                              │
│  - 更新任务状态                                          │
│  - 完成后通知 Coordinator                                │
│                                                          │
│  位置：tasks/workers/                                    │
└─────────────────────────────────────────────────────────┘
```

---

## 目录结构

```
tasks/
├── planner/               # Planner 专用
│   ├── instructions.md    # Planner 行为规范
│   ├── vision.md          # 项目愿景和目标理解
│   ├── observations.md    # 当前观察到的项目状态
│   ├── plans/             # 生成的任务计划
│   │   └── YYYY-MM-DD-ITERATION-X.md
│   └── backlog.md         # 长期待办列表
│
├── coordinator/           # Coordinator 专用
│   ├── instructions.md    # Coordinator 行为规范
│   ├── queue.md          # 任务队列
│   └── assignments.md    # 当前任务分配表
│
├── workers/               # Worker 共享
│   ├── instructions.md   # Worker 行为规范
│   ├── locks/            # 文件锁
│   ├── status.md         # 各 Worker 状态
│   └── branches.md       # 各 Worker 分支
│
├── shared/               # 所有 Agent 共享
│   ├── rules/            # 项目规范引用
│   │   ├── git-workflow.md
│   │   ├── rust-conventions.md
│   │   └── cli-operations.md
│   └── progress.md       # 进度追踪
│
└── ARCHITECTURE.md       # 本文档
```

---

## 各 Agent 职责详解

### Planner Agent

**核心问题：判断"做什么"**

#### 职责
1. **持续观察**
   - 读取 `notifications/github-meta.json` 了解上游动态
   - 检查 GitHub issues、PR 状态
   - 分析 `progress.txt` 了解当前进度
   - 分析代码库发现问题（TODO、FIXME、BUG、clippy 警告）
   - 运行测试获取项目健康状态

2. **制定计划**
   - 根据观察结果生成任务列表
   - 按优先级排序（紧急 > 重要 > 一般）
   - 识别任务依赖关系
   - 将任务写入 `tasks/planner/plans/YYYY-MM-DD-ITERATION-X.md`

3. **下发任务**
   - 将任务写入 `tasks/coordinator/queue.md`
   - 设置任务优先级和截止时间
   - 跟踪任务执行状态

4. **监督改进**
   - 检查 Worker 完成任务的质量
   - 根据反馈调整后续计划
   - 识别需要用户介入的情况

#### 何时介入/通知用户
- 发现重大方向性问题
- 需要用户做决策（如：选择 A 方案还是 B 方案）
- 发现关键阻塞无法自行解决
- 重大里程碑完成

---

### Coordinator Agent

**核心问题：怎么协调"做"**

#### 职责
1. **接收任务**
   - 从 `tasks/coordinator/queue.md` 读取 Planner 下发的任务
   - 分析任务所需的技能和文件
   - 评估任务是否可以并行

2. **拆分任务**
   - 将大任务拆分为可并行执行的子任务
   - 每个子任务尽量独立（减少文件冲突）
   - 更新 `tasks/coordinator/assignments.md`

3. **分配任务**
   - 根据 Worker 状态分配任务
   - 遵循规则：同一文件同一时间只能被一个 Worker 修改
   - 分配后更新 `tasks/workers/status.md`

4. **协调冲突**
   - 监控 `tasks/workers/locks/` 中的文件锁
   - 当冲突发生时，决定优先级或调整任务分配
   - 必要时向 Planner 报告阻塞

5. **汇报进度**
   - 定期更新 `tasks/shared/progress.md`
   - 任务完成后通知 Planner
   - 遇到问题时请求 Planner 决策

---

### Worker Agent

**核心问题：具体怎么执行**

#### 职责
1. **启动准备**
   - 阅读 `tasks/workers/instructions.md`
   - 阅读相关项目规范（`tasks/shared/rules/`）
   - 在 `tasks/workers/status.md` 中注册

2. **认领任务**
   - 从 `tasks/coordinator/assignments.md` 认领任务
   - 更新状态为 `working`

3. **锁定文件**
   - 在 `tasks/workers/locks/` 中创建锁文件
   - 锁文件命名：`文件路径.lock`（路径中的 `/` 替换为 `_`）
   - 内容包含：Agent ID、任务 ID、锁定时间、要修改的文件列表

4. **执行任务**
   - 按照项目规范执行代码编写
   - 遵循 Rust 编码规范（`tasks/shared/rules/rust-conventions.md`）
   - 遵循 Git 工作流（`tasks/shared/rules/git-workflow.md`）
   - 提交代码：`git add . && git commit -m "type: 描述"`

5. **完成任务**
   - 删除锁文件
   - 更新 `tasks/workers/status.md` 状态为 `idle`
   - 更新 `tasks/coordinator/assignments.md` 任务状态为 `completed`
   - 通知 Coordinator

---

## 自主闭环流程

### 完整周期

```
1. Planner 观察
   └→ 读取通知、分析代码、检查进度

2. Planner 制定计划
   └→ 生成任务列表到 tasks/planner/plans/

3. Planner 下发任务
   └→ 写入 tasks/coordinator/queue.md

4. Coordinator 接收并拆分
   └→ 分解为子任务，写入 tasks/coordinator/assignments.md

5. Coordinator 分配给 Worker
   └→ Worker 认领任务，创建锁

6. Worker 执行
   └→ 编写代码、运行测试、提交

7. Worker 完成任务
   └→ 释放锁、更新状态

8. Coordinator 监控
   └→ 检查任务完成，处理冲突

9. Coordinator 汇报
   └→ 向 Planner 报告进度

10. Planner 重新评估
    └→ 如果有新发现，回到步骤 1
```

### 用户介入点

```
用户可以在任何时候介入：

A. 直接向 Planner 发指令
   └→ "暂停当前计划，转向 X"

B. 向 Coordinator 发指令
   └→ "优先完成 Y 任务"

C. 向 Worker 发指令
   └→ "这个文件这样改..."

D. 直接修改协调文件
   └→ 修改 tasks/ 下的任何文件

E. 旁观不做任何事
   └→ 系统自主运转
```

---

## 文件锁机制

### 锁文件格式
位置：`tasks/workers/locks/<文件路径>.lock`

内容：
```
Agent: Worker-001
Task: TASK-001
Time: 2026-04-18 15:30:00
Files:
  - crates/cli/src/main.rs
  - crates/cli/src/config.rs
```

### 锁冲突处理
1. Worker A 锁定 `crates/cli/src/main.rs`
2. Worker B 也想修改同一文件
3. Coordinator 检测到冲突
4. Coordinator 决策：
   - 如果 A 任务紧急 → B 等待 A 完成
   - 如果可拆分 → B 只改不冲突的部分
   - 如果无法协调 → 报告 Planner

---

## 状态文件

### `tasks/workers/status.md`
| Agent ID | 状态 | 当前任务 | 开始时间 | 最新心跳 |
|----------|------|----------|----------|----------|
| Worker-001 | idle | - | - | - |

### `tasks/coordinator/assignments.md`
| 任务 ID | 描述 | 分配给 | 状态 | 创建时间 |
|---------|------|--------|------|----------|
| TASK-001 | 实现 X 功能 | Worker-001 | completed | 2026-04-18 |

### `tasks/shared/progress.md`
当前迭代进度，各任务完成百分比，阻塞列表。

---

## 与原项目的集成

### 尊重现有规范
- 所有 Agent 必须遵循 `AGENTS.md` 的核心原则
- Git 工作流遵循 `docs/agent-rules/git-workflow.md`
- Rust 编码遵循 `docs/agent-rules/rust-conventions.md`

### 不重复造轮子
- `progress.txt` 保留作为人工可读进度记录
- `notifications/` 保留作为 GitHub 动态来源
- `docs/plans/` 和 `docs/agent-rules/` 保留作为项目文档

### 协调文件位置
- 新协调系统使用 `tasks/` 目录
- 不影响原有项目结构
- 通过 Git 追踪协调历史

---

## 实施步骤

### 第一阶段：重构目录结构
1. 创建 `tasks/planner/` 目录和文件
2. 创建 `tasks/coordinator/` 目录和文件
3. 重命名/移动 `tasks/` 下现有文件到 `tasks/workers/`
4. 创建 `tasks/shared/rules/` 并链接/复制规范文件

### 第二阶段：实现 Planner
1. 创建 `tasks/planner/instructions.md` — Planner 行为规范
2. 创建 `tasks/planner/vision.md` — 项目愿景
3. 实现观察循环（GitHub 通知分析、代码分析）
4. 实现计划生成

### 第三阶段：实现 Coordinator
1. 扩展现有 `tasks/coordinator/instructions.md`
2. 实现任务队列管理
3. 实现冲突协调逻辑

### 第四阶段：实现 Worker
1. 完善 `tasks/workers/instructions.md`
2. 实现锁机制
3. 实现状态报告

### 第五阶段：集成与测试
1. 更新 `AGENTS.md` 引入新架构
2. 更新 `.gitignore`
3. 端到端测试

---

## 预期效果

- **完全自主**：系统可以自己决定做什么、怎么分工、怎么执行
- **零冲突**：文件锁机制防止同时修改
- **进度透明**：所有 Agent 状态可见
- **用户省心**：用户只需要偶尔监督，必要时介入
- **可追溯**：所有决策和改动都有记录

---

## 文件清单

| 文件路径 | 用途 | 是否纳入 Git |
|----------|------|-------------|
| `tasks/planner/instructions.md` | Planner 规范 | 是 |
| `tasks/planner/vision.md` | 项目愿景 | 是 |
| `tasks/planner/observations.md` | 观察记录 | 是 |
| `tasks/planner/plans/*.md` | 任务计划 | 是 |
| `tasks/planner/backlog.md` | 长期待办 | 是 |
| `tasks/coordinator/instructions.md` | Coordinator 规范 | 是 |
| `tasks/coordinator/queue.md` | 任务队列 | 是 |
| `tasks/coordinator/assignments.md` | 任务分配 | 是 |
| `tasks/workers/instructions.md` | Worker 规范 | 是 |
| `tasks/workers/locks/*.lock` | 文件锁 | **否** |
| `tasks/workers/status.md` | Worker 状态 | 是 |
| `tasks/workers/branches.md` | 分支记录 | 是 |
| `tasks/shared/progress.md` | 进度追踪 | 是 |
| `tasks/shared/rules/*.md` | 规范链接 | 是 |
| `tasks/ARCHITECTURE.md` | 本文档 | 是 |
