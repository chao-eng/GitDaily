use rusqlite::Connection;
use std::sync::Mutex;
use tauri::State;
use crate::models::Report;
use crate::services::report_service::ReportService;
use crate::errors::Result;

#[tauri::command]
pub async fn list_reports(
    conn: State<'_, Mutex<Connection>>,
    date_from: Option<String>,
    date_to: Option<String>,
) -> Result<Vec<Report>> {
    ReportService::list_reports(&conn, date_from, date_to)
}

#[tauri::command]
pub async fn save_report(
    conn: State<'_, Mutex<Connection>>,
    report: Report,
) -> Result<i64> {
    ReportService::save_report(&conn, report)
}

#[tauri::command]
pub async fn delete_report(
    conn: State<'_, Mutex<Connection>>,
    id: i64,
) -> Result<()> {
    ReportService::delete_report(&conn, id)
}

#[tauri::command]
pub async fn get_report_dates(
    conn: State<'_, Mutex<Connection>>,
) -> Result<Vec<String>> {
    ReportService::get_report_dates(&conn)
}

#[tauri::command]
pub async fn get_activity_data(
    conn: State<'_, Mutex<Connection>>,
) -> Result<std::collections::HashMap<String, i32>> {
    ReportService::get_activity_data(&conn)
}
