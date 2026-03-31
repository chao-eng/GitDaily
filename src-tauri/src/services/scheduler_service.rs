use std::sync::Mutex;
use rusqlite::Connection;
use tauri::{AppHandle, Manager};
use tokio::time::{interval, Duration};
use chrono::{Local, Datelike, Timelike};
use tauri_plugin_notification::NotificationExt;
use crate::models::{SchedulerConfig, ScheduleFrequency, GitLogQuery, Report};
use crate::services::{git_service::GitService, ai_service::AiService, report_service::ReportService};
use crate::errors::Result;

pub struct SchedulerState(pub Mutex<String>);

pub struct SchedulerService;

impl SchedulerService {
    pub async fn start_scheduler(app: AppHandle) {
        let mut interval = interval(Duration::from_secs(60)); // 每分钟检查一次

        loop {
            interval.tick().await;
            
            let now = Local::now();
            let today = now.format("%Y-%m-%d").to_string();

            // 检查缓存的执行日期
            {
                let state = app.state::<SchedulerState>();
                let last_date = state.0.lock().unwrap();
                if *last_date == today {
                    continue;
                }
            }

            // 获取连接并检查是否需要执行
            let conn_guard = app.state::<Mutex<Connection>>();
            let conn = conn_guard.inner();

            let config = match Self::get_config(conn) {
                Ok(c) => c,
                Err(_) => continue,
            };

            if !config.enabled {
                continue;
            }

            if !Self::should_run_today(&config, now) {
                continue;
            }

            if !Self::is_match_time(&config, now) {
                continue;
            }

            // 执行自动生成
            let result = Self::run_auto_generation(&app, conn).await;
            
            // 只有成功生成或明确跳过（今日无提交）才更新最后执行日期
            match &result {
                Ok(_) => {
                    let state = app.state::<SchedulerState>();
                    let mut last_date = state.0.lock().unwrap();
                    *last_date = today;
                },
                Err(_) => {} // 如果报错，下一分钟还可以重试
            }

            // 发送通知
            match &result {
                Ok(report) => {
                    let body = if report == "今日没有提交记录，跳过生成" {
                        report.clone()
                    } else {
                        "日报已自动生成并保存".to_string()
                    };
                    let _ = app.notification()
                        .builder()
                        .title("GitDaily 自动生成")
                        .body(body)
                        .show();
                }
                Err(e) => {
                    let _ = app.notification()
                        .builder()
                        .title("GitDaily 自动生成失败")
                        .body(e.to_string())
                        .show();
                }
            }
        }
    }

    pub fn get_config(conn: &Mutex<Connection>) -> Result<SchedulerConfig> {
        let conn = conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT key, value FROM settings WHERE key LIKE 'scheduler.%'")?;
        let rows = stmt.query_map([], |row| {
            Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
        })?;

        let mut config = SchedulerConfig::default();

        for row in rows {
            let (key, value) = row?;
            match key.as_str() {
                "scheduler.enabled" => config.enabled = value.parse::<bool>().unwrap_or(false),
                "scheduler.frequency" => {
                    config.frequency = match value.as_str() {
                        "weekly" => ScheduleFrequency::Weekly,
                        "workdays" => ScheduleFrequency::Workdays,
                        _ => ScheduleFrequency::Daily,
                    };
                }
                "scheduler.hour" => config.hour = value.parse::<u8>().unwrap_or(18),
                "scheduler.minute" => config.minute = value.parse::<u8>().unwrap_or(30),
                "scheduler.day_of_week" => config.day_of_week = value.parse::<u8>().ok(),
                "scheduler.repo_ids" => {
                    if let Ok(ids) = serde_json::from_str::<Vec<i64>>(&value) {
                        config.repo_ids = ids;
                    }
                }
                "scheduler.prompt_id" => config.prompt_id = value.parse::<i64>().ok(),
                _ => {}
            }
        }

        Ok(config)
    }

    pub fn update_config(conn: &Mutex<Connection>, config: SchedulerConfig) -> Result<()> {
        let conn = conn.lock().unwrap();

        let freq_str = match config.frequency {
            ScheduleFrequency::Daily => "daily",
            ScheduleFrequency::Weekly => "weekly",
            ScheduleFrequency::Workdays => "workdays",
        };

        let settings = vec![
            ("scheduler.enabled", config.enabled.to_string()),
            ("scheduler.frequency", freq_str.to_string()),
            ("scheduler.hour", config.hour.to_string()),
            ("scheduler.minute", config.minute.to_string()),
            ("scheduler.day_of_week", config.day_of_week.map(|d| d.to_string()).unwrap_or_default()),
            ("scheduler.repo_ids", serde_json::to_string(&config.repo_ids).unwrap_or_default()),
            ("scheduler.prompt_id", config.prompt_id.map(|d| d.to_string()).unwrap_or_default()),
        ];

        for (key, value) in settings {
            if value.is_empty() {
                continue;
            }
            conn.execute(
                "INSERT INTO settings (key, value) VALUES (?1, ?2) ON CONFLICT(key) DO UPDATE SET value = ?2, updated_at = datetime('now', 'localtime')",
                rusqlite::params![key, value],
            )?;
        }

        Ok(())
    }

    pub async fn run_auto_generation(app: &AppHandle, conn: &Mutex<Connection>) -> Result<String> {
        let config = Self::get_config(conn)?;
        if config.repo_ids.is_empty() {
            // 如果没有选择仓库，使用所有启用的仓库
            let ids: Vec<i64> = {
                let conn_lock = conn.lock().unwrap();
                let mut stmt = conn_lock.prepare("SELECT id FROM repositories WHERE is_active = 1")?;
                let x = stmt.query_map([], |row| row.get(0))?.collect::<rusqlite::Result<Vec<i64>>>()?;
                x
            };
            let config = SchedulerConfig {
                repo_ids: ids,
                ..config
            };

            Self::do_generate(app, conn, config).await
        } else {
            Self::do_generate(app, conn, config).await
        }
    }

    async fn do_generate(_app: &AppHandle, conn: &Mutex<Connection>, config: SchedulerConfig) -> Result<String> {
        // 1. Get the date range for today
        let today = Local::now().format("%Y-%m-%d").to_string();

        // 2. Fetch commits
        let query = GitLogQuery {
            repo_ids: config.repo_ids.clone(),
            date_from: today.clone(),
            date_to: today.clone(),
            author: None,
        };

        let commits = GitService::fetch_commits(conn, query)?;

        if commits.is_empty() {
            return Ok("今日没有提交记录，跳过生成".to_string());
        }

        // 3. Get prompt content
        let prompt_content = if let Some(prompt_id) = config.prompt_id {
            let conn = conn.lock().unwrap();
            let mut stmt = conn.prepare("SELECT content FROM prompts WHERE id = ?1")?;
            stmt.query_row([prompt_id], |row| row.get(0)).unwrap_or_else(|_| {
                Self::get_default_prompt(&conn)
            })
        } else {
            let conn = conn.lock().unwrap();
            Self::get_default_prompt(&conn)
        };

        // 4. Generate report with AI
        let content = AiService::generate_report(conn, prompt_content, commits.clone()).await?;

        // 5. Save report to database
        let report = Report {
            id: 0,
            date: today,
            raw_commits: serde_json::to_string(&commits).unwrap_or_default(),
            content: content.clone(),
            repo_ids: serde_json::to_string(&config.repo_ids).unwrap_or_default(),
            prompt_id: config.prompt_id,
            created_at: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
        };

        ReportService::save_report(conn, report)?;

        Ok(content)
    }

    fn get_default_prompt(conn: &Connection) -> String {
        let _ = conn;
        // Default daily report prompt
        "你是一个专业的技术日报助手，请基于我今天的 Git 提交记录，帮我生成一份清晰、专业的日报。\n要求：\n1. 按功能/模块分类整理工作内容\n2. 使用简洁专业的语言描述\n3. 突出完成的工作和解决的问题\n4. 格式使用 Markdown\n5. 控制在 300 字以内".to_string()
    }

    fn should_run_today(config: &SchedulerConfig, now: chrono::DateTime<Local>) -> bool {
        let weekday = now.weekday().num_days_from_sunday(); // 0 = Sunday, 6 = Saturday

        match config.frequency {
            ScheduleFrequency::Daily => true,
            ScheduleFrequency::Weekly => {
                if let Some(config_day) = config.day_of_week {
                    config_day as u32 == weekday
                } else {
                    false
                }
            }
            ScheduleFrequency::Workdays => {
                // Monday to Friday (1-5)
                weekday >= 1 && weekday <= 5
            }
        }
    }

    fn is_match_time(config: &SchedulerConfig, now: chrono::DateTime<Local>) -> bool {
        let current_hour = now.hour() as u8;
        let current_minute = now.minute() as u8;

        // 如果当前时间已经超过了设定的时间，就认为匹配（配合顶层的 last_execution_date 检查实现补跑）
        if current_hour > config.hour {
            return true;
        }

        if current_hour == config.hour && current_minute >= config.minute {
            return true;
        }

        false
    }
}
