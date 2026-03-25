use rusqlite::{params, Connection};
use std::sync::Mutex;
use crate::models::Prompt;
use crate::errors::Result;

pub struct PromptService;

impl PromptService {
    pub fn list_prompts(conn: &Mutex<Connection>) -> Result<Vec<Prompt>> {
        let conn = conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT id, title, content, is_builtin, is_default, created_at FROM prompts ORDER BY updated_at DESC, created_at DESC")?;
        
        let iter = stmt.query_map([], |row| {
            Ok(Prompt {
                id: row.get(0)?,
                title: row.get(1)?,
                content: row.get(2)?,
                is_builtin: row.get::<_, i32>(3)? != 0,
                is_default: row.get::<_, i32>(4)? != 0,
                created_at: row.get(5)?,
            })
        })?;

        let mut results = Vec::new();
        for item in iter {
            results.push(item?);
        }
        Ok(results)
    }

    pub fn create_prompt(conn: &Mutex<Connection>, title: String, content: String) -> Result<Prompt> {
        let conn = conn.lock().unwrap();
        conn.execute(
            "INSERT INTO prompts (title, content, is_builtin, is_default) VALUES (?1, ?2, 0, 0)",
            params![title, content],
        )?;
        let id = conn.last_insert_rowid();
        Ok(Prompt {
            id,
            title,
            content,
            is_builtin: false,
            is_default: false,
            created_at: chrono::Local::now().to_string(),
        })
    }

    pub fn update_prompt(conn: &Mutex<Connection>, id: i64, title: String, content: String) -> Result<()> {
        let conn = conn.lock().unwrap();
        conn.execute(
            "UPDATE prompts SET title = ?1, content = ?2, is_builtin = 0, updated_at = datetime('now', 'localtime') WHERE id = ?3",
            params![title, content, id],
        )?;
        Ok(())
    }

    pub fn delete_prompt(conn: &Mutex<Connection>, id: i64) -> Result<()> {
        let conn = conn.lock().unwrap();
        conn.execute("DELETE FROM prompts WHERE id = ?1 AND is_builtin = 0", params![id])?;
        Ok(())
    }

    pub fn set_default_prompt(conn: &Mutex<Connection>, id: i64) -> Result<()> {
        let conn = conn.lock().unwrap();
        conn.execute("UPDATE prompts SET is_default = 0", [])?;
        conn.execute("UPDATE prompts SET is_default = 1 WHERE id = ?1", params![id])?;
        Ok(())
    }

    // Settings logic included here for convenience
    pub fn get_settings(conn: &Mutex<Connection>) -> Result<serde_json::Value> {
        let conn = conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT key, value FROM settings")?;
        let rows = stmt.query_map([], |row| {
            Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
        })?;

        let mut map = serde_json::Map::new();
        for row in rows {
            let (key, value) = row?;
            map.insert(key, serde_json::Value::String(value));
        }
        Ok(serde_json::Value::Object(map))
    }

    pub fn update_settings(conn: &Mutex<Connection>, settings: serde_json::Value) -> Result<()> {
        let conn = conn.lock().unwrap();
        let obj = settings.as_object().ok_or_else(|| crate::errors::AppError::RepoError("Invalid settings format".to_string()))?;
        
        for (key, value) in obj {
            let val_str = match value {
                serde_json::Value::String(s) => s.clone(),
                _ => value.to_string(),
            };
            conn.execute(
                "INSERT INTO settings (key, value) VALUES (?1, ?2) ON CONFLICT(key) DO UPDATE SET value = ?2, updated_at = datetime('now', 'localtime')",
                params![key, val_str],
            )?;
        }
        Ok(())
    }
}
