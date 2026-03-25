use std::path::Path;
use rusqlite::Connection;
use crate::errors::Result;

pub fn init_db(app_data_dir: &Path) -> Result<Connection> {
    if !app_data_dir.exists() {
        std::fs::create_dir_all(app_data_dir)?;
    }
    
    let db_path = app_data_dir.join("gitdaily.db");
    let conn = Connection::open(db_path)?;
    
    Ok(conn)
}
