use rusqlite::Connection;
use crate::errors::Result;

pub fn run_migrations(conn: &Connection) -> Result<()> {
    // 1. Create tables
    conn.execute(
        "CREATE TABLE IF NOT EXISTS repositories (
            id          INTEGER PRIMARY KEY AUTOINCREMENT,
            name        TEXT    NOT NULL,
            path        TEXT    NOT NULL UNIQUE,
            is_active   INTEGER NOT NULL DEFAULT 1,
            created_at  TEXT    NOT NULL DEFAULT (datetime('now', 'localtime')),
            updated_at  TEXT    NOT NULL DEFAULT (datetime('now', 'localtime'))
        )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS prompts (
            id          INTEGER PRIMARY KEY AUTOINCREMENT,
            title       TEXT    NOT NULL,
            content     TEXT    NOT NULL,
            is_builtin  INTEGER NOT NULL DEFAULT 0,
            is_default  INTEGER NOT NULL DEFAULT 0,
            created_at  TEXT    NOT NULL DEFAULT (datetime('now', 'localtime')),
            updated_at  TEXT    NOT NULL DEFAULT (datetime('now', 'localtime'))
        )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS reports (
            id          INTEGER PRIMARY KEY AUTOINCREMENT,
            date        TEXT    NOT NULL,
            raw_commits TEXT,
            content     TEXT    NOT NULL,
            repo_ids    TEXT,
            prompt_id   INTEGER,
            created_at  TEXT    NOT NULL DEFAULT (datetime('now', 'localtime')),
            updated_at  TEXT    NOT NULL DEFAULT (datetime('now', 'localtime'))
        )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS settings (
            key         TEXT    PRIMARY KEY,
            value       TEXT    NOT NULL,
            updated_at  TEXT    NOT NULL DEFAULT (datetime('now', 'localtime'))
        )",
        [],
    )?;

    // 2. Create indexes
    conn.execute("CREATE INDEX IF NOT EXISTS idx_reports_date ON reports(date)", [])?;
    conn.execute("CREATE INDEX IF NOT EXISTS idx_repositories_path ON repositories(path)", [])?;

    // 3. Insert default data (using a simple check)
    let count: i64 = conn.query_row("SELECT COUNT(*) FROM prompts", [], |r| r.get(0))?;
    if count == 0 {
        insert_default_prompts(conn)?;
    }

    Ok(())
}

fn insert_default_prompts(conn: &Connection) -> Result<()> {
    let defaults = vec![
        ("严谨正式版", "# Role: 资深研发项目经理兼技术文档专家\n\n# Objective\n你现在的任务是将繁杂的 Git 提交记录，提炼并转化为结构清晰、面向业务的高质量工作日报。\n\n# Rules & Constraints\n1. **业务视角翻译**：必须将纯技术面的描述（如 \"修复 NPE\", \"重构 UserAPI\"）转化为体现业务价值和用户视角的语言（如 \"修复了特定场景下的用户登录异常问题\", \"优化了用户数据加载性能\"）。避免堆砌代码级别的专业术语。\n2. **智能合并与提炼**：识别并合并属于同一功能点或任务的多个提交（例如针对同一功能的多次 WIP、前后端对接修改、bugfix），将其融合成一项完整的业务工作成果。\n3. **严格过滤噪音**：直接忽略无实际业务意义或极小颗粒度的提交，例如：\"fix typo\", \"format code\", \"update comments\", \"Merge branch\" 等。\n4. **模块化分组**：根据提交记录的上下文，自动推断并按「业务模块」或「功能区域」进行分类展示。\n5. **纯文本排版**：严禁使用表格。必须使用纯文本和基础 Markdown 语法（如标题和无序列表）进行输出，确保格式适合直接复制到邮件或工作群组中。\n\n# Output Format Template\n请严格按照以下结构输出：\n\n### 📅 工作日报\n**[可选：一句话总结今日核心产出]**\n\n#### 📌 [业务模块名称 A] (例如：用户系统 / 订单模块)\n- **[新增/优化/修复]** 用一句话描述业务层面的工作成果。（如果必要，可简要补充其解决的核心痛点）\n- **[新增/优化/修复]** 用一句话描述业务层面的工作成果。\n\n#### 📌 [业务模块名称 B] \n- ...\n\n#### 🔧 技术基建与日常维护 (若全天只有业务开发，此项可省略)\n- 描述非业务向但必要的底层技术优化、脚手架升级或环境配置等。\n\n---\n\n# Input\n请根据以下 Git 提交记录生成今天的工作日报：\n\n```text\n"),
        ("站会简报版", "你是一位敏捷开发教练。请根据以下 Git 提交记录，用极简的语言生成每日站会汇报内容。格式要求：**昨天完成了什么：** - 要点1 - 要点2。**今天计划做什么：** （基于昨天的工作上下文推断）。**遇到的阻碍：** （写\"暂无\"）。"),
        ("技术周报版", "你是一位资深架构师。请根据本周提交总结项目进展。要求分项列出主要 feature、bugfix、性能优化和基础建设，并规划下周工作重点。"),
        ("自由风格版", "你是一位风趣幽默的开发者。请根据记录用口语化的风格记录你今天忙碌的一天，包括写了哪些牛逼的功能、修了哪些离谱的 Bug。")
    ];

    for (title, content) in defaults {
        conn.execute(
            "INSERT INTO prompts (title, content, is_builtin, is_default) VALUES (?1, ?2, 1, ?3)",
            (title, content, if title == "严谨正式版" { 1 } else { 0 }),
        )?;
    }
    
    Ok(())
}
