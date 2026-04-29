# bot_quickstart_template

qqbotsdk的 快速开始模板。

## 项目结构

```text
.
├── Cargo.toml              # Rust 项目配置
├── config.toml             # 机器人运行配置
├── src
│   ├── main.rs             # 程序入口
│   ├── commands            # 指令模块
│   │   ├── mod.rs
│   │   ├── ping.rs
│   │   └── me.rs
│   └── context             # 上下文/示例数据模块
└── qqbot_sdk               # 本地 QQ Bot SDK
```

