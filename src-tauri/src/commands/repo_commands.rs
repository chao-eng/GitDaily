use rusqlite::Connection;
use std::sync::Mutex;
use tauri::State;
use crate::models::Repository;
use crate::services::repo_service::RepoService;
use crate::errors::Result;

#[tauri::command]
pub async fn add_repository(
    conn: State<'_, Mutex<Connection>>,
    path: String,
) -> Result<Repository> {
    RepoService::add_repository(&conn, &path)
}

#[tauri::command]
pub async fn list_repositories(
    conn: State<'_, Mutex<Connection>>,
) -> Result<Vec<Repository>> {
    RepoService::list_repositories(&conn)
}

#[tauri::command]
pub async fn remove_repository(
    conn: State<'_, Mutex<Connection>>,
    id: i64,
) -> Result<()> {
    RepoService::remove_repository(&conn, id)
}

#[tauri::command]
pub async fn toggle_repository(
    conn: State<'_, Mutex<Connection>>,
    id: i64,
    is_active: bool,
) -> Result<()> {
    RepoService::toggle_repository(&conn, id, is_active)
}

#[tauri::command]
pub fn validate_repo_path(path: String) -> bool {
    RepoService::validate_repo_path(&path)
}
