use reqwest;
use serde::Deserialize;
use tauri::{self, WebviewWindow};

#[derive(Deserialize)]
struct Release {
    tag_name: String,
}

#[tauri::command]
pub fn restart() {
    tauri::process::restart(&tauri::Env::default())
}

#[tauri::command]
pub fn log_info(info: String) {
    println!("{}", info);
}

pub fn check_update(flag: String) -> String {
    let url = "https://api.github.com/repos/initialencounter/checkList/releases/latest";
    let client = reqwest::blocking::Client::new();

    let resp = match client
        .get(url)
        .header(reqwest::header::USER_AGENT, "rust-app")
        .send()
    {
        Ok(response) => response,
        Err(_) => return flag,
    };

    if !resp.status().is_success() {
        return flag;
    }

    let release = match resp.json::<Release>() {
        Ok(release) => release,
        Err(_) => return flag,
    };
    release.tag_name
}

pub fn hide_or_show(window: WebviewWindow) {
    if window.is_visible().unwrap() {
        window.hide().unwrap();
    } else {
        window
            .set_always_on_top(true)
            .expect("Failed to set window as topmost");
        window.show().unwrap();
    }
}