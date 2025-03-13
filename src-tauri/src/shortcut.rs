use tauri::{Emitter, WebviewWindow};

#[cfg(any(target_os = "linux", target_os = "macos"))]
use rdev::{grab, Event, EventType, Key};

#[cfg(any(target_os = "linux", target_os = "macos"))]
use std::sync::{Arc, Mutex};

#[cfg(target_os = "windows")]
use windows::Win32::UI::{
    Input::KeyboardAndMouse::{RegisterHotKey, HOT_KEY_MODIFIERS},
    WindowsAndMessaging::{GetMessageW, MSG, WM_HOTKEY},
};

#[cfg(target_os = "windows")]
async fn listen_hotkey<F, Fut>(mut callback: F)
where
    F: FnMut() -> Fut,
    Fut: futures::Future<Output = bool>,
{
    unsafe {
        let _ = RegisterHotKey(None, 1, HOT_KEY_MODIFIERS(0x0006), 0x43);
        let mut msg = MSG::default();
        while GetMessageW(&mut msg, None, 0, 0).as_bool() {
            if msg.message == WM_HOTKEY && !callback().await {
                break;
            }
        }
    }
}
#[cfg(target_os = "windows")]
pub fn rdev_shortcut(window: WebviewWindow) {
    std::thread::spawn(move || {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            listen_hotkey(move || {
                window.emit("KeyPressed", Some(())).unwrap();
                async move { true }
            })
            .await;
        });
    });
}

#[cfg(any(target_os = "linux", target_os = "macos"))]
pub fn rdev_shortcut(window: WebviewWindow) {
    tauri::async_runtime::spawn(async move {
        let is_ctrl_press = Arc::new(Mutex::new(false));
        let is_shift_press = Arc::new(Mutex::new(false));
        let _callback = move |event: Event| -> Option<Event> {
            let mut ctrl = is_ctrl_press.lock().unwrap();
            let mut shift = is_shift_press.lock().unwrap();
            match event.event_type {
                EventType::KeyPress(Key::ControlLeft) => *ctrl = true,
                EventType::KeyPress(Key::ShiftLeft) => *shift = true,
                EventType::KeyRelease(Key::ControlLeft) => *ctrl = false,
                EventType::KeyRelease(Key::ShiftLeft) => *shift = false,
                EventType::KeyPress(Key::KeyC) => {
                    if *ctrl && *shift {
                        window.emit("KeyPressed", Some(())).unwrap();
                        return None;
                    }
                }
                _ => (),
            }
            Some(event)
        };

        let _a = grab(_callback); // 开始捕获事件
    });
}
