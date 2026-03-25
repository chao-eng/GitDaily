use reqwest::Client;
use std::sync::Mutex;
use rusqlite::Connection;
use tauri::{Window, Emitter};
use futures_util::StreamExt;
use crate::models::{AiConfig, ChatMessage, ChatCompletionRequest, CommitRecord};
use crate::errors::{AppError, Result};
use crate::commands::TestResult;

pub struct AiService;

impl AiService {
    pub async fn test_connection(config: AiConfig) -> Result<TestResult> {
        let client = Client::new();
        let url = format!("{}/chat/completions", config.api_base_url.trim_end_matches('/'));
        
        let request = ChatCompletionRequest {
            model: config.model_name,
            messages: vec![ChatMessage {
                role: "user".to_string(),
                content: "Hello".to_string(),
            }],
            stream: false,
            temperature: 0.7,
            max_tokens: Some(10),
        };

        let response = client
            .post(url)
            .header("Authorization", format!("Bearer {}", config.api_key))
            .json(&request)
            .send()
            .await?;

        if response.status().is_success() {
            Ok(TestResult {
                success: true,
                message: "连接成功".to_string(),
            })
        } else {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            Ok(TestResult {
                success: false,
                message: format!("连接失败 ({}): {}", status, body),
            })
        }
    }

    pub async fn generate_report(
        conn: &Mutex<Connection>,
        prompt_content: String,
        commits: Vec<CommitRecord>,
    ) -> Result<String> {
        // 1. Get AI config from DB
        let config = Self::get_ai_config(conn)?;
        
        // 2. Prepare prompt
        let commits_text = Self::format_commits_for_ai(commits);
        let user_content = format!("请基于以下提交记录生成日报：\n\n{}", commits_text);

        // 3. Call API
        let client = Client::new();
        let url = format!("{}/chat/completions", config.api_base_url.trim_end_matches('/'));
        
        let request = ChatCompletionRequest {
            model: config.model_name,
            messages: vec![
                ChatMessage { role: "system".to_string(), content: prompt_content },
                ChatMessage { role: "user".to_string(), content: user_content },
            ],
            stream: false,
            temperature: config.temperature,
            max_tokens: Some(config.max_tokens),
        };

        let response = client
            .post(url)
            .header("Authorization", format!("Bearer {}", config.api_key))
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await?;
            return Err(AppError::AiError(format!("AI 后端响应码 {}: {}", status, error_text)));
        }

        let json: serde_json::Value = response.json().await?;
        let content = json["choices"][0]["message"]["content"]
            .as_str()
            .ok_or_else(|| AppError::AiError("解析 AI 响应失败".to_string()))?;

        Ok(content.to_string())
    }

    pub async fn generate_report_stream(
        window: Window,
        conn: &Mutex<Connection>,
        prompt_content: String,
        commits: Vec<CommitRecord>,
    ) -> Result<()> {
        let config = Self::get_ai_config(&conn)?;
        let commits_text = Self::format_commits_for_ai(commits);
        let user_content = format!("请基于以下提交记录生成日报：\n\n{}", commits_text);

        let client = Client::new();
        let url = format!("{}/chat/completions", config.api_base_url.trim_end_matches('/'));
        
        let request = ChatCompletionRequest {
            model: config.model_name,
            messages: vec![
                ChatMessage { role: "system".to_string(), content: prompt_content },
                ChatMessage { role: "user".to_string(), content: user_content },
            ],
            stream: true,
            temperature: config.temperature,
            max_tokens: Some(config.max_tokens),
        };

        let response = client
            .post(url)
            .header("Authorization", format!("Bearer {}", config.api_key))
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await?;
            return Err(AppError::AiError(format!("AI 后端响应码 {}: {}", status, error_text)));
        }

        let mut stream = response.bytes_stream();
        let mut full_content = String::new();

        while let Some(item) = stream.next().await {
            let chunk = item?;
            let text = String::from_utf8_lossy(&chunk);
            
            for line in text.lines() {
                if line.starts_with("data: ") {
                    let data = line.trim_start_matches("data: ").trim();
                    if data == "[DONE]" {
                        window.emit("report-stream-done", &full_content).unwrap();
                        break;
                    }
                    
                    if let Ok(json) = serde_json::from_str::<serde_json::Value>(data) {
                        if let Some(content) = json["choices"][0]["delta"]["content"].as_str() {
                            full_content.push_str(content);
                            window.emit("report-stream-chunk", content).unwrap();
                        }
                    }
                }
            }
        }

        Ok(())
    }

    fn get_ai_config(conn: &Mutex<Connection>) -> Result<AiConfig> {
        let conn = conn.lock().unwrap();
        let mut config = AiConfig::default();
        
        let mut stmt = conn.prepare("SELECT key, value FROM settings WHERE key LIKE 'ai.%'")?;
        let rows = stmt.query_map([], |row| {
            Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
        })?;

        for row in rows {
            let (key, value) = row?;
            match key.as_str() {
                "ai.api_base_url" => config.api_base_url = value,
                "ai.api_key" => config.api_key = value,
                "ai.model_name" => config.model_name = value,
                "ai.temperature" => config.temperature = value.parse().unwrap_or(0.5),
                "ai.max_tokens" => config.max_tokens = value.parse().unwrap_or(2048),
                _ => {}
            }
        }
        
        Ok(config)
    }

    fn format_commits_for_ai(commits: Vec<CommitRecord>) -> String {
        let mut text = String::new();
        for (i, c) in commits.iter().enumerate() {
            text.push_str(&format!("{}. [{}] {} - {}\n", i + 1, c.short_hash, c.repo_name, c.message));
            if let Some(stat) = &c.diff_stat {
                if !stat.files_changed.is_empty() {
                    text.push_str(&format!("   变更文件: {}\n", stat.files_changed.join(", ")));
                }
                text.push_str(&format!("   统计: +{} insertions, -{} deletions\n", stat.insertions, stat.deletions));
            }
            text.push_str("\n");
        }
        text
    }
}
