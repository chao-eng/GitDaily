use std::sync::Mutex;
use rusqlite::Connection;
use serde_json::json;
use crate::errors::{AppError, Result};
use crate::services::prompt_service::PromptService;

pub struct NotificationService;

impl NotificationService {
    pub async fn send_feishu_markdown(
        webhook_url: &str,
        title: &str,
        markdown_content: &str,
    ) -> Result<()> {
        let client = reqwest::Client::new();
        let cleaned_content = Self::preprocess_markdown_for_feishu(markdown_content);

        // 构造飞书消息卡片 JSON
        let payload = json!({
            "msg_type": "interactive",
            "card": {
                "header": {
                    "title": {
                        "tag": "plain_text",
                        "content": title
                    },
                    "template": "blue" // 默认蓝色主题，AI 可根据通知紧急度动态调整
                },
                "elements": [
                    {
                        "tag": "markdown",
                        "content": cleaned_content
                    }
                ]
            }
        });

        let response = client.post(webhook_url)
            .json(&payload)
            .send()
            .await?;

        if response.status().is_success() {
            // 进一步校验返回的 json 是否包含 error code (如飞书接口层面的报错)
            let res_body: serde_json::Value = response.json().await.unwrap_or_default();
            if let Some(code) = res_body.get("code") {
                if code.as_i64().unwrap_or(0) != 0 {
                    let msg = res_body.get("msg").and_then(|m| m.as_str()).unwrap_or("未知错误");
                    return Err(AppError::AiError(format!("飞书接口错误: {}", msg)));
                }
            }
            Ok(())
        } else {
            Err(AppError::AiError(format!("飞书响应状态码异常: {}", response.status())))
        }
    }

    pub fn trigger_report_notification(
        conn_mutex: &Mutex<Connection>,
        date: &str,
        content: &str,
    ) {
        // 读取配置
        let settings = match PromptService::get_settings(conn_mutex) {
            Ok(s) => s,
            Err(_) => return,
        };

        let notify_on_generate = settings
            .get("notification.notify_on_generate")
            .and_then(|v| v.as_str())
            .map(|s| s == "true")
            .unwrap_or(false);

        let feishu_enabled = settings
            .get("notification.feishu_enabled")
            .and_then(|v| v.as_str())
            .map(|s| s == "true")
            .unwrap_or(false);

        let feishu_webhook_url = settings
            .get("notification.feishu_webhook_url")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
            .unwrap_or_default();

        if notify_on_generate && feishu_enabled && !feishu_webhook_url.is_empty() {
            let title = format!("GitDaily 日报已生成 ({})", date);
            let content_clone = content.to_string();
            // 异步发送，不阻塞当前线程
            tauri::async_runtime::spawn(async move {
                if let Err(e) = Self::send_feishu_markdown(&feishu_webhook_url, &title, &content_clone).await {
                    eprintln!("发送飞书通知失败: {:?}", e);
                }
            });
        }
    }

    fn preprocess_markdown_for_feishu(content: &str) -> String {
        let mut result = String::new();
        for line in content.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with('#') {
                let hashes_count = trimmed.chars().take_while(|&c| c == '#').count();
                if hashes_count > 0 {
                    let header_text = trimmed[hashes_count..].trim();
                    // 飞书消息卡片不支持标准 Markdown 标题，替换为粗体 ** 以达到优雅排版效果
                    result.push_str(&format!("**{}**\n", header_text));
                    continue;
                }
            }
            result.push_str(line);
            result.push_str("\n");
        }
        result
    }
}
