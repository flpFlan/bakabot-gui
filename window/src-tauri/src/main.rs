// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::default().build())
        .invoke_handler(tauri::generate_handler![
            open_bot_config,
            install_environment,
            uninstall_environment
        ])
        // .menu(build_menu())
        // .on_menu_event(listen_menu_event)
        .run(tauri::generate_context!())
        .expect("error while running bakabot-gui!");
}

// fn build_menu() -> Menu {
//     let install = CustomMenuItem::new("install".to_string(), "安装环境");
//     let config = CustomMenuItem::new("config".to_string(), "打开bot配置文件");
//     let start = CustomMenuItem::new("start".to_string(), "启动bot");
//     let menu = Menu::new()
//         .add_item(install)
//         .add_item(config)
//         .add_item(start);
//     menu
// }

// fn listen_menu_event(event: WindowMenuEvent) {
//     match event.menu_item_id() {
//         "config" => {
//             tauri::async_runtime::spawn(async move {
//                 if let Err(e) = open_bot_config().await {
//                     eprintln!("error while opening config file: {}", e);
//                 }
//             });
//         }
//         "install" => {
//             tauri::async_runtime::spawn(async move {
//                 if let Err(e) = install_enviroment().await {
//                     eprintln!("error while installing envirnment: {}", e);
//                 }
//             });
//         }
//         "start" => {
//             tauri::async_runtime::spawn(async move {
//                 if let Err(e) = start_bot().await {
//                     eprintln!("error while starting bot: {}", e);
//                 }
//             });
//         }
//         _ => {}
//     }
// }

// async fn start_bot() -> Result<(), Box<dyn std::error::Error>> {
//     use std::io::{BufRead, BufReader};
//     use std::process::{Command, Stdio};
//     use std::thread;

//     let mut child = Command::new("poetry")
//         .arg("run")
//         .arg("python ./bakabot/start_bot.py")
//         .stdout(Stdio::piped())
//         .spawn()?;

//         let stdout = child.stdout.take().expect("Failed to capture stdout");
//         let stderr = child.stderr.take().expect("Failed to capture stderr");
//         let stdout = BufReader::new(stdout);
//         let stderr = BufReader::new(stderr);

//         let handle1 = thread::spawn(move || {
//             for line in stdout.lines() {
//                 if let Ok(line) = line {
//                     println!("Stdout: {}", line);
//                 }
//             }
//         });

//         let handle2 = thread::spawn(move || {
//             for line in stderr.lines() {
//                 if let Ok(line) = line {
//                     eprintln!("Stderr: {}", line);
//                 }
//             }
//         });

//         let status = child.wait()?;
//         handle1.join().expect("WTF?!!");
//         handle2.join().expect("WTF?!!");

//         println!("Child process exited with: {:?}", status);

//         Ok(())
// }

#[tauri::command]
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

#[tauri::command]
async fn install_environment() -> Result<(), String> {
    let (terminal, command) = if cfg!(target_os = "windows") {
        ("cmd", r"/C start cmd /C .\resources\install")
    } else {
        ("bash", "-c ./resources/install")
    };
    match std::process::Command::new(terminal).arg(command).spawn() {
        Ok(_) => Ok(()),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
async fn uninstall_environment() -> Result<(), String> {
    let (terminal, command) = if cfg!(target_os = "windows") {
        ("cmd", r"/C start cmd /C .\resources\uninstall")
    } else {
        ("bash", "-c ./resources/uninstall")
    };
    match std::process::Command::new(terminal).arg(command).spawn() {
        Ok(_) => Ok(()),
        Err(e) => Err(e.to_string()),
    }
}
