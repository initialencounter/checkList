use serde::Serialize;
use tauri::menu::{MenuBuilder};
use tauri::{App, AppHandle, Emitter, Manager, PhysicalPosition, Wry};
use tauri::tray::{MouseButton, MouseButtonState, TrayIcon, TrayIconBuilder, TrayIconEvent};
use tauri_plugin_dialog::{DialogExt, MessageDialogKind};
use crate::config::{get_config_struct};
use crate::menu::create_menu_item;
use crate::utils::{check_update, hide_or_show, restart};


#[derive(Serialize, Clone)]
struct Link {
    link: String,
}

pub fn handle_update(app: &AppHandle<Wry>) {
    let current_version = format!("v{}", env!("CARGO_PKG_VERSION"));
    let latest = check_update(String::from("000"));
    if latest == "000" {
        app.dialog().message("检查更新失败!").kind(MessageDialogKind::Error).show(|_| {});
    } else if latest != current_version {
        app.dialog().message(format!("发现新版本{}，是否前往", latest)).kind(MessageDialogKind::Info).show(|_| {});
        app.emit("open_link", Some(Link { link: "https://github.com/initialencounter/checkList/releases/latest".to_string() })).unwrap();
    } else {
        app.dialog().message("当前版本是最新版").kind(MessageDialogKind::Info).show(|_| {});
    }
}

pub fn handle_tray_icon_event(tray: &TrayIcon, event: TrayIconEvent){
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
}

pub fn handle_setup(app: &App<Wry>){
    let [help_, quit, hide, about, update, restart_] = create_menu_item(app);
    let tray_menu = MenuBuilder::new(app)
        .items(&[&help_, &update, &restart_, &about, &hide, &quit]) // insert the menu items here
        .build()
        .unwrap();
    let _ = TrayIconBuilder::new()
        .icon(app.default_window_icon().unwrap().clone())
        .menu(&tray_menu)
        .on_menu_event(|app, event| match event.id().as_ref() {
            "help" => app.emit("open_link", Some(Link { link: "https://github.com/initialencounter/checkList?tab=readme-ov-file#使用帮助".to_string() })).unwrap(),
            "quit" => app.exit(0),
            "hide" => {
                let window = app.get_webview_window("main").unwrap();
                hide_or_show(window);
            }
            "restart" => restart(),
            "about" => app.emit("open_link", Some(Link { link: "https://github.com/initialencounter/checkList".to_string() })).unwrap(),
            "update" => handle_update(app),
            _ => {}
        })
        .on_tray_icon_event(handle_tray_icon_event)
        .build(app).unwrap();
    let window = app.get_webview_window("main").unwrap();
    let config = get_config_struct();
    let pos = PhysicalPosition::new(config.position.x, config.position.y);
    window.set_position(pos).unwrap();
    window.set_shadow(false).unwrap();
    window.set_always_on_top(true).expect("Failed to set window as topmost");
}