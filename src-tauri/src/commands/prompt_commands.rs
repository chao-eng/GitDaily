use rusqlite::Connection;
use std::sync::Mutex;
use tauri::State;
use crate::models::Prompt;
use crate::services::prompt_service::PromptService;
use crate::errors::Result;

#[tauri::command]
pub async fn list_prompts(
    conn: State<'_, Mutex<Connection>>,
) -> Result<Vec<Prompt>> {
    PromptService::list_prompts(&conn)
}

#[tauri::command]
pub async fn create_prompt(
    conn: State<'_, Mutex<Connection>>,
    title: String,
    content: String,
) -> Result<Prompt> {
    PromptService::create_prompt(&conn, title, content)
}

#[tauri::command]
pub async fn update_prompt(
    conn: State<'_, Mutex<Connection>>,
    id: i64,
    title: String,
    content: String,
) -> Result<()> {
    PromptService::update_prompt(&conn, id, title, content)
}

#[tauri::command]
pub async fn delete_prompt(
    conn: State<'_, Mutex<Connection>>,
    id: i64,
) -> Result<()> {
    PromptService::delete_prompt(&conn, id)
}

#[tauri::command]
pub async fn set_default_prompt(
    conn: State<'_, Mutex<Connection>>,
    id: i64,
) -> Result<()> {
    PromptService::set_default_prompt(&conn, id)
}
