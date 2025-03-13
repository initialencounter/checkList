use tauri::WebviewWindow;
use tauri::Emitter;

#[cfg(target_os = "windows")]
use windows::Win32::UI::{
    Input::KeyboardAndMouse::{MOD_CONTROL, MOD_SHIFT, RegisterHotKey, VIRTUAL_KEY},
    WindowsAndMessaging::{GetMessageW, MSG, WM_HOTKEY},
};

#[cfg(any(target_os = "linux", target_os = "macos"))]
use rdev::{grab, Event, EventType, Key};
#[cfg(any(target_os = "linux", target_os = "macos"))]
use std::sync::{Arc, Mutex};

// Windows 实现
#[cfg(target_os = "windows")]
fn listen_hotkey<F>(callback: F)
where
    F: Fn() + Send + 'static,
{
    unsafe {
        let modifiers = MOD_CONTROL | MOD_SHIFT;
        let key = VIRTUAL_KEY('C' as u16);
        if RegisterHotKey(None, 1, modifiers, key.0 as u32).is_err() {
            eprintln!("Failed to register hotkey");
            return;
        }

        let mut msg = MSG::default();
        while GetMessageW(&mut msg, None, 0, 0).as_bool() {
            if msg.message == WM_HOTKEY {
                callback();
            }
        }
    }
}

#[cfg(target_os = "windows")]
pub fn rdev_shortcut(window: WebviewWindow) {
    std::thread::spawn(move || {
        listen_hotkey(move || {
            window.emit("KeyPressed", Some(())).unwrap();
        });
    });
}

// Linux/macOS 实现
#[cfg(any(target_os = "linux", target_os = "macos"))]
pub fn rdev_shortcut(window: WebviewWindow) {
    std::thread::spawn(move || {
        let is_ctrl_press = Arc::new(Mutex::new(false));
        let is_shift_press = Arc::new(Mutex::new(false));
        let window_clone = window.clone();
        let _ = grab(move |event| {
            let mut ctrl = is_ctrl_press.lock().unwrap();
            let mut shift = is_shift_press.lock().unwrap();
            match event.event_type {
                EventType::KeyPress(Key::ControlLeft) => *ctrl = true,
                EventType::KeyPress(Key::ShiftLeft) => *shift = true,
                EventType::KeyRelease(Key::ControlLeft) => *ctrl = false,
                EventType::KeyRelease(Key::ShiftLeft) => *shift = false,
                EventType::KeyPress(Key::KeyC) => {
                    if *ctrl && *shift {
                        window_clone.emit("KeyPressed", Some(())).unwrap();
                        return None;
                    }
                }
                _ => (),
            }
            Some(event)
        }).unwrap();
    });
}