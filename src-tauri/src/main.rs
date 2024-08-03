use serde::{Deserialize, Serialize};
#[cfg_attr(mobile, tauri::mobile_entry_point)]
use tauri::{Emitter, Manager};
use tauri::menu::{MenuBuilder, MenuItemBuilder};
use tauri::tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent};
use tauri_plugin_dialog::{DialogExt, MessageDialogKind};

mod utils;
use utils::{check_update, restart, get_config, hide_or_show, log_info};

#[derive(Serialize, Deserialize, Debug)]
struct CheckItem {
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
    check_list: Vec<CheckItem>,
    position: PositionConfig,
}

#[derive(Serialize, Clone)]
struct Link {
    link: String,
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            let help_ = MenuItemBuilder::new("帮助(H)").id("help").build(app).unwrap();
            let quit = MenuItemBuilder::new("退出(X)").id("quit").build(app).unwrap();
            let hide = MenuItemBuilder::new("隐藏(H)").id("hide").build(app).unwrap();
            let about = MenuItemBuilder::new("关于(A)").id("about").build(app).unwrap();
            let update = MenuItemBuilder::new("检查更新(U)").id("update").build(app).unwrap();
            let restart_ = MenuItemBuilder::new("重启(R)").id("restart").build(app).unwrap();
            let tray_menu = MenuBuilder::new(app)
                .items(&[&help_, &update, &restart_, &about, &hide, &quit]) // insert the menu items here
                .build()
                .unwrap();
            let _ = TrayIconBuilder::new()
                .icon(app.default_window_icon().unwrap().clone())
                .menu(&tray_menu)
                .on_menu_event(|app, event| match event.id().as_ref() {
                    "help" => app.emit("open_link", Some(Link{link: "https://github.com/initialencounter/checkList?tab=readme-ov-file#使用帮助".to_string() })).unwrap(),
                    "quit" => app.exit(0),
                    "hide" => {
                        let window = app.get_webview_window("main").unwrap();
                        hide_or_show(window);
                    }
                    "restart" => restart(),
                    "about" => app.emit("open_link", Some(Link{link: "https://github.com/initialencounter/checkList".to_string() })).unwrap(),
                    "update" => {
                        let current_version = format!("v{}", env!("CARGO_PKG_VERSION"));
                        let latest = check_update(String::from("000"));
                        if latest == "000" {
                            app.dialog().message("检查更新失败!").kind(MessageDialogKind::Error).show(|_| {});
                        } else if latest != current_version {
                            app.dialog().message(format!("发现新版本{}，是否前往", latest)).kind(MessageDialogKind::Info).show(|_| {});
                            app.emit("open_link", Some(Link{link: "https://github.com/initialencounter/checkList/releases/latest".to_string() })).unwrap();
                        } else {
                            app.dialog().message("当前版本是最新版").kind(MessageDialogKind::Info).show(|_| {});
                        }
                    }
                    _ => {}
                })
                .on_tray_icon_event(|tray, event| {
                    if let TrayIconEvent::Click {
                        button: MouseButton::Left,
                        button_state: MouseButtonState::Up,
                        ..
                    } = event
                    {
                        let app = tray.app_handle();
                        if let Some(window) = app.get_webview_window("main") {
                            hide_or_show(window);
                        }
                    }
                })
                .build(app).unwrap();
            app.get_webview_window("main").unwrap().set_always_on_top(true).expect("Failed to set window as topmost");
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![restart, get_config, log_info])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
