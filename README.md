# GitDaily (日报之星) 🚀

GitDaily 是一款专为开发者设计的本地桌面级日报自动化工具。它通过扫描您的本地 Git 提交记录，结合 AI (LLM) 模型，自动为您生成结构清晰、内容专业的日报及周报。

项目采用了最新的 **Lark Design (飞书风格)** 视觉规范，旨在提供高密度、高专业度的企业级办公体验。

![GitDaily UI](https://img.shields.io/badge/UI-Lark_Design-blue?style=flat-square) ![Tech Stack](https://img.shields.io/badge/Stack-Tauri_+_Vue3_+_Rust-orange?style=flat-square)

---

## ✨ 核心特性

- 🏢 **飞书风格 UI (Lark Design)**: 全面重构的工业级界面，提供极致的信息密度与视觉愉悦感。
- 🤖 **AI 驱动生成**: 支持自定义 System Prompt，自由调整生成温度 (Temperature) 与长度，适配多种汇报场景（严谨版、幽默版、站会简报等）。
- 📂 **多仓库联动**: 支持同时挂载多个本地 Git 仓库，一键汇总跨项目的研发产出。
- 📊 **活跃度图谱**: 直观展示过去一年的日报生成轨迹，记录您的成长点滴。
- 📅 **历史回溯**: 完整的持久化存储，支持按日期筛选历史记录，一键复制或导出为 Markdown 文件。
- 🌓 **深色模式**: 完美适配深色主题，保护深夜 Coding 的双眼。
- 🔒 **本地优先**: 数据存储于本地 SQLite 数据库，API Key 等敏感信息仅保存在本地配置文件，保障隐私安全。

---

## 🛠️ 技术架构

- **前端 (Frontend)**: [Vue 3](https://vuejs.org/) + [TypeScript](https://www.typescriptlang.org/) + [Vite](https://vitejs.dev/) + [Tailwind CSS v4](https://tailwindcss.com/)
- **桌面框架 (Framework)**: [Tauri v2](https://v2.tauri.app/) (跨平台、高性能、包体积极小)
- **后端 (Backend)**: [Rust](https://www.rust-lang.org/) (处理 Git 流、SQLite 交互及 AI 聚合)
- **数据库 (Database)**: [SQLite](https://www.sqlite.org/) (高性能本地关系型数据库)
- **UI 组件库**: [Element Plus](https://element-plus.org/) (深度定制 Lark 主题)

---

## 🚀 快速开始

### 1. 环境准备
确保您的计算机已安装以下环境：
- [Node.js](https://nodejs.org/) (建议 v18+)
- [Rust](https://www.rust-lang.org/tools/install) 工具链

### 2. 获取代码
```bash
git clone https://github.com/your-repo/GitDaily.git
cd GitDaily
```

### 3. 安装依赖并启动
```bash
# 安装前端依赖
npm install

# 启动开发服务器 (Tauri dev)
npm run tauri dev
```

### 4. 配置 AI 服务
启动应用后，点击左侧菜单栏的 **“设置”**：
1. 填入您的 AI API 基础地址 (Base URL) 与 API Key。
2. 设置模型名称 (如 `gpt-4o-mini` 或 `claude-3-5-sonnet`)。
3. 点击“测试连通性”确保配置成功。

---

## 💡 使用小技巧

- **提示词模板**: 您可以在“模板管理”中根据自己的岗位需求（如前端、后端、架构师）修改内置模板。系统会自动记住您最近修改的内容。
- **Git 用户名**: 在设置中配置您的 Git 用户名，GitDaily 将更精准地过滤属于您的 Commit。
- **一键导出**: 在历史记录中，点击“导出”图标可直接将生成的日报保存为标准格式的 `.md` 文档，方便飞书/钉钉转发。

---

## 📄 开源协议
本项目遵循 MIT 协议。
