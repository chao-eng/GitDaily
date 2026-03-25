pub mod errors;
pub mod models;
pub mod commands;
pub mod services;
pub mod db;
pub mod utils;

use tauri::Manager;
use std::sync::Mutex;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let app_data_dir = app.path().app_data_dir().expect("Failed to get app data dir");
            let conn = db::connection::init_db(&app_data_dir).expect("Failed to init database");
            db::migrations::run_migrations(&conn).expect("Failed to run migrations");
            
            app.manage(Mutex::new(conn));
            
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_window_state::Builder::new().build())
        .invoke_handler(tauri::generate_handler![
            commands::repo_commands::add_repository,
            commands::repo_commands::list_repositories,
            commands::repo_commands::remove_repository,
            commands::repo_commands::toggle_repository,
            commands::repo_commands::validate_repo_path,
            commands::git_commands::fetch_commits,
            commands::git_commands::get_git_user_name,
            commands::ai_commands::generate_report,
            commands::ai_commands::generate_report_stream,
            commands::ai_commands::test_ai_connection,
            commands::report_commands::list_reports,
            commands::report_commands::save_report,
            commands::report_commands::delete_report,
            commands::report_commands::get_report_dates,
            commands::prompt_commands::list_prompts,
            commands::prompt_commands::create_prompt,
            commands::prompt_commands::update_prompt,
            commands::prompt_commands::delete_prompt,
            commands::prompt_commands::set_default_prompt,
            commands::settings_commands::get_settings,
            commands::settings_commands::update_settings,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
