# rtml 集成 tauri

## 使用

安装命令行工具

```bash
cargo install tauri-cli --locked --version "^1.0.0-rc"
```

build 好 todo example, 确保 examples/todo/pkg 目录已经有 index.html 等文件

```bash
cd examples/rtml-tauri/src-tauri

cargo tauri build
```

如果成功, 将会在 target/release/bundle 目录下找到安装包
