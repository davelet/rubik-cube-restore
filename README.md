# Tauri + Vue

This template should help get you started developing with Tauri and Vue in your applications.

## Recommended IDE Setup

- [VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)

# 运行

## 1. dev模式
```bash
npm run tauri dev
```
或者
```bash
cargo tauri dev
```

## 2. build模式
先打包到 `dist` 目录：
```bash
npm run
```
然后安装服务器：
```bash
npm install -g http-server
```
然后启动服务器：
```bash
http-server dist
```