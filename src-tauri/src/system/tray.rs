use tauri::{
    AppHandle, CustomMenuItem, GlobalWindowEvent, Manager, SystemTray, SystemTrayEvent,
    SystemTrayMenu, SystemTrayMenuItem, WindowEvent,
};

// 托盘
pub fn system_tray() -> SystemTray {
    let tray_menu = SystemTrayMenu::new()
        .add_item(CustomMenuItem::new("hide".to_string(), "Hide"))
        .add_item(CustomMenuItem::new("show".to_string(), "Show"))
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(CustomMenuItem::new("quit".to_string(), "Quit"));

    SystemTray::new().with_menu(tray_menu)
}

// 菜单事件
pub fn tray_handler(app: &AppHandle, event: SystemTrayEvent) {
    // 获取应用窗口
    let window = app.get_window("main").unwrap();

    match event {
        SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
            "hide" => window.minimize().unwrap(),
            "show" => window.unminimize().unwrap(),
            "quit" => {
                std::process::exit(0);
            }
            _ => {}
        },
        SystemTrayEvent::LeftClick { .. } => {}
        SystemTrayEvent::RightClick { .. } => {}
        SystemTrayEvent::DoubleClick { .. } => {}
        _ => {}
    }
}

// window 事件
pub fn window_handler(event: GlobalWindowEvent) {
    match event.event() {
        WindowEvent::Resized(_) => {}
        WindowEvent::Moved(_) => {}
        WindowEvent::CloseRequested { api, .. } => {
            event.window().minimize().unwrap();
            api.prevent_close();
        }
        WindowEvent::Destroyed => {}
        WindowEvent::Focused(_) => {}
        WindowEvent::ScaleFactorChanged { .. } => {}
        WindowEvent::FileDrop(_) => {}
        WindowEvent::ThemeChanged(_) => {}
        _ => {}
    }
}
