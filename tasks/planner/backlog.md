# 长期待办列表

此文件记录无法在当前迭代完成但需要跟踪的任务。

---

## Upstream PR 回迁评估报告

> 生成时间：2026-04-23 11:56:00 | 任务：TASK-ITER11-007 | 分析者：Worker
> 更新：2026-04-23 11:56:00 - 验证 upstream 最新状态，添加 Issue #43

### 评估总览

| PR# | 标题 | 状态 | 回迁优先级 | 回迁风险 | 建议 |
|-----|------|------|-----------|---------|------|
| #30 | feat: emit plan items for update_plan | OPEN | 🔴 P0-高 | 🟢 低 | ✅ 强烈推荐回迁 |
| #51 | Dev/review0422 | MERGED | 🔴 P0-高 | 🟡 中 | ✅ 推荐回迁（需仔细合并） |
| #50 | Dev/doc | MERGED | 🟡 P1-中 | 🟢 低 | ✅ 推荐回迁 |
| #49 | add doc to cli crate; remove ProviderFamily | MERGED | 🟡 P1-中 | 🟡 中 | ✅ 推荐回迁（注意 ProviderFamily 移除） |
| #46 | Dev/brand | MERGED | 🟢 P2-低 | 🟢 低 | ⚠️ 可选回迁（品牌重命名） |
| #45 | upgrade tui to v2 | MERGED | 🔴 P0-高 | 🔴 高 | ⚠️ 推荐但需大量适配 |
| #34 | fix config BUG; add markdown support for TUI | MERGED | 🟡 P1-中 | 🟡 中 | ✅ 推荐回迁 |
| #33 | Dev/fix thinking | MERGED | 🟡 P1-中 | 🟡 中 | ✅ 推荐回迁 |

### 最新动态（2026-04-23 验证）

- **Upstream 仓库名**：实际为 `7df-lab/devo`（fork 自 `7df-lab/claw-code-rust`）
- **upstream/main 最新 commit**：`c55c4b0` (ci) — 2026-04-22 17:55
- **当前 OPEN PR**：仅 #30（feat: emit plan items）
- **Issue #43**：第三方独立评估报告，评分 7.83/10，详见下方

### PR 详细分析

#### PR #30 — feat: emit plan items for update_plan
- **状态**：OPEN（未合并）
- **作者**：Distortedlogic (Jeremy Meek)
- **变更规模**：+1463/-44，12 文件
- **核心变更**：
  - 为 `update_plan` 工具结果添加一流的 plan items 和 item/plan/delta 通知
  - 在 plan item 完成后 emit `turn/plan/updated`
  - 新增 `crates/server/src/runtime/plan.rs`（91行）
  - 新增集成测试 `crates/server/tests/plan_integration.rs`（824行）
  - TUI 层新增 plan 事件处理
- **回迁价值**：🟢 高 — 增强了 plan 系统的事件驱动能力，与 ValveOS 的任务追踪理念一致
- **回迁风险**：🟢 低 — 变更独立且自包含，有完整测试覆盖
- **回迁策略**：直接 cherry-pick，冲突概率低

#### PR #51 — Dev/review0422
- **状态**：MERGED（2026-04-22）
- **作者**：wangtsiao
- **变更规模**：+1186/-407，32 文件
- **核心变更**：
  - 大规模协议层重构：`protocol/src/session.rs`、`protocol/src/turn.rs`、`protocol/src/event.rs`
  - 服务端持久化和运行时重构：`persistence.rs`、`runtime.rs`、`projection.rs`
  - TUI 层重大更新：`chatwidget.rs`（+209/-118）、`host.rs`（+103/-78）、`worker.rs`（+157/-65）
  - 新增 `custom_terminal.rs`、`custom_terminal_clear_tests.rs`
  - Footer 组件重构（+80/-1）
- **回迁价值**：🔴 极高 — 协议层和 TUI 层的重大架构更新，是后续 PR 的基础
- **回迁风险**：🟡 中 — 变更量大，但结构清晰；可能与 ValveOS 自定义的 TUI/worker 代码冲突
- **回迁策略**：需要逐模块合并，优先处理 protocol 层变更，再处理 server 层，最后处理 TUI 层

#### PR #50 — Dev/doc
- **状态**：MERGED（2026-04-22）
- **作者**：wangtsiao
- **变更规模**：+1002/-691，108 文件
- **核心变更**：
  - 修复 paste burst 问题
  - TUI 底部面板大规模重构：`chat_composer.rs`（+147/-31）
  - 新增 `paste_burst.rs` 模块
  - 新增 `host.rs`（475行新增）
  - 大量文件从 `tui/src/` 迁移到 `tui/src/bottom_pane/` 和 `tui/src/` 下的新结构
  - Snapshot 文件更新（v2 命名空间）
- **回迁价值**：🟡 中 — paste burst 修复有实用价值，TUI 重构是架构改进
- **回迁风险**：🟢 低 — 主要是 TUI 层变更，不影响核心逻辑
- **回迁策略**：与 PR#51 一起回迁，注意文件路径变化

#### PR #49 — add doc to cli crate; remove ProviderFamily
- **状态**：MERGED（2026-04-21）
- **作者**：wangtsiao
- **变更规模**：+1092/-857，25 文件
- **核心变更**：
  - CLI 重构：拆分 `agent.rs` 为 `agent_command.rs`、`doctor_command.rs`、`prompt_command.rs`
  - 移除 `ProviderFamily`（从 `provider.rs` -68行）
  - 协议层 `model.rs` 重构（+81/-66）
  - 日志系统改进（+67/-7）
  - OpenAI provider 更新（+42/-19）
- **回迁价值**：🟡 中 — CLI 重构和 ProviderFamily 移除是架构简化
- **回迁风险**：🟡 中 — ProviderFamily 移除可能影响 ValveOS 的 provider 配置
- **回迁策略**：需检查 ValveOS 是否依赖 ProviderFamily，如有则需适配

#### PR #46 — Dev/brand
- **状态**：MERGED（2026-04-21）
- **作者**：wangtsiao
- **变更规模**：+1432/-1443，134 文件
- **核心变更**：
  - 仓库名从 `claw-code-rust` 重命名为 `devo`
  - 所有 crate 名称、二进制名称、配置路径从 `clawcr_*` 改为 `devo*`
  - README 多语言版本更新
  - 添加 demo gif 和封面图片
  - Cargo.toml 大量更新
- **回迁价值**：🟢 低 — 纯品牌重命名，无功能改进
- **回迁风险**：🟢 低 — 但回迁后需反向操作，将 `devo*` 改回 `clawcr_*`
- **回迁策略**：⚠️ **不建议直接回迁**。如果 ValveOS 要保持 `clawcr_*` 命名，回迁此 PR 后需大量反向替换。建议仅参考其文档和 gif 资源。

#### PR #45 — upgrade tui to v2
- **状态**：MERGED（2026-04-21）
- **作者**：wangtsiao
- **变更规模**：+53324/-7783，185 文件（**超大变更**）
- **核心变更**：
  - TUI 重大重写，采用 Codex 风格的内联模式和事件驱动架构
  - 删除旧 TUI 实现（`input.rs`、`paste_burst.rs`、`render/`、`runtime.rs`、`selection.rs`、`slash.rs`、`terminal.rs`、`tests.rs`、`transcript.rs`）
  - 新增 `v2/` 目录下完整 TUI 实现（chatwidget、bottom_pane、markdown_render 等）
  - 新增 `file-search` crate（1187行 lib.rs）
  - 协议层新增 `parse_command.rs`、`protocol.rs` 扩展
  - Provider 层 Anthropic 和 OpenAI 适配更新
  - 工具层 `apply_patch.rs`、`file_write.rs`、`read.rs` 改进
- **回迁价值**：🔴 极高 — TUI v2 是核心架构升级，所有后续 TUI PR 都基于此
- **回迁风险**：🔴 极高 — 53000+ 行新增，完全重写 TUI 层，与 ValveOS 自定义代码冲突概率极大
- **回迁策略**：**必须首先回迁**，但需要：
  1. 先合并 PR#33（fix thinking）和 PR#32（refactor 0414）作为前置
  2. 逐模块验证编译
  3. ValveOS 自定义的 TUI 代码需要完全重写到 v2 架构上

#### PR #34 — fix config BUG; add markdown support for TUI
- **状态**：MERGED（2026-04-17）
- **作者**：wangtsiao
- **变更规模**：+4151/-700，30 文件
- **核心变更**：
  - 修复配置 BUG
  - Provider 配置重构（-277行）
  - 工具运行时重构：新增 `runtime/` 模块（assembly、builtins、executor、legacy、registry、shell_command、tests、types）
  - 新增 `shell_exec.rs`（467行）
  - TUI markdown 渲染支持
  - Onboarding 流程更新
- **回迁价值**：🟡 中 — 工具运行时重构和配置 BUG 修复有价值
- **回迁风险**：🟡 中 — 工具运行时重构可能与 ValveOS 的工具扩展冲突
- **回迁策略**：先回迁 PR#33 和 PR#32，再处理此 PR 的工具运行时部分

#### PR #33 — Dev/fix thinking
- **状态**：MERGED（2026-04-16）
- **作者**：wangtsiao
- **变更规模**：+1501/-667，32 文件
- **核心变更**：
  - 移除 `TurnToolMode` 和 `SystemPromptMode`
  - 新增 OpenAI responses 和 chat completion API 到 provider family
  - 更新 system prompt 和 user prompt
  - 修复 log level 问题
  - 修复 TUI thinking 渲染
  - 新增 `user_input.rs`（110行）
  - Provider 配置大幅重构（+563/-100）
- **回迁价值**：🟡 中 — thinking 修复和 provider family 扩展有实用价值
- **回迁风险**：🟡 中 — TurnToolMode/SystemPromptMode 移除可能影响 ValveOS 的 turn 处理逻辑
- **回迁策略**：需检查 ValveOS 是否使用 TurnToolMode 或 SystemPromptMode

### 回迁依赖关系

```
PR#32 (refactor 0414) ← PR#33 (fix thinking) ← PR#34 (config+markdown)
                                                    ↓
PR#31 (api doc) ← PR#32 (refactor) ← PR#45 (TUI v2) ← PR#46 (brand)
                                        ↓
                                    PR#49 (remove ProviderFamily)
                                        ↓
                                    PR#50 (doc/paste burst)
                                        ↓
                                    PR#51 (review0422)
                                        ↓
                                    PR#30 (plan items) [OPEN]
```

### 推荐回迁顺序

1. **第一批（基础层）**：PR#31 → PR#32 → PR#33
   - 协议层和 provider 层重构，是后续所有 PR 的基础
2. **第二批（核心架构）**：PR#45 (TUI v2)
   - 必须在第一批完成后进行，是最大的变更
3. **第三批（功能增强）**：PR#34 → PR#49
   - 工具运行时和 CLI 重构
4. **第四批（近期更新）**：PR#50 → PR#51
   - TUI 修复和协议更新
5. **第五批（可选）**：PR#30 → PR#46
   - plan items 功能（OPEN PR）和品牌重命名（可选）

### 风险提示

- ⚠️ PR#46（品牌重命名）将所有 `clawcr_*` 改为 `devo*`，ValveOS 如需保持原命名则不建议回迁
- ⚠️ PR#45（TUI v2）变更量极大（53000+行），建议分配专门迭代处理
- ⚠️ PR#33 移除了 TurnToolMode 和 SystemPromptMode，需确认 ValveOS 是否依赖这些类型
- ⚠️ PR#49 移除了 ProviderFamily，需确认 ValveOS 的 provider 配置是否受影响

### Issue #43 — 第三方独立评估报告（重要参考）

- **标题**：📊 Comprehensive Independent Evaluation of claw-code-rust (7.83/10)
- **评估者**：z.ai agent mode (GLM-5.1) | **日期**：April 2026
- **核心结论**：claw-code-rust 是唯一一个从 Claude Code 泄露代码中实现真正技术独立的衍生项目
- **关键优势**：
  - 真正的干净房间开发（11份规格文档，无专有代码）
  - 技术独立性已证明（Rust Edition 2024 + Resolver 3）
  - 支持 6+ providers（Claude, OpenAI, z.ai, Qwen, Deepseek, Ollama）
  - MIT 许可证，法律立场清晰
- **需改进**：
  - 单一维护者依赖（95%+ 提交来自 wangtsiao）
  - 早期社区（251 stars）
  - 文档需扩展
- **战略建议**：
  1. 差异化品牌定位
  2. 扩大贡献者基础
  3. 投资文档
  4. 建立治理机制
  5. 构建集成测试
  6. 寻求企业合作

---

## P3 - 长期改进

<!-- 长期改进任务 -->

---

## 已搁置

<!-- 暂时搁置的任务及原因 -->

---

## 未来探索

<!-- 探索性想法，不确定是否实施 -->

```
- [ ] 探索：XXX
```

---

## 备注

- 定期审查此列表，清理已完成或不再相关的项
- 当新迭代开始时，评估哪些项可以提升优先级
