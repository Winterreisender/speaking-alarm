// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::{time::{Duration}};

use timer::set_integral_minutes;

mod tray;
mod speaker;
mod timer;
mod app_config;

#[tauri::command]
fn time_report() -> Result<(), String> {
    speaker::time_report().map_err(|it| it.to_string())
}

#[tauri::command]
fn submit_config(integral_minutes :Vec<u32>, hour12 :bool, date_report :&str) -> Result<(), String> {
    dbg!(&integral_minutes, hour12, date_report);
    set_integral_minutes(&integral_minutes);

    let date_report = app_config::DateReport::new(date_report);
    app_config::set_config(app_config::Config {hour12, date_report});
    Ok(())
}

#[tokio::main]
async fn main() {
    timer::start_timer(Duration::from_secs(60));
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::default().build())
        .system_tray(tray::menu())
        .on_system_tray_event(tray::handler)
        .invoke_handler(tauri::generate_handler![time_report,submit_config])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
