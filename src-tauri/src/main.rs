// Prevents additional console window on Windows in release, DO NOT REMOVE!!
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod bin;

use bin::static_param::{STATIC_DATA, STATIC_SETTING};
use bin::{database::db, model, service /*, static_param */};
use model::file_model::FileModel;
use model::params::RequestFileParam;
use service::search;
use service::setting_service;
use crate::bin::model::params::ResultParam;
use crate::bin::model::setting::Setting;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn refresh_disk(name: &str) -> String {
    println!("refresh_disk {:?}", name);
    let base_dir = vec![
        "d:\\emby",
        "e:\\emby",
        "f:\\emby",
        "g:\\emby",
        "h:\\emby",
        "i:\\emby",
        "J:\\emby",
        "k:\\emby",
        "/Users/harmay/Documents",
    ];
    let _filelist: Vec<FileModel> = Vec::new();
    db::init_db();
    let res = search::search_disk(base_dir.to_vec());
    if res.is_err() {
        let msg = &res.err().unwrap().to_string();
        println!("refresh_disk error:{}", &msg);
        return serde_json::to_string(msg).unwrap();
    } else {
        let count = res.ok().unwrap();
        return serde_json::to_string(&count).unwrap();
    }
}
#[tauri::command]
fn search_index(params: &str) -> RequestFileParam {
    // println!("search_index params{:?}", params);
    let request: RequestFileParam = match serde_json::from_str(params) {
        Ok(v) => v,
        Err(err) => {
            println!("serde_json::from_str {:?}", err);
            RequestFileParam::new()
        }
    };
    // println!("search_index request{:?}", request);
    let res: model::params::ResultData = search::search_index(request.clone());
    return search::wrapper_request(&request, &res);
}
#[tauri::command]
fn find_file_info(id: &str) -> FileModel {
    let map = STATIC_DATA.lock().unwrap();
    println!("find_file_info id:{} ", id);
    if map.contains_key(id) {
        let res: FileModel = map.get(id).unwrap().clone();
        println!("find_file_info id:{} {:?}", id, res);
        return res.clone();
    }
    return FileModel::new();
}

#[tauri::command]
fn submit_settings(params: &str) -> ResultParam {
    let request: Setting = match serde_json::from_str(params) {
        Ok(v) => v,
        Err(err) => {
            println!("serde_json::from_str {:?}", err);
            Setting::new()
        }
    };
    setting_service::refresh_setting(&request);
    ResultParam::success()
}


#[tauri::command]
fn read_settings() -> Setting {
    let res = STATIC_SETTING.lock().unwrap().clone();
    return res;
}


fn main() {
    bin::service::setting_service::loading_file();
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            greet,
            submit_settings,
            read_settings,
            refresh_disk,
            search_index,
            find_file_info
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
