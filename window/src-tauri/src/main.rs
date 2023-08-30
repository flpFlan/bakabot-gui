// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{CustomMenuItem, Menu, WindowMenuEvent};

async fn open_bot_config() -> Result<(), String> {
    let editor = if cfg!(target_os = "windows") {
        "notepad"
    } else if cfg!(target_os = "macos") {
        "open"
    } else {
        "vi"
    };

    match std::process::Command::new(editor)
        .arg("./bakabot/config.ini")
        .spawn()
    {
        Ok(_) => Ok(()),
        Err(e) => Err(e.to_string()),
    }
}

async fn install_enviroment() -> Result<(), String> {
    let (terminal, command) = if cfg!(target_os = "windows") {
        ("cmd", r"/C .\resources\install")
    } else {
        ("bash", "-c ./resources/install")
    };
    match std::process::Command::new(terminal).arg(command).spawn() {
        Ok(_) => Ok(()),
        Err(e) => Err(e.to_string()),
    }
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::default().build())
        // .invoke_handler(tauri::generate_handler![open_bot_config])
        .menu(build_menu())
        .on_menu_event(listen_menu_event)
        .run(tauri::generate_context!())
        .expect("error while running bakabot-gui!");
}

fn build_menu() -> Menu {
    let install = CustomMenuItem::new("install".to_string(), "安装环境");
    let config = CustomMenuItem::new("config".to_string(), "打开bot配置文件");
    let menu = Menu::new().add_item(install).add_item(config);
    menu
}

fn listen_menu_event(event: WindowMenuEvent) {
    match event.menu_item_id() {
        "config" => {
            tauri::async_runtime::spawn(async move {
                if let Err(e) = open_bot_config().await {
                    eprintln!("error while opening config file: {}", e);
                }
            });
        }
        "install" => {
            tauri::async_runtime::spawn(async move {
                if let Err(e) = install_enviroment().await {
                    eprintln!("error while installing envirnment: {}", e);
                }
            });
        }
        _ => {}
    }
}
