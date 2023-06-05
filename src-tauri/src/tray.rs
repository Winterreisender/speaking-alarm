use tauri::{AppHandle, SystemTrayEvent, SystemTray, CustomMenuItem, SystemTrayMenu, SystemTrayMenuItem, SystemTraySubmenu};

use crate::speaker;

// 托盘菜单
pub fn menu() -> SystemTray {
    let tray_menu = SystemTrayMenu::new()
        .add_item(CustomMenuItem::new("show".to_string(), "显示")) 
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(CustomMenuItem::new("quit".to_string(), "退出"));

    SystemTray::new().with_menu(tray_menu)
}

// 菜单事件
pub fn handler(_: &AppHandle, event: SystemTrayEvent) {
    match event {
        SystemTrayEvent::LeftClick { position: _, size: _, .. } => {
            speaker::time_report().unwrap();
        }

        SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
            "quit" => {
                std::process::exit(0);
            }
            "show" => {

            }
            _ => {}
        },
        _ => {}
    }
}