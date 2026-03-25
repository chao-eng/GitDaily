use rusqlite::Connection;
use std::sync::Mutex;
use tauri::{State, Window};
use crate::models::{AiConfig, CommitRecord};
use crate::services::ai_service::AiService;
use crate::errors::Result;
use crate::commands::TestResult;

#[tauri::command]
pub async fn test_ai_connection(
    config: AiConfig,
) -> Result<TestResult> {
    AiService::test_connection(config).await
}

#[tauri::command]
pub async fn generate_report(
    conn: State<'_, Mutex<Connection>>,
    prompt_content: String,
    commits: Vec<CommitRecord>,
) -> Result<String> {
    AiService::generate_report(&conn, prompt_content, commits).await
}

#[tauri::command]
pub async fn generate_report_stream(
    window: Window,
    conn: State<'_, Mutex<Connection>>,
    prompt_content: String,
    commits: Vec<CommitRecord>,
) -> Result<()> {
    AiService::generate_report_stream(window, &conn, prompt_content, commits).await
}
