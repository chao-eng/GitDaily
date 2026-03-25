use rusqlite::{params, Connection};
use std::sync::Mutex;
use crate::models::Report;
use crate::errors::Result;

pub struct ReportService;

impl ReportService {
    pub fn list_reports(
        conn: &Mutex<Connection>,
        date_from: Option<String>,
        date_to: Option<String>,
    ) -> Result<Vec<Report>> {
        let conn = conn.lock().unwrap();
        let mut query = "SELECT id, date, raw_commits, content, repo_ids, prompt_id, created_at FROM reports".to_string();
        let mut params_vec: Vec<String> = Vec::new();

        if let (Some(from), Some(to)) = (date_from, date_to) {
            query.push_str(" WHERE date BETWEEN ?1 AND ?2");
            params_vec.push(from);
            params_vec.push(to);
        }
        
        query.push_str(" ORDER BY date DESC, created_at DESC");

        let mut stmt = conn.prepare(&query)?;
        let rows = stmt.query_map(rusqlite::params_from_iter(params_vec), |row| {
            Ok(Report {
                id: row.get(0)?,
                date: row.get(1)?,
                raw_commits: row.get(2)?,
                content: row.get(3)?,
                repo_ids: row.get(4)?,
                prompt_id: row.get(5)?,
                created_at: row.get(6)?,
            })
        })?;

        let mut reports = Vec::new();
        for row in rows {
            reports.push(row?);
        }
        Ok(reports)
    }

    pub fn save_report(
        conn: &Mutex<Connection>,
        report: Report,
    ) -> Result<i64> {
        let conn = conn.lock().unwrap();
        conn.execute(
            "INSERT INTO reports (date, raw_commits, content, repo_ids, prompt_id) VALUES (?1, ?2, ?3, ?4, ?5)",
            params![
                report.date,
                report.raw_commits,
                report.content,
                report.repo_ids,
                report.prompt_id,
            ],
        )?;
        Ok(conn.last_insert_rowid())
    }

    pub fn delete_report(conn: &Mutex<Connection>, id: i64) -> Result<()> {
        let conn = conn.lock().unwrap();
        conn.execute("DELETE FROM reports WHERE id = ?1", params![id])?;
        Ok(())
    }

    pub fn get_report_dates(conn: &Mutex<Connection>) -> Result<Vec<String>> {
        let conn = conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT DISTINCT date FROM reports ORDER BY date ASC")?;
        let rows = stmt.query_map([], |row| row.get(0))?;
        let mut dates = Vec::new();
        for row in rows {
            dates.push(row?);
        }
        Ok(dates)
    }
}
