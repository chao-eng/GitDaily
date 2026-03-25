use rusqlite::Connection;
use std::sync::Mutex;
use tauri::State;
use crate::models::{CommitRecord, GitLogQuery};
use crate::services::git_service::GitService;
use crate::errors::Result;

#[tauri::command]
pub async fn fetch_commits(
    conn: State<'_, Mutex<Connection>>,
    query: GitLogQuery,
) -> Result<Vec<CommitRecord>> {
    GitService::fetch_commits(&conn, query)
}

#[tauri::command]
pub async fn get_git_user_name(repo_path: Option<String>) -> Result<String> {
    GitService::get_git_user_name(repo_path.as_deref())
}
