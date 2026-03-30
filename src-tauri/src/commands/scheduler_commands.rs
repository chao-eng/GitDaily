use rusqlite::Connection;
use std::sync::Mutex;
use tauri::AppHandle;
use tauri::State;
use crate::models::SchedulerConfig;
use crate::services::scheduler_service::SchedulerService;
use crate::errors::Result;

#[tauri::command]
pub async fn get_scheduler_config(
    conn: State<'_, Mutex<Connection>>,
) -> Result<SchedulerConfig> {
    SchedulerService::get_config(&conn)
}

#[tauri::command]
pub async fn update_scheduler_config(
    conn: State<'_, Mutex<Connection>>,
    config: SchedulerConfig,
) -> Result<()> {
    SchedulerService::update_config(&conn, config)
}

#[tauri::command]
pub async fn trigger_auto_generation(
    app: AppHandle,
    conn: State<'_, Mutex<Connection>>,
) -> Result<String> {
    SchedulerService::run_auto_generation(&app, &conn).await
}
