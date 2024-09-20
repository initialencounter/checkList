use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use std::{env, io};

#[derive(Serialize, Deserialize, Debug)]
struct CheckItem {
    name: String,
    state: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct PositionConfig {
    pub(crate) x: f64,
    pub(crate) y: f64,
}
#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Config {
    check_list: Vec<CheckItem>,
    pub(crate) position: PositionConfig,
}

pub fn get_config_struct() -> Config {
    let config_data = get_config();
    serde_json::from_str(config_data.as_str()).unwrap()
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
            // println!("{}",contents);
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
            "name": "报告编号",
            "state": false
        }
    ],
    "position": {
        "x": -15,
        "y": 1395
    }
}"#,
            );
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
