# AGENTS.md — 项目基础规则

> ⚠️ **本文件包含所有模式下都必须遵守的基础规则**
>
> **ValveOS 多 Agent 协作系统**仅在输入 `/valveos` 命令后启用。
> 详见 `docs/agent-rules/valveos-protocol.md`
>
> 💡 默认为 **🌿 普通模式**（直接对话，零协议开销）。
> 需要 PR 流程、多 Agent 协作、任务追踪时，输入 `/valveos` 切换到 **⚡ ValveOS 模式**。

---

## 🔒 安全铁律

- 永不删除 main 分支
- 永不 push 到 upstream（只读）
- 永不向 PR 中包含 tasks/、notifications/、.trae/、AGENTS.md
- 永不绕过用户直接操作 origin 以外的远程
- 重置前必须二次确认
- **永不执行 `git init`**（会覆盖已有的 .git 历史，导致所有提交记录丢失）
- **`.git` 目录丢失时，从远程 clone 恢复**：先 `git clone [fork-url] [临时目录]`，再 `Copy-Item [临时目录]/.git [原仓库]/.git`

## 📝 提交纪律

每次更改后立即使用Git MCP 进行 `git add` + `git commit` + `git push`，格式 `type: 描述`，绝不留未提交工作。Git 安全规则详见 `docs/agent-rules/git-workflow.md`。

## 🛡️ 社交边界

- **可自主**：本地代码修改、测试、分析、提交、读取通知、运行构建
- **不可自主**：回复评论、创建/更新 PR/issue、任何代表用户的行为、合并到上游
- **技术决策**：AI 分析推荐，用户批准；主动提选项而非等待指令

---

## ⚡ 快速启动 ValveOS

当需要多 Agent 协作时，输入以下任一命令：

| 命令 | 效果 |
|------|------|
| `/valveos` 或 `/vo` | 进入 ValveOS 模式（加载完整协议） |
| `唤醒 [Agent名]` | 进入 ValveOS 模式 + 直接切换到该 Agent |
| `/heartbeat` 或 `/hb` | 进入心跳模式（Agent 自主轮询，直接通信） |

**何时应该启动 ValveOS？**
- ✅ 需要 PR 完整流程（编码 → 测试 → 审核 → 提交）
- ✅ 需要多 Agent 并行协作
- ✅ 需要任务追踪和状态管理
- ✅ 需要系统审计或改进分析
- ✅ 需要 Agent 间实时通信和自主协作

**何时不应该启动？**
- ❌ 快速提问或简单修改
- ❌ 单一功能的调试或排查
- ❌ GitHub Actions / CI 配置调整
- ❌ 代码解释或学习
