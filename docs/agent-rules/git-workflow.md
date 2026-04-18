# Git 工作流与上游协作

## 远程仓库

```
origin (bigmanBass666/claw-code-rust)  ← 你的 fork
upstream (claw-cli/claw-code-rust)     ← 上游（只读）
```

**首次设置**：
```bash
git remote add upstream https://github.com/claw-cli/claw-code-rust.git
git fetch upstream
```

---

## 分支策略

### 分支类型

| 分支名 | 用途 | 基于 | 推送到 |
|--------|------|------|--------|
| `main` | 开发分支，包含所有AI文件 | - | origin |
| `agent/planner/<task>` | Planner工作分支 | `main` | origin |
| `agent/coordinator/<task>` | Coordinator工作分支 | `main` | origin |
| `agent/worker-<id>/<task>` | Worker工作分支 | **`upstream/main`** | origin |
| `feat/<description>` | 准备提PR的干净分支 | **`upstream/main`** | origin |

> **Housekeeper**：通常不需要独立分支，直接在 main 上操作 origin 远程分支（git push origin --delete）。如需记录清理操作，使用 `agent/housekeeper/cleanup-<date>` 基于 main。

### 六层 Agent 流程（含 inbox 通信）

```
Planner → 写Coordinator的inbox → 用户唤醒Coordinator
   ↓                              ↓
 决策                           分配任务
                                  ↓
Worker ← 读Worker的inbox ← 用户唤醒Worker
   ↓                              ↓
 执行代码                        完成通知到PR Manager inbox
                                  ↓
PR Manager ← 用户唤醒            提取干净PR
   ↓
分析改进 → Maintainer
   ↓
分支清理 → Housekeeper
```

**关键**：Agent间通过 `tasks/shared/inbox/` 传递消息，用户作为"阀门"控制唤醒。

### 关键规则

1. **Worker 和 feat/ 分支必须基于 `upstream/main`**
   - diff天然干净，不包含AI协调文件
2. **Planner 和 Coordinator 的分支基于 `main`**
   - 需要访问 tasks/ 协调文件，不需要提PR
3. **不要在 main 上直接做要给上游的改动**
4. **Housekeeper 只操作 origin 远程分支**

### 分支命名规范

- **Worker**: `agent/worker-001/fix-windows-unc`
- **PR**: `feat/42-fix-windows-unc-path`
- **功能**: `feat/improve-error-messages`

---

## 多 Agent 协作的完整流程

### 开发阶段

```
1. Planner 被用户唤醒
   └→ 读取 inbox → 观察 → 制定计划
   └→ 写入消息到 tasks/shared/inbox/coordinator.md
   └→ 告知用户："请唤醒 Coordinator"

2. Coordinator 被用户唤醒
   └→ 读取 inbox 发现 Planner 消息
   └→ 分配任务给 Worker
   └→ 写入消息到 tasks/shared/inbox/worker.md
   └→ 告知用户："请唤醒 Worker"

3. Worker 开发
   a. 读取 inbox 发现任务
   b. git fetch upstream
   c. git checkout -b agent/worker-001/fix-windows-unc upstream/main
   d. 编写代码、测试、提交
   e. push 到 origin
   f. 写入消息到 PR Manager 的 inbox
```

### PR 准备阶段

```
4. PR Manager 被用户唤醒
   a. 读取 inbox 发现 Worker 完成
   b. 创建 feat/fix-windows-unc (基于 upstream/main)
   c. cherry-pick Worker 的相关 commit
   d. 运行质量检查：fmt / clippy / test / diff清洁度
   e. 如果通过 → 生成PR描述 → 告知用户审批
   f. 如果失败 → 写入消息给Worker要求修复
```

---

## 提交信息

- 格式：`type: 简短描述`
- 类型：`feat:` `fix:` `refactor:` `test:` `docs:` `chore:`

### 好的 commit 信息示例
```
fix: strip Windows UNC prefix from canonicalized path  ✅
fix: handle null arrays in OpenAI responses            ✅
```

### 不好的 commit 信息示例
```
chore: apply clippy fixes across workspace             ❌ 太泛
chore: run cargo clippy --check                         ❌ 太懒
```

---

## 提交频率

- **`main`（个人维护分支）**：可以频繁提交
- **`agent/xxx`（Agent 工作分支）**：可以频繁提交
- **`feat/xxx`（给上游提 PR 的分支）**：只放干净的、相关的 commit

---

## 提交 PR 前（必须全部通过才能 push）

1. `cargo fmt --all -- --check` 无差异
2. `cargo clippy --workspace --all-targets` 无错误
3. `cargo test --workspace` 全部通过
4. 验证上游兼容性
5. 写清晰的 PR 描述：做什么/为什么/怎么做

### Diff 清洁度检查

```bash
# 检查 PR 会包含哪些文件
git diff upstream/main --name-only

# 确保不包含以下内容：
# - tasks/
# - notifications/
# - .trae/
# - AGENTS.md
```

---

## 上游协作

- 开始重要工作前务必检查上游，避免重复劳动
- 上游合并相关变更时，rebase 或 merge 保持本地同步
- 及时响应维护者对 PR 的反馈

---

## 开始重要工作前检查

1. 上游是否已实现了这个功能？
2. 是否已有相关的 open issue 或 PR？
3. 是否会与某个 open PR 冲突？
4. 是否有需要更新或添加的测试？
5. 是否需要更新文档？

---

## PR Manager 角色

PR Manager 是专门负责将 AI 工作转化为干净 PR 的 Agent：

- 从 agent/worker-xxx 分支提取干净的功能改动
- 创建基于 upstream/main 的 feat/xxx 分支
- 自动化执行所有质量检查
- 生成 PR 描述
- 向用户汇报，等待审批

详见 `tasks/pr-manager/instructions.md`
