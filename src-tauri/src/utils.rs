use reqwest;
use serde::Deserialize;
use tauri::{self, Window};
use webbrowser;
use std::fs::File;
use std::io::{self, Read, Write};
use std::path::Path;
use std::env;

#[derive(Deserialize)]
struct Release {
    tag_name: String,
}

#[tauri::command]
pub fn open_link(url: &str) {
    if let Err(e) = webbrowser::open(&url) {
        eprintln!("Failed to open link: {}", e);
    }
}

#[tauri::command]
pub fn restart() {
    tauri::api::process::restart(&tauri::Env::default())
}

#[tauri::command]
pub fn get_config() -> String {
    match env::current_dir() {
        Ok(path) => {
            println!("当前工作目录: {}", path.display());
        }
        Err(e) => {
            println!("获取当前工作目录时出错: {}", e);
        }
    }

    let file_path = "checkList.json";
    match read_file_to_string(file_path) {
        Ok(contents) => {
            println!("{}",contents);
            contents
        }
        Err(_e) => {
            let config = String::from(
r#"{
    "check_list": [
        {
            "name": "结论",
            "state": false
        },
        {
            "name": "品名",
            "state": false
        },
        {
            "name": "颜色",
            "state": false
        },
        {
            "name": "形状",
            "state": false
        },
        {
            "name": "尺寸",
            "state": false
        },
        {
            "name": "型号",
            "state": false
        },
        {
            "name": "商标",
            "state": false
        },
        {
            "name": "数量",
            "state": false
        },
        {
            "name": "净重",
            "state": false
        },
        {
            "name": "电池类型",
            "state": false
        },
        {
            "name": "质量",
            "state": false
        },
        {
            "name": "设备品名",
            "state": false
        },
        {
            "name": "设备型号",
            "state": false
        },
        {
            "name": "设备商标",
            "state": false
        },
        {
            "name": "数量",
            "state": false
        },
        {
            "name": "跌落",
            "state": false
        },
        {
            "name": "堆码",
            "state": false
        },
        {
            "name": "电压",
            "state": false
        },
        {
            "name": "容量",
            "state": false
        },
        {
            "name": "瓦数",
            "state": false
        },
        {
            "name": "结论",
            "state": false
        },
        {
            "name": "报告编号",
            "state": false
        }
    ],
    "position": {
        "x": -10,
        "y": 925
    }
}"#);
            write_to_file(file_path, &config);
            config
              
        }
    }

}

fn write_to_file(filename: &str, content: &str) {
    // 打开文件，如果文件不存在则创建
    let mut file = File::create(filename).unwrap();
    // 写入内容到文件
    file.write_all(content.as_bytes()).unwrap();

}

fn read_file_to_string<P: AsRef<Path>>(path: P) -> io::Result<String> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
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

pub fn set_window_topmost(window: Window) {
    window
        .set_always_on_top(true)
        .expect("Failed to set window as topmost");
}
