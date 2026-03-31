pub mod errors;
pub mod models;
pub mod commands;
pub mod services;
pub mod db;
pub mod utils;

use tauri::Manager;
use tauri::tray::{TrayIconBuilder, TrayIconEvent};
use tauri::menu::{MenuBuilder, MenuItem, PredefinedMenuItem};
use std::sync::Mutex;
use tauri::WindowEvent;
use tauri_plugin_notification::NotificationExt;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            // 创建系统托盘菜单
            let tray_menu = MenuBuilder::new(app)
                .item(&MenuItem::with_id(app, "open", "打开主窗口", true, None::<&str>)?)
                .item(&PredefinedMenuItem::separator(app)?)
                .item(&MenuItem::with_id(app, "quit", "退出", true, None::<&str>)?)
                .build()?;

            let tray_icon = tauri::image::Image::from_bytes(include_bytes!("../../public/logo.png"))?;
            let _tray = TrayIconBuilder::new()
                .icon(tray_icon)
                .menu(&tray_menu)
                .show_menu_on_left_click(false)
                .on_menu_event(|app, event| {
                    match event.id.as_ref() {
                        "open" => {
                            #[cfg(target_os = "macos")]
                            let _ = app.set_activation_policy(tauri::ActivationPolicy::Regular);
                            if let Some(window) = app.get_webview_window("main") {
                                window.show().unwrap();
                                window.set_focus().unwrap();
                            }
                        }
                        "quit" => {
                            app.exit(0);
                        }
                        _ => {}
                    }
                })
                .on_tray_icon_event(|tray, event| {
                    if let TrayIconEvent::Click {
                        button: tauri::tray::MouseButton::Left,
                        ..
                    } = event
                    {
                        let app = tray.app_handle();
                        #[cfg(target_os = "macos")]
                        let _ = app.set_activation_policy(tauri::ActivationPolicy::Regular);
                        if let Some(window) = app.get_webview_window("main") {
                            window.show().unwrap();
                            window.set_focus().unwrap();
                        }
                    }
                })
                .build(app)?;

            let app_data_dir = app.path().app_data_dir().expect("Failed to get app data dir");
            let conn = db::connection::init_db(&app_data_dir).expect("Failed to init database");
            db::migrations::run_migrations(&conn).expect("Failed to run migrations");

            let conn_mutex = Mutex::new(conn);
            app.manage(conn_mutex);

            // 启动定时调度器
            let handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                // 确保有权限发送通知
                #[cfg(not(target_os = "android"))]
                let _ = handle.notification().request_permission();
                
                services::scheduler_service::SchedulerService::start_scheduler(handle).await;
            });

            Ok(())
        })
        .on_window_event(|window, event| match event {
            WindowEvent::CloseRequested { api, .. } => {
                let app = window.app_handle();
                let conn_guard = app.state::<Mutex<rusqlite::Connection>>();
                
                // 检查是否开启了后台常驻（由自动生成开关决定）
                let hide_to_tray = if let Ok(config) = services::scheduler_service::SchedulerService::get_config(&conn_guard) {
                    config.enabled
                } else {
                    false
                };

                if hide_to_tray {
                    #[cfg(target_os = "macos")]
                    let _ = app.set_activation_policy(tauri::ActivationPolicy::Accessory);
                    window.hide().unwrap();
                    api.prevent_close();
                } else {
                    app.exit(0);
                }
            }
            _ => {}
        })
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_window_state::Builder::new().build())
        .plugin(tauri_plugin_notification::init())
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
            commands::scheduler_commands::get_scheduler_config,
            commands::scheduler_commands::update_scheduler_config,
            commands::scheduler_commands::trigger_auto_generation,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
