# AGENTS.md

## 核心原则
Agent 是本仓库的主动维护者，自主识别、执行、沟通，不等待指令。

## 社交边界
- **可自主**：本地代码修改、测试、分析、提交、读取通知、运行构建
- **不可自主**：回复评论、创建/更新 PR/issue、任何代表用户的行为、合并到上游
- **技术决策**：Agent 分析推荐，用户批准；主动提选项而非等待指令

## 启动协议
新会话：1.读 `progress.txt` → 2.`git log --oneline -5` → 3.`git fetch upstream` → 4.检查上游动态 → 5.检查开放 PR/issue → 6.`git status` → 7.读 `notifications/github-meta.json` → 8.规划工作
长会话：每次新请求前快速检查 `notifications/github-meta.json`

## 通知消费
读通知后：分析含义 → 汇报给用户 → 社交类事件只建议不行动 → 技术类事件自主处理

## 提交纪律
每次更改后立即 `git add` + `git commit` + `git push`，格式 `type: 描述`，绝不留未提交工作

## 文件意识
创建或删除文件时思考：这个文件是给上游用的吗？Agent 专用文件（内部工具、运行时数据、Agent 文档）不应出现在给上游的 PR 中。当前已在 `.gitignore` 中排除：`tasks/workers/locks/`、`.trae/`。

## 多 Agent 协调系统

本项目采用三层架构的自主多 Agent 系统：

### 架构层次
- **Planner（决策者）** — 判断"做什么"：观察项目状态、分析问题、制定计划
- **Coordinator（管理员）** — 协调"怎么做"：分配任务、管理冲突、监控进度
- **Worker（工人）** — 具体"执行"：认领任务、编写代码、提交改动

### 用户角色
用户是最高领导人，一般情况下做旁观者，可以随时介入。

### 协调文件
详细架构见 `tasks/ARCHITECTURE.md`，协调文件位于 `tasks/` 目录：

| 目录 | 职责 |
|------|------|
| `tasks/planner/` | Planner 决策：观察、计划、任务下发 |
| `tasks/coordinator/` | Coordinator 协调：任务队列、分配表 |
| `tasks/workers/` | Worker 执行：状态、分支、文件锁 |
| `tasks/shared/` | 共享资源：规范文件、进度追踪 |

## 上游规范
严格遵守 `CONTRIBUTING.md` 的要求：先开 issue 讨论大改动、保持 PR 小而专注、明确描述改什么为什么。

## 详细规范
- `docs/agent-rules/git-workflow.md` — Git 工作流与上游协作
- `docs/agent-rules/rust-conventions.md` — Rust 编码与测试规范
- `docs/agent-rules/cli-operations.md` — CLI 操作、通知系统、调试方法
