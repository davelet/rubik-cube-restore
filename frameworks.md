# 项目使用的框架

本项目是一个使用多个现代框架的全栈应用程序：

## 前端框架
1. **Vue.js 3** - 主要的前端框架
   - 使用 Vue Router 4 进行路由管理
   - 使用 TypeScript 作为开发语言

2. **Vite** - 现代前端构建工具
   - 用于开发环境的快速热重载
   - 用于生产环境的构建优化

3. **Three.js** - 3D 图形库
   - 用于 3D 魔方的渲染和交互

4. **Tauri 2.0** - 跨平台桌面应用程序框架
   - 将 Web 前端与 Rust 后端结合
   - 提供原生性能和系统访问能力

## 后端框架
1. **Java** (java-puzzle-resolver 模块)
   - 使用 Maven 作为构建工具
   - JUnit 5 用于测试
   - Log4j 2 用于日志管理
   - Java 17 作为目标版本

2. **Rust** (src-tauri 模块)
   - Tauri 的核心后端
   - 用于与操作系统交互和性能关键型操作

## 构建和开发工具
- TypeScript 用于类型安全
- Maven 用于 Java 项目管理
- Cargo (Rust 的包管理器)
- npm/yarn 用于 JavaScript 依赖管理