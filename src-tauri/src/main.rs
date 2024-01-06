// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

extern crate core;

use crate::dao::db;

mod dao;
mod service;
mod system;
mod router;

fn main() {
    db::init();
    tauri::Builder::default()
        .setup(|_app| {
            db::init();
            Ok({})
        })
        .system_tray(system::tray::system_tray())
        .on_system_tray_event(system::tray::tray_handler)
        .on_window_event(system::tray::window_handler)
        .invoke_handler(tauri::generate_handler![router::handlers::get_wallpaper,router::handlers::set_wallpaper,router::handlers::refresh])
        .run(tauri::generate_context!())
        .expect("程序启动失败。。");
}

