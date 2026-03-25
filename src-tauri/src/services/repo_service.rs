use rusqlite::{params, Connection};
use chrono;
use std::path::Path;
use std::sync::Mutex;
use crate::models::Repository;
use crate::errors::{AppError, Result};

pub struct RepoService;

impl RepoService {
    pub fn add_repository(conn: &Mutex<Connection>, path: &str) -> Result<Repository> {
        let conn = conn.lock().unwrap();
        
        // 1. Validate if it's a git repo
        if !Self::validate_repo_path(path) {
            return Err(AppError::RepoError("无效的 Git 仓库路径".to_string()));
        }

        // 2. Extract name from path
        let name = Path::new(path)
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("未知仓库")
            .to_string();

        // 3. Insert into DB
        conn.execute(
            "INSERT INTO repositories (name, path, is_active) VALUES (?1, ?2, 1)",
            params![name, path],
        )?;

        let id = conn.last_insert_rowid();
        
        // 4. Return new repo
        Ok(Repository {
            id,
            name,
            path: path.to_string(),
            is_active: true,
            created_at: chrono::Local::now().to_string(),
        })
    }

    pub fn list_repositories(conn: &Mutex<Connection>) -> Result<Vec<Repository>> {
        let conn = conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT id, name, path, is_active, created_at FROM repositories ORDER BY created_at DESC")?;
        
        let repo_iter = stmt.query_map([], |row| {
            Ok(Repository {
                id: row.get(0)?,
                name: row.get(1)?,
                path: row.get(2)?,
                is_active: row.get::<_, i32>(3)? != 0,
                created_at: row.get(4)?,
            })
        })?;

        let mut repos = Vec::new();
        for repo in repo_iter {
            repos.push(repo?);
        }
        
        Ok(repos)
    }

    pub fn remove_repository(conn: &Mutex<Connection>, id: i64) -> Result<()> {
        let conn = conn.lock().unwrap();
        conn.execute("DELETE FROM repositories WHERE id = ?1", params![id])?;
        Ok(())
    }

    pub fn toggle_repository(conn: &Mutex<Connection>, id: i64, is_active: bool) -> Result<()> {
        let conn = conn.lock().unwrap();
        conn.execute(
            "UPDATE repositories SET is_active = ?1, updated_at = datetime('now', 'localtime') WHERE id = ?2",
            params![if is_active { 1 } else { 0 }, id],
        )?;
        Ok(())
    }

    pub fn validate_repo_path(path: &str) -> bool {
        let git_path = Path::new(path).join(".git");
        git_path.exists() && git_path.is_dir()
    }
}
