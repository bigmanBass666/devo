# PR 质量检查清单

此文件是 PR Manager 执行质量检查的标准模板。

**基于上游 CI 配置和 CONTRIBUTING.md 要求**

## 上游 CI 强制检查项（必须通过）

### 1. 代码格式 ✅ 强制
```bash
cargo fmt --all -- --check
```
- [ ] 通过：无差异
- [ ] 失败：需要运行 `cargo fmt --all`

### 2. 编译检查 ✅ 强制
```bash
cargo check --workspace --all-targets
```
- [ ] 通过：编译成功
- [ ] 失败：有编译错误

### 3. 测试 ✅ 强制
```bash
cargo test --workspace
```
- [ ] 通过：全部测试通过
- [ ] 失败：有测试失败

### 4. 文档生成 ✅ 强制
```bash
cargo doc --workspace --no-deps
```
- [ ] 通过：文档生成无警告
- [ ] 失败：有文档警告（RUSTDOCFLAGS: -Dwarnings）

## 上游 CI 非强制检查项（建议修复）

### 5. 静态分析 ⚠️ 建议但非强制
```bash
cargo clippy --workspace --all-targets
```
- [ ] 通过：无错误/警告
- [ ] 有警告：**上游 CI 未启用 clippy，建议修复但不阻塞 PR**
- **注意**：不要专门为 clippy 警告提 PR，除非与功能改动相关

## 我们自己的质量标准

### 6. Diff 清洁度 ✅ 必须通过
```bash
git diff upstream/main --name-only
```
检查结果：
- [ ] 不包含 `tasks/` 目录
- [ ] 不包含 `notifications/` 目录
- [ ] 不包含 `.trae/` 目录
- [ ] 不包含 `AGENTS.md`
- [ ] 改动文件数 ≤ 10 个（否则需拆分）

### 7. Commit 质量 ✅ 必须通过
```bash
git log upstream/main..HEAD --oneline
```
- [ ] commit 信息符合规范（type: description）
- [ ] 无 lazy commit（如 "run cargo clippy --fix"、"apply clippy fixes across workspace"）
- [ ] commit 数量合理（≤ 5 个）

### 8. 上游贡献规范 ✅ 必须符合
- [ ] 非平凡改动已先开 Issue 讨论
- [ ] 没有与其他 Issue/PR 重复的工作
- [ ] 改动遵循现有 crate 结构和代码风格
- [ ] PR 描述清晰说明改什么、为什么

---

## 检查结果记录

```markdown
## PR 检查: <task-id>

### 基本信息
- **任务**: <描述>
- **Worker**: <id>
- **分支**: <branch>
- **关联 Issue**: #XXX（如果有）
- **检查时间**: YYYY-MM-DD HH:MM

### 上游 CI 检查（必须全部通过）
| 项目 | 结果 | 备注 |
|------|------|------|
| cargo fmt | ✅/❌ | |
| cargo check | ✅/❌ | |
| cargo test | ✅/❌ | |
| cargo doc | ✅/❌ | |

### 非强制检查
| 项目 | 结果 | 备注 |
|------|------|------|
| cargo clippy | ⚠️/✅ | 建议，不阻塞 |

### 我们的额外检查
| 项目 | 结果 | 备注 |
|------|------|------|
| Diff 清洁度 | ✅/❌ | |
| Commit 质量 | ✅/❌ | |
| 上游规范 | ✅/❌ | |

### 结论
- [ ] 可以提交 PR
- [ ] 需要修复后重试
- [ ] 需要返回给 Worker
```

---

## 使用说明

1. 每次 PR 准备时，复制此模板填写
2. **上游 CI 检查项（1-4）必须全部通过**
3. **Clippy（第5项）是建议性的，不阻塞 PR**
4. **我们的额外检查（6-8）必须通过**
5. 将检查结果保存到 `tasks/pr-manager/pr-history.md`

---

## 重要提醒

### 关于 Clippy
上游的 CI 配置中 **clippy 是注释掉的**！这意味着：
- 不要为 clippy 警告单独提 PR
- 如果功能改动顺便修了 clippy，可以一起提交
- 但如果 PR 主要内容是"fix clippy warnings"，可能不会被接受

### 关于 Issue 先行
上游 CONTRIBUTING.md 明确要求：
> For non-trivial changes, please open an issue or start a discussion before submitting a PR

这意味着：
- 新功能 → 必须先开 Issue
- Bug 修复 → 可以直接提 PR（但最好先确认）
- 文档改进 → 可以直接提 PR
