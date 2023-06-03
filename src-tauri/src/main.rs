// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]


mod tray;
mod speaker;

#[tauri::command]
fn time_report(name: &str) -> String {
    speaker::time_report();

    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    tauri::Builder::default()
        .system_tray(tray::menu())
        .on_system_tray_event(tray::handler)
        .invoke_handler(tauri::generate_handler![time_report])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
