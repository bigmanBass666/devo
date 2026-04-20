# COO 审计日志

> 记录所有一致性审计和文档修改的历史。

## 审计记录

| 时间 | 触发来源 | 审计范围 | 发现问题 | 修复数 | 状态 |
|------|----------|----------|----------|--------|------|
| 2026-04-19 20:53 | skill-creator 正式评估（evaluate-audit-skill-dynamicity spec） | valveos-audit skill 动态性评估 | 见下方详情 | 3 项 skill 改进 | ✅ 完成 |
| 2026-04-19 21:37 | 元数据层重构（valveos-metadata-layer spec） | 全系统元数据架构 | 见下方详情 | 10+ 文件改动 | ✅ 完成 |
| 2026-04-19 22:27 | 遗留项修复（fix-backlog-items spec） | 3 个遗留 P1 + 用户指南重写 | 见下方详情 | 7 文件改动 | ✅ 完成 |
| 2026-04-19 23:20 | AGENTS.md 路由化重构（agents-md-router-refactor spec） | AGENTS.md 从静态定义改为触发→动作路由表 | 见下方详情 | 1 文件改动 | ✅ 完成 |
| 2026-04-20 02:25 | 路由化深度加固（harden-router-system spec） | 路由系统免疫系统加固 | 见下方详情 | 6 文件改动 | ✅ 完成 |

---

## ⏬ 系统重置分隔线 — 2026-04-20 (Iteration 9)

> 以下为重置前历史记录。Iteration 8 已废弃，Iteration 9 从空白开始。

---

### 2026-04-19 20:53 评估详情

**评估方法**: skill-creator 正式评估流程，5 个测试场景 × 2 版本（with-skill vs old_skill baseline）

**测试场景**:
1. 架构模型名称过时检测
2. 信息源迁移适应性（AGENTS.md 精简 → 双源提取）
3. 文件删除容错处理（用户指南不存在）
4. 跨文件同步检测（PR Manager 待机模式）
5. 定义一致性检测（单会话模式）

**关键结论**:
- ✅ **双源提取验证通过**: Eval 1 中 old_skill 无法从 AGENTS.md 提取概念表/开场白表（已移至 ARCHITECTURE.md），with-skill 正确从双源提取
- ✅ **"若存在"容错保护验证通过**: Eval 2 中 old_skill 因文件缺失导致 Step 3/4 受阻，with-skill 优雅跳过并完成全流程
- ✅ 动态基线机制对架构变更、跨文件同步、定义一致性检测均有效

**Skill 改进项（本次）**:
1. 新增 P1 #11: AGENTS.md 引用完整性检查（检测"→ 详解 XXX"目标是否存在）
2. 精简"已知历史问题"表：移除已被动态基线覆盖的架构层数模式（五层/六层/七层）
3. 新增 P2 #5: ARCHITECTURE.md 内部一致性检查（inbox 图 vs 目录树、日志列表 vs 目录树）

### 2026-04-19 21:37 元数据层重构

**触发原因**: ValveOS "纯文档驱动/全硬编码"问题——Agent 列表在 ~10 处重复、架构模型在 ~8 处重复，每次变更需改 12+ 文件。

**核心变更**: 引入 `tasks/SYSTEM-MANIFEST.md` 作为元数据唯一事实来源。

**具体改动**:
1. **新建 SYSTEM-MANIFEST.md** — 6 个章节（Agents / Architecture Model / Core Concepts / File Registry / Feature Index / 所有权约定）
2. **AGENTS.md 精简** — 从 ~154 行降到 ~119 行，移除所有元数据表格（Agent 列表/功能索引/Git 追踪规则），保留纯宪法内容
3. **ARCHITECTURE.md 更新** — 添加 4 处 Manifest 派生标注；修复 inbox 结构图缺 coo.md、目录树缺 coo.log
4. **Audit Skill 增强** — 基线来源改为 SYSTEM-MANIFEST.md（主）；新增 P1 #12 所有权约定违反检测；核心文件清单动态读取
5. **Instructions 标准化** — 7 个 Agent 的角色标签行统一格式 + Manifest 引用注
6. **logs/README.md** — 补充 coo.log 条目

**全量审计结果（重构后）**:
- 🔴 P0: **0** ✅
- 🟡 P1: 2（标准开场白章节缺失、Coordinator/Worker 缺待机章节 — 均为已知遗留）
- 🟢 P2: 0（coo.log 相关已修复）
- 所有权约定违反: **0** ✅

**遗留 P1 问题（待后续处理）**:
- ARCHITECTURE.md 缺少"标准开场白"章节（AGENTS.md 引用断裂）
- Coordinator / Worker instructions 缺待机模式章节

### 2026-04-19 22:27 遗留项修复

**修复的 3 个遗留 P1**:
1. **标准开场白章节** — 在 ARCHITECTURE.md L206 新建，7 个 Agent 完整表格。AGENTS.md L36 引用断裂修复。
2. **Coordinator 待机章节** — coordinator/instructions.md L284-L323，格式与 pr-manager 对称
3. **Worker 待机章节** — workers/instructions.md L359-L425，含 while 循环禁用警告

**用户指南重写**: `tasks/multi-agent-user-guide.md`（~185 行），面向人类读者。fix-single-session-and-pr-standby Task 5 完成。

**审计发现并即时修复**:
- P0: pr-manager/instructions.md 残留 progress.txt 引用 → 已清除
- P1: maintainer instructions 缺 coo.log 采集 → 已补充
- P1: 用户指南缺核心概念表 → 已添加

**全量审计结果**: 0 P0 / 0 P1 / 0 P2 ✅

### 2026-04-19 23:20 AGENTS.md 路由化重构

**触发原因**: 用户设计洞察 —— "AI 每次回复都会读取 AGENTS.md，这是不是可以利用？当用户提到单会话模式时，去看某某文件？"

**核心变更**: 将 AGENTS.md 从"静态宪法文档"升级为"宪法 + 中央调度器"，利用其始终在 AI 上下文中的独特属性。

**设计理念**: 从"是什么"（信息声明型）→ "怎么做"（触发→动作型）

**6 大路由领域**:
1. **架构速查** — 触发词 → SYSTEM-MANIFEST.md#Agents（完整元数据）
2. **单会话模式** — 触发词 → coo/instructions.md#单会话模式
3. **待机模式** — 触发词 → cli-operations.md#待机模式
4. **操作指引** — 7 条目子路由表（重置/审计/状态/PR/git）
5. **错误恢复** — 5 场景逃生路径（git冲突/push失败/.git损坏/审计问题/唤醒谁）
6. **审计触发** — 自动质量门禁（改文档→审计→修复→评估skill）

**非路由化章节保留**: 核心原则、铁门协议、社交边界、提交纪律、上游规范（声明式写法不变）

**量化结果**: 119 行 → 108 行（-9 行），信息零丢失，所有原内容可通过路由找到

**审计发现并即时修复**:
- P1: 单会话模式路由目标原指向 `ARCHITECTURE.md#单会话模式`（该章节不存在于此文件）→ 修正为 `coo/instructions.md#单会话模式`

**最终审计结果**: 0 P0 / 0 P1 / 0 P2 ✅

---

## ⏬ 系统重置分隔线 — 2026-04-20 (Iteration 10)

> 以下为重置前历史记录。Iteration 9 已废弃，Iteration 10 从空白开始。

---

---

## ⏬ 系统重置分隔线 — 2026-04-20

> 以下为重置前历史记录。Iteration 7 已废弃，Iteration 8 从空白开始。

---

### 2026-04-20 02:25 路由化深度加固

**触发原因**: 路由化重构后的 5 个加固方向——审计 skill 路由目标检查、路由所有权约定、章节分类原则、速查行指引、fallback 路由。

**核心改动**:
1. **审计 skill P1 #13** — 新增路由目标存在性检查（Step 2.5），验证 AGENTS.md 每个路由动作引用的文件和章节是否真实存在
2. **SYSTEM-MANIFEST.md 路由所有权** — 所有权约定表新增"AGENTS.md 路由条目"行 + 路由覆盖原则注
3. **SYSTEM-MANIFEST.md 章节分类原则** — 新增"## AGENTS.md 章节分类原则"章节，文档化可路由化 vs 不可路由化的二分法
4. **AGENTS.md 速查行指引** — 路由表头部加速查行质量约束注
5. **AGENTS.md fallback 路由** — 新增"❓ 不确定时"路由条目

**审计发现并修复**:
- P1: AGENTS.md 路由表使用缩写路径（`SYSTEM-MANIFEST.md` 缺 `tasks/` 前缀、`cli-operations.md` 缺 `docs/agent-rules/` 前缀）→ 统一为完整路径
- P1: Coordinator instructions 待机模式使用 while 循环，与 cli-operations.md 规范矛盾 → 改为单次 sleep + 警告
- P1: PR Manager instructions 待机模式使用 while 循环，同上 → 改为单次 sleep + 警告
- P2: ARCHITECTURE.md 目录树缺 SYSTEM-MANIFEST.md/README.md/multi-agent-user-guide.md → 已补充
- P2: ARCHITECTURE.md 文件清单表缺 SYSTEM-MANIFEST.md/README.md/multi-agent-user-guide.md → 已补充
- P2: AGENTS.md 路径风格不统一 → 随 P1-1 一起修复
- P2: tasks/README.md 目录结构缺 SYSTEM-MANIFEST.md 和 coo/ → 已补充

**最终审计结果**: 0 P0 / 0 P1 / 0 P2 ✅
