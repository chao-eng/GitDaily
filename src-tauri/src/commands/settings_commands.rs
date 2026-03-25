use rusqlite::Connection;
use std::sync::Mutex;
use tauri::State;
use crate::services::prompt_service::PromptService;
use crate::errors::Result;

#[tauri::command]
pub async fn get_settings(
    conn: State<'_, Mutex<Connection>>,
) -> Result<serde_json::Value> {
    PromptService::get_settings(&conn)
}

#[tauri::command]
pub async fn update_settings(
    conn: State<'_, Mutex<Connection>>,
    settings: serde_json::Value,
) -> Result<()> {
    PromptService::update_settings(&conn, settings)
}
