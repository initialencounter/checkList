use serde::{Deserialize, Serialize};
use serde_json::json;
use std::path::Path;
use tauri_plugin_store::StoreExt;

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

impl Config {
    pub fn default() -> Self {
        Config {
            check_list: vec![
                CheckItem {
                    name: "结论".to_string(),
                    state: false
                },
                CheckItem {
                    name: "品名".to_string(),
                    state: false
                },
                CheckItem {
                    name: "颜色".to_string(),
                    state: false
                },
                CheckItem {
                    name: "形状".to_string(),
                    state: false
                },
                CheckItem {
                    name: "尺寸".to_string(),
                    state: false
                },
                CheckItem {
                    name: "型号".to_string(),
                    state: false
                },
                CheckItem {
                    name: "商标".to_string(),
                    state: false
                },
                CheckItem {
                    name: "数量".to_string(),
                    state: false
                },
                CheckItem {
                    name: "净重".to_string(),
                    state: false
                },
                CheckItem {
                    name: "电池类型".to_string(),
                    state: false
                },
                CheckItem {
                    name: "质量".to_string(),
                    state: false
                },
                CheckItem {
                    name: "设备品名".to_string(),
                    state: false
                },
                CheckItem {
                    name: "设备型号".to_string(),
                    state: false
                },
                CheckItem {
                    name: "设备商标".to_string(),
                    state: false
                },
                CheckItem {
                    name: "数量".to_string(),
                    state: false
                },
                CheckItem {
                    name: "跌落".to_string(),
                    state: false
                },
                CheckItem {
                    name: "堆码".to_string(),
                    state: false
                },
                CheckItem {
                    name: "电压".to_string(),
                    state: false
                },
                CheckItem {
                    name: "容量".to_string(),
                    state: false
                },
                CheckItem {
                    name: "瓦数".to_string(),
                    state: false
                },
                CheckItem {
                    name: "报告编号".to_string(),
                    state: false
                }
            ],
            position: PositionConfig { x: 50.0, y: 1395.0 },
        }
    }
}

#[tauri::command]
pub fn get_config(app: tauri::AppHandle) -> String {
    let check_list = get_config_struct(&app);
    serde_json::to_string(&check_list).unwrap()
}

pub fn get_config_struct(app: &tauri::AppHandle) -> Config {
    let check_list = get_check_item_config(app);
    let position = get_position_config(app);
    Config { check_list, position }
}

pub fn get_position_config(app: &tauri::AppHandle) -> PositionConfig {
    let store = app.store(&Path::new("checkList.json")).unwrap();
    let position = match store.get("position") {
        Some(data) => match serde_json::from_value(data) {
            Ok(position) => position,
            Err(_) => Config::default().position,
        },
        None => {
            save_position_config(app, Config::default().position).unwrap();
            Config::default().position
        }
    };
    position
}

fn get_check_item_config(app: &tauri::AppHandle) -> Vec<CheckItem> {
    let store = app.store(&Path::new("checkList.json")).unwrap();
    let check_list = match store.get("check_list") {
        Some(data) => match serde_json::from_value(data) {
            Ok(check_list) => check_list,
            Err(_) => Config::default().check_list,
        },
        None => {
            save_check_item_config(app, Config::default().check_list).unwrap();
            Config::default().check_list
        }
    };
    check_list
}

// 保存配置
fn save_check_item_config(app: &tauri::AppHandle, config: Vec<CheckItem>) -> Result<(), String> {
    let store = app.store(&Path::new("checkList.json")).unwrap();
    store.set("check_list", json!(config));
    store.save().map_err(|e| e.to_string())?;
    Ok(())
}

// 保存配置
fn save_position_config(app: &tauri::AppHandle, config: PositionConfig) -> Result<(), String> {
    let store = app.store(&Path::new("checkList.json")).unwrap();
    store.set("position", json!(config));
    store.save().map_err(|e| e.to_string())?;
    Ok(())
}