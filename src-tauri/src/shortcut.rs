use std::sync::{Arc, Mutex};

use rdev::{Event, EventType, grab, Key};
use tauri::{Emitter, WebviewWindow};
// use std::ptr::null_mut;
// use std::sync::{ Once, OnceLock };
// use windows::Win32::Foundation::{HINSTANCE, LPARAM, LRESULT, WPARAM};
// use windows::Win32::UI::WindowsAndMessaging::{CallNextHookEx, HHOOK, KBDLLHOOKSTRUCT, MSG,
//                                               SetWindowsHookExW, UnhookWindowsHookEx,
//                                               WH_KEYBOARD_LL, WM_KEYDOWN, WM_KEYUP};
// use windows::Win32::UI::WindowsAndMessaging;

// static mut MODIFIER_STATE: Option<ModifierState> = None;
// static INIT: Once = Once::new();
// 
// struct ModifierState {
//     ctrl: bool,
//     shift: bool,
// }
// 
// #[derive(Debug)]
// struct HookHandler {
//     window: WebviewWindow,
// }
// 
// impl HookHandler {
//     fn handle_keypress(&self) {
//         // 使用外部参数
//         self.window.emit("KeyPressed", Some(())).unwrap();
//     }
// }
// 
// // 静态全局变量保存 HookHandler 实例
// static HOOK_HANDLER: OnceLock<HookHandler> = OnceLock::new();
// 
// unsafe extern "system" fn keyboard_proc(ncode: i32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
//     if ncode == 0 {
//         let kb_struct = *(lparam.0 as *const KBDLLHOOKSTRUCT);
//         let vk_code = kb_struct.vkCode;
// 
//         INIT.call_once(|| {
//             MODIFIER_STATE = Some(ModifierState { ctrl: false, shift: false });
//         });
// 
//         let modifier_state: &mut ModifierState = MODIFIER_STATE.as_mut().unwrap();
// 
//         // 处理按键按下事件
//         if wparam.0 == WM_KEYDOWN as usize {
//             match vk_code {
//                 162 => modifier_state.ctrl = true,
//                 160 => modifier_state.shift = true,
//                 67 => {
//                     if modifier_state.ctrl && modifier_state.shift {
//                         if let Some(handler) = HOOK_HANDLER.get() {
//                             handler.handle_keypress();
//                             return LRESULT(1);
//                         }
//                     }
//                 }
//                 _ => {
//                     ()
//                 }
//             }
//         }
//         // 处理按键释放事件
//         else if wparam.0 == WM_KEYUP as usize {
//             match vk_code {
//                 162 => modifier_state.ctrl = false,
//                 160 => modifier_state.shift = false,
//                 _ => {
//                     ()
//                 }
//             }
//         }
//     }
// 
//     CallNextHookEx(None, ncode, wparam, lparam)
// }
// 
// 
// pub fn win32shortcut(window: WebviewWindow) {
//     // 初始化全局的 HookHandler
// 
//     HOOK_HANDLER.set(HookHandler { window }).unwrap();
// 
//     unsafe {
//         // 设置钩子并处理可能的错误
//         let hook_result = SetWindowsHookExW(WH_KEYBOARD_LL, Some(keyboard_proc), HINSTANCE(null_mut()), 0);
// 
//         // 确保钩子成功设置
//         let hook_id: HHOOK = match hook_result {
//             Ok(hook) => hook,
//             Err(err) => {
//                 eprintln!("Failed to set hook: {:?}", err);
//                 return;
//             }
//         };
// 
//         let mut msg: MSG = MSG::default();
//         while WindowsAndMessaging::GetMessageW(&mut msg, None, 0, 0).into() {
//             let _ = WindowsAndMessaging::TranslateMessage(&msg);
//             WindowsAndMessaging::DispatchMessageW(&msg);
//         }
// 
//         // 卸载钩子
//         UnhookWindowsHookEx(hook_id).unwrap();
//     }
// }

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
                _ => {
                    ()
                }
            }
            Some(event)
        };

        let _a = grab(_callback); // 开始捕获事件
    });
}