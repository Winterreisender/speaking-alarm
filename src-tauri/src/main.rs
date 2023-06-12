// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]


mod tray;
mod speaker;
mod timer;

#[tauri::command]
fn time_report(hour12 :bool) -> Result<(), String> {
    speaker::time_report(hour12).map_err(|it| it.to_string())
}

#[tokio::main]
async fn main() {
    timer::start_timer();
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::default().build())
        .system_tray(tray::menu())
        .on_system_tray_event(tray::handler)
        .invoke_handler(tauri::generate_handler![time_report])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
