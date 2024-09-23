use serde::Serialize;
use tauri::{App, AppHandle, Emitter, Manager, PhysicalPosition, Wry};
use tauri::menu::{MenuBuilder, MenuItem};
use tauri::tray::{MouseButton, MouseButtonState, TrayIcon, TrayIconBuilder, TrayIconEvent};
use tauri_plugin_autostart::ManagerExt;
use tauri_plugin_dialog::{DialogExt, MessageDialogKind};

use crate::config::get_config_struct;
use crate::menu::create_menu_item;
use crate::shortcut::win32shortcut;
use crate::utils::{check_update, hide_or_show, restart};

#[derive(Serialize, Clone)]
struct Link {
    link: String,
}

pub fn handle_auto_start(app: &AppHandle<Wry>, auto_start: &MenuItem<Wry>) {
    let autostart_manager = app.autolaunch();
    let is_enabled = autostart_manager.is_enabled().unwrap();
    if is_enabled {
        let _ = autostart_manager.disable();
    } else {
        let _ = autostart_manager.enable();
    }
    auto_start
        .set_text(if is_enabled {
            "开机自启动(❌)"
        } else {
            "开机自启动(✔️)"
        })
        .expect("Failed to set tray text");
}

pub fn handle_hide_or_show(app: &AppHandle<Wry>, hide: &MenuItem<Wry>) {
    let window = app.get_webview_window("main").unwrap();
    let title = hide_or_show(window);
    hide.set_text(title).expect("Failed to set tray text");
}

pub fn handle_update(app: &AppHandle<Wry>) {
    let current_version = format!("v{}", env!("CARGO_PKG_VERSION"));
    let latest = check_update(String::from("000"));
    if latest == "000" {
        app.dialog()
            .message("检查更新失败!")
            .kind(MessageDialogKind::Error)
            .show(|_| {});
    } else if latest != current_version {
        app.dialog()
            .message(format!("发现新版本{}，是否前往", latest))
            .kind(MessageDialogKind::Info)
            .show(|_| {});
        app.emit(
            "open_link",
            Some(Link {
                link: "https://github.com/initialencounter/checkList/releases/latest".to_string(),
            }),
        )
            .unwrap();
    } else {
        app.dialog()
            .message("当前版本是最新版")
            .kind(MessageDialogKind::Info)
            .show(|_| {});
    }
}

pub fn handle_tray_icon_event(tray: &TrayIcon, event: TrayIconEvent) {
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

pub fn handle_setup(app: &App<Wry>) {
    let [help_, quit, hide,
    about, update, restart_,
    auto_start] = create_menu_item(app);
    let tray_menu = MenuBuilder::new(app)
        .items(&[&help_, &update, &restart_, &auto_start, &about, &hide, &quit]) // insert the menu items here
        .build()
        .unwrap();
    let _ = TrayIconBuilder::new()
        .icon(app.default_window_icon().unwrap().clone())
        .menu(&tray_menu)
        .on_menu_event(move |app, event| match event.id().as_ref() {
            "help" => app.emit("open_link", Some(Link { link: "https://github.com/initialencounter/checkList?tab=readme-ov-file#使用帮助".to_string() })).unwrap(),
            "quit" => app.exit(0),
            "hide" => handle_hide_or_show(&app, &hide),
            "restart" => restart(),
            "about" => app.emit("open_link", Some(Link { link: "https://github.com/initialencounter/checkList".to_string() })).unwrap(),
            "update" => handle_update(app),
            "auto_start" => handle_auto_start(&app, &auto_start),
            _ => {}
        })
        .on_tray_icon_event(handle_tray_icon_event)
        .build(app).unwrap();
    let window = app.get_webview_window("main").unwrap();
    let config = get_config_struct();
    let pos = PhysicalPosition::new(config.position.x, config.position.y);
    window.set_position(pos).unwrap();
    window.set_shadow(false).unwrap();
    window
        .set_always_on_top(true)
        .expect("Failed to set window as topmost");
    tauri::async_runtime::spawn(async move {
        win32shortcut(window);
    });
}
