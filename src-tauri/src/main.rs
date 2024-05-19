// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use serde::{Deserialize, Serialize};
use tauri::{api::dialog, CustomMenuItem, LogicalPosition, Manager, Position, SystemTray, SystemTrayEvent, SystemTrayMenu};
mod utils;
use utils::{check_update, open_link, restart, set_window_topmost, get_config};

#[derive(Serialize, Deserialize, Debug)]
struct  CheckItme {
    name: String,
    state: bool
}

#[derive(Serialize, Deserialize, Debug)]
struct  PositionConfig {
    x: f64,
    y: f64
}

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    check_list: Vec<CheckItme>,
    position: PositionConfig,
}



fn main() {
    let quit = CustomMenuItem::new("quit".to_string(), "退出(X)");
    let hide = CustomMenuItem::new("hide".to_string(), "隐藏(H)");
    let about = CustomMenuItem::new("about".to_string(), "关于(A)");
    let update = CustomMenuItem::new("update".to_string(), "检查更新(U)");
    let restart_ = CustomMenuItem::new("restart".to_string(), "重启(R)");
    let tray_menu = SystemTrayMenu::new()
        .add_item(update)
        .add_item(restart_)
        .add_item(about)
        .add_item(hide)
        .add_item(quit); // insert the menu items here
    tauri::Builder::default()
        .system_tray(SystemTray::new().with_menu(tray_menu))
        .on_system_tray_event(|app, event| match event {
            SystemTrayEvent::LeftClick {
                position: _,
                size: _,
                ..
            } => {
                let window = match app.get_window("main") {
                    Some(a) => a,
                    None => panic!("Unkonw"),
                };
                if window.is_visible().expect("REASON") {
                    match window.hide() {
                        Ok(a) => a,
                        Err(e) => println!("{}", e.to_string()),
                    };
                } else {
                    match window.show() {
                        Ok(a) => a,
                        Err(e) => println!("{}", e.to_string()),
                    };
                }
            }
            SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
                "quit" => {
                    std::process::exit(0);
                }
                "hide" => {
                    let window = match app.get_window("main") {
                        Some(a) => a,
                        None => panic!("Unkonw"),
                    };
                    match window.hide() {
                        Ok(a) => a,
                        Err(e) => println!("{}", e.to_string()),
                    };
                }
                "about" => open_link("https://github.com/initialencounter/checkList"),
                "update" => {
                    let window = match app.get_window("main") {
                        Some(a) => a,
                        None => panic!("Unkonw"),
                    };
                    let current_version = format!("v{}", env!("CARGO_PKG_VERSION"));
                    let lastest = check_update(String::from("000"));
                    if lastest == "000" {
                        dialog::ask(Some(&window), "checkList", "检查更新失败!", |answer| {
                            match answer {
                                true => (),
                                false => (),
                            }
                        })
                    } else if lastest != current_version {
                        dialog::ask(
                            Some(&window),
                            "RainWarm",
                            format!("发现新版本{}，是否前往", lastest).as_str(),
                            |answer| match answer {
                                true => open_link(
                                    "https://github.com/initialencounter/checkList/releases/latest",
                                ),
                                false => (),
                            },
                        );
                    } else {
                        dialog::ask(
                            Some(&window),
                            "RainWarm",
                            format!("当前版本是最新版").as_str(),
                            |answer| match answer {
                                true => (),
                                false => (),
                            },
                        )
                    }
                }
                "restart" => restart(),
                _ => {}
            },
            _ => {}
        })
        .on_window_event(|event| match event.event() {
            tauri::WindowEvent::CloseRequested { api, .. } => {
                let _ = event.window().hide();
                api.prevent_close();
            }
            _ => {}
        })
        .on_page_load(|window, _| {
            let data = get_config();
            let config: Config = serde_json::from_str(data.as_str()).unwrap();
            let _ = window.set_position(Position::Logical(LogicalPosition {
                x: config.position.x as f64,
                y: config.position.y as f64
            }));
            set_window_topmost(window.clone());
        })
        .invoke_handler(tauri::generate_handler![open_link, restart, get_config])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
