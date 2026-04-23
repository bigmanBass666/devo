# 🚰 ValveOS 运行时

> **Valve（阀门）+ OS（操作系统）**
> 用户是阀门，Agent是水流。打开阀门，让AI为你工作。

## 目录结构

```
tasks/
├── ARCHITECTURE.md          ← ValveOS 完整架构文档
├── SYSTEM-MANIFEST.md       ← 元数据唯一事实来源
├── multi-agent-user-guide.md ← 用户操作指南
├── planner/                  ← 决策者工作区
├── coordinator/              ← 管理员工作区
├── workers/                  ← 工人工作区
├── pr-manager/               ← PR管理员工作区
├── maintainer/               ← 维护者工作区
├── housekeeper/              ← 仓库守护工作区
├── coo/                      ← 首席系统官工作区
├── logs/                     ← 运行日志
└── shared/                   ← 共享资源
    ├── inbox/                ← Agent消息收件箱（通信总线）
    ├── agent-status.md       ← Agent状态与任务追踪
    └── iteration-log.md      ← 迭代日志（断点续传）
```

## 快速开始

1. 唤醒 Planner：`"你是Planner。读取指令和inbox，然后开始工作。"`
2. Planner 完成后告诉你唤醒谁
3. 打开那个Agent的会话
4. 重复直到所有任务完成

## 核心机制

| 机制 | 说明 |
|------|------|
| 阀门模式 | 用户控制哪个Agent能"听到" |
| Inbox | Agent间通过文件传递消息 |
| 断点续传 | 跨会话恢复工作进度 |
| 系统重置 | 告诉任意Agent"执行系统重置" |

详见 `ARCHITECTURE.md` 和 `multi-agent-user-guide.md`
