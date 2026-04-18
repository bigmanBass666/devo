# 面向 PR 规范化的多 Agent 协调系统 — 架构文档

## 背景

### 问题
用户开启多个 Trae AI 会话并行工作，但没有分工、没有协调。需要一个可以**自主判断"做什么"**的系统，让 AI 能够自动运转，产出**干净的 PR**。

### 目标
建立五层架构的自主闭环系统：
- **Planner（决策者）** — 判断做什么，决定项目下一步
- **Coordinator（管理员）** — 分配任务、协调冲突
- **Worker（工人）** — 执行具体任务
- **PR Manager（PR 管理员）** — 提取干净改动、质量检查、准备 PR
- **Maintainer（维护者）** — 分析运行日志，持续改进系统本身

用户是最高领导人，一般情况下做旁观者，必要时介入。

---

## 项目理解

### 项目类型
**claw-code-rust** — 开源的 coding agent（类 Claude Code），用 Rust 构建。

### 仓库关系
```
upstream (claw-cli/claw-code-rust)     ← 上游（只读）
origin (bigmanBass666/claw-code-rust)  ← 你的 fork
```

---

## 五层 Agent 架构

```
┌─────────────────────────────────────────────────────────┐
│                    用户（最高领导人）                     │
│              一般旁观者，必要时介入                        │
└─────────────────────────────────────────────────────────┘
                           ▲
                           │ 指令/审批
                           ▼
┌─────────────────────────────────────────────────────────┐
│              Planner Agent（决策者）                      │
│                                                          │
│  - 理解项目现状和目标                                    │
│  - 分析 GitHub 动态、issues、PR、代码质量                 │
│  - 决定"做什么" — 生成任务计划                           │
│  - 评估优先级和依赖                                      │
│  - 评估任务是否值得提 PR                                  │
│  - 向 Coordinator 下发任务                               │
│  - 监督整体进度                                          │
│  - 记录运行日志到 tasks/logs/planner.log                │
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
│  - 管理 agent/ 分支的生命周期                             │
│  - 监控 Worker 进度                                      │
│  - 通知 PR Manager 准备 PR                               │
│  - 向 Planner 汇报状态                                   │
│  - 记录运行日志到 tasks/logs/coordinator.log            │
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
│  - 从 upstream/main 创建工作分支                         │
│  - 创建文件锁，开始工作                                  │
│  - 执行代码编写、测试、提交                              │
│  - 更新任务状态                                          │
│  - 完成后通知 Coordinator                                │
│  - 记录运行日志到 tasks/logs/workers.log                │
│                                                          │
│  位置：tasks/workers/                                    │
└─────────────────────────────────────────────────────────┘
                           ▲
                           │ 完成通知
                           ▼
┌─────────────────────────────────────────────────────────┐
│           PR Manager Agent（PR 管理员）                  │
│                                                          │
│  - 接收 Worker 完成通知                                  │
│  - 从 agent/ 分支提取干净的功能改动                       │
│  - 创建 feat/xxx 分支（基于 upstream/main）               │
│  - 执行 PR 质量检查                                      │
│  - 生成 PR 描述                                          │
│  - 向用户汇报 PR 状态                                    │
│  - 记录运行日志到 tasks/logs/pr-manager.log             │
│                                                          │
│  位置：tasks/pr-manager/                                │
└─────────────────────────────────────────────────────────┘
                           ▲
                           │ 日志 + 反馈
                           ▼
┌─────────────────────────────────────────────────────────┐
│           Maintainer Agent（维护者）                     │
│                                                          │
│  【第五层 — 自我改进闭环】                               │
│                                                          │
│  - 收集所有 Agent 的运行日志                             │
│  - 分析系统瓶颈、低效模式、重复问题                       │
│  - 生成改进报告到 tasks/maintainer/reports/              │
│  - 维护改进队列 tasks/maintainer/improvements.md        │
│  - 经用户批准后实施改进                                   │
│  - 记录运行日志到 tasks/logs/maintainer.log             │
│                                                          │
│  触发条件：每3个任务完成 / 每24小时 / 连续失败>2次       │
│                                                          │
│  位置：tasks/maintainer/                                │
└─────────────────────────────────────────────────────────┘

         ┌──────────────────────────────┐
         │      日志系统 (tasks/logs/)    │
         │                              │
         │  system.log    系统总日志     │
         │  planner.log   Planner 日志   │
         │  coordinator.log Coordinator  │
         │  workers.log   Worker 日志    │
         │  pr-manager.log PR Manager    │
         │  maintainer.log Maintainer    │
         └──────────────────────────────┘
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
│   └── backlog.md         # 长期待办列表
│
├── coordinator/           # Coordinator 专用
│   ├── instructions.md    # Coordinator 行为规范
│   ├── queue.md          # 任务队列
│   └── assignments.md    # 当前任务分配表
│
├── workers/               # Worker 共享
│   ├── instructions.md   # Worker 行为规范
│   ├── locks/            # 文件锁（不纳入 Git）
│   ├── status.md         # 各 Worker 状态
│   └── branches.md       # 各 Worker 分支
│
├── pr-manager/            # PR Manager 专用
│   ├── instructions.md   # PR Manager 行为规范
│   ├── pr-checklist.md   # PR 质量检查模板
│   ├── pr-queue.md       # 待处理的 PR
│   └── pr-history.md     # PR 历史
│
├── maintainer/            # 【第五层】Maintainer 专用
│   ├── instructions.md   # Maintainer 行为规范
│   ├── improvements.md   # 改进队列（待实施项）
│   └── reports/          # 分析报告输出
│       └── report-YYYY-MM-DD.md
│
├── logs/                  # 【新增】运行日志系统
│   ├── README.md         # 日志格式说明
│   ├── system.log        # 系统总日志
│   ├── planner.log       # Planner 日志
│   ├── coordinator.log   # Coordinator 日志
│   ├── workers.log       # Worker 日志
│   ├── pr-manager.log    # PR Manager 日志
│   └── maintainer.log    # Maintainer 日志
│
├── shared/               # 所有 Agent 共享
│   ├── rules/            # 项目规范引用
│   └── progress.md       # 进度追踪
│
└── ARCHITECTURE.md       # 本文档
```

---

## 分支策略

### Git 远程仓库结构

```
upstream (claw-cli/claw-code-rust)     ← 上游，只读
    └── main                           ← 上游主分支

origin (bigmanBass666/claw-code-rust)  ← 你的 fork
    ├── main                           ← 你的开发分支（包含所有 AI 文件）
    ├── feat/xxx                       ← 准备提 PR 的干净分支
    ├── agent/planner/xxx              ← Planner 工作分支
    ├── agent/coordinator/xxx          ← Coordinator 工作分支
    └── agent/worker-001/xxx           ← Worker 工作分支
```

### 分支创建规则

| 角色 | 分支名 | 基于 | 包含内容 |
|------|--------|------|----------|
| Planner | `agent/planner/<task>` | `main` | 所有文件 |
| Coordinator | `agent/coordinator/<task>` | `main` | 所有文件 |
| Worker | `agent/worker-<id>/<task>` | `upstream/main` | **只有功能代码** |
| PR Manager | `feat/<description>` | `upstream/main` | **只提取相关 commit** |

### 关键区别

**Worker 和 PR Manager 的分支必须基于 `upstream/main`**：
- 这样它们的 diff 天然就是干净的
- 不包含 main 上积累的 AI 协调文件
- PR 时直接 push 到 origin，从 feat/xxx 提 PR

---

## PR 流程详解

### 完整流程图

```
1. Planner 观察
   └→ 读取通知、分析代码、检查进度
   └→ 记录日志到 tasks/logs/planner.log

2. Planner 制定计划
   └→ 生成任务列表到 tasks/planner/plans/

3. Planner 下发任务
   └→ 写入 tasks/coordinator/queue.md

4. Coordinator 接收并拆分
   └→ 分解为子任务，写入 tasks/coordinator/assignments.md
   └→ 记录日志到 tasks/logs/coordinator.log

5. Coordinator 分配给 Worker
   └→ 告知 Worker 必须从 upstream/main 创建分支

6. Worker 执行
   a. git fetch upstream
   b. git checkout -b agent/worker-001/task-xxx upstream/main
   c. 编写代码、运行测试、提交
   d. push 到 origin
   e. 记录日志到 tasks/logs/workers.log

7. Worker 完成任务
   └→ 释放锁、更新状态、通知 Coordinator

8. Coordinator 通知 PR Manager
   └→ 将任务添加到 tasks/pr-manager/pr-queue.md

9. PR Manager 处理
   a. 检查 Worker 的分支
   b. 创建 feat/xxx (基于 upstream/main)
   c. cherry-pick 相关 commit
   d. 运行质量检查：
      - cargo fmt --check
      - cargo clippy（推荐但不阻塞）
      - cargo test
      - 检查 diff 是否干净
   e. 如果通过 → 生成 PR 描述
   f. 如果失败 → 返回给 Worker 修复
   g. 记录日志到 tasks/logs/pr-manager.log

10. 用户审批
    └→ 查看 PR 草稿和质量报告

11. 提交 PR
    └→ 从 feat/xxx 向 upstream/main 提 PR

12. 【改进闭环】Maintainer 分析
    a. 收集 tasks/logs/*.log 所有日志
    b. 分析系统运行模式：
       - 任务完成率 / 失败率
       - 平均任务耗时
       - 冲突频率和解决方式
       - PR 通过率
       - 低效模式识别
    c. 生成报告 → tasks/maintainer/reports/report-YYYY-MM-DD.md
    d. 更新改进队列 → tasks/maintainer/improvements.md
    e. 向用户汇报发现的问题和改进建议
    f. 用户批准后实施改进
    g. 记录日志到 tasks/logs/maintainer.log

13. Planner 重新评估
    └→ 如果有新发现或改进实施完成，回到步骤 1
```

---

## 用户介入点

```
用户可以在任何时候介入：

A. 直接向 Planner 发指令
   └→ "暂停当前计划，转向 X"

B. 向 Coordinator 发指令
   └→ "优先完成 Y 任务"

C. 向 Worker 发指令
   └→ "这个文件这样改..."

D. 向 PR Manager 发指令
   └→ "这个 PR 可以提交了"

E. 向 Maintainer 发指令
   └→ "分析一下最近的日志"
   └→ "系统有什么可以改进的？"
   └→ "批准实施改进 #003"

F. 直接修改协调文件
   └→ 修改 tasks/ 下的任何文件

G. 旁观不做任何事
   └→ 系统自主运转 + 自我改进
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

## PR 质量检查清单

PR Manager 自动执行：

```markdown
## PR 质量检查

### 代码质量
- [ ] `cargo fmt --all -- --check` 通过
- [ ] `cargo clippy --workspace --all-targets` 无错误
- [ ] `cargo test --workspace` 全部通过

### Diff 清洁度
- [ ] 改动文件数 ≤ 10 个（否则需拆分）
- [ ] 不包含 `tasks/` 目录
- [ ] 不包含 `notifications/` 目录
- [ ] 不包含 `.trae/` 目录
- [ ] 不包含 `AGENTS.md`
- [ ] 不包含 `progress.txt`

### Commit 质量
- [ ] commit 信息符合规范（type: description）
- [ ] 无 "chore: run cargo clippy --fix" 类型的 lazy commit
- [ ] commit 数量合理（≤ 5 个）
```

---

## 与原项目的集成

### 尊重现有规范
- 所有 Agent 必须遵循 AGENTS.md 的核心原则
- Git 工作流遵循 docs/agent-rules/git-workflow.md
- Rust 编码遵循 docs/agent-rules/rust-conventions.md

### 不重复造轮子
- progress.txt 保留作为人工可读进度记录
- notifications/ 保留作为 GitHub 动态来源
- docs/plans/ 和 docs/agent-rules/ 保留作为项目文档

### 协调文件位置
- 新协调系统使用 tasks/ 目录
- 不影响原有项目结构
- 通过 Git 追踪协调历史

---

## 预期效果

- **完全自主**：系统可以自己决定做什么、怎么分工、怎么执行、怎么准备 PR
- **PR 天然干净**：feat/ 分支基于 upstream/main，不包含 AI 垃圾
- **零冲突**：文件锁机制防止同时修改
- **进度透明**：所有 Agent 状态可见
- **质量自动化**：PR Manager 自动检查，减少人为疏忽
- **用户省心**：用户只需要偶尔监督，必要时介入
- **可追溯**：所有决策和改动都有记录
- **自我改进**：Maintainer 分析日志 → 发现问题 → 提出改进 → 持续优化
- **运行可观测**：完整的日志系统记录每个 Agent 的行为
- **闭环进化**：系统通过反馈循环不断变强

---

## 文件清单

| 文件路径 | 用途 | 是否纳入 Git |
|----------|------|-------------|
| `tasks/ARCHITECTURE.md` | 架构文档 | 是 |
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
| `tasks/pr-manager/instructions.md` | PR Manager 规范 | 是 |
| `tasks/pr-manager/pr-checklist.md` | PR 检查清单 | 是 |
| `tasks/pr-manager/pr-queue.md` | PR 队列 | 是 |
| `tasks/pr-manager/pr-history.md` | PR 历史 | 是 |
| `tasks/maintainer/instructions.md` | Maintainer 规范 | 是 |
| `tasks/maintainer/improvements.md` | 改进队列 | 是 |
| `tasks/maintainer/reports/*.md` | 分析报告 | 是 |
| `tasks/logs/README.md` | 日志格式说明 | 是 |
| `tasks/logs/system.log` | 系统总日志 | 可选 |
| `tasks/logs/planner.log` | Planner 日志 | 可选 |
| `tasks/logs/coordinator.log` | Coordinator 日志 | 可选 |
| `tasks/logs/workers.log` | Worker 日志 | 可选 |
| `tasks/logs/pr-manager.log` | PR Manager 日志 | 可选 |
| `tasks/logs/maintainer.log` | Maintainer 日志 | 可选 |
| `tasks/shared/progress.md` | 进度追踪 | 是 |
| `tasks/shared/rules/*.md` | 规范链接 | 是 |

### 日志文件说明

- **可选**：日志文件可以不纳入 Git（避免仓库膨胀）
- 建议在 `.gitignore` 中添加：`tasks/logs/*.log`
- 但保留 `tasks/logs/README.md` 以说明日志格式
- Maintainer 的报告（reports/*.md）应纳入 Git 作为改进历史
