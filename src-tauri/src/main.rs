// Prevents additional console window on Windows in release, DO NOT REMOVE!!
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod code;

use code::model_actress::ActressModel;
use code::model_actress::TypeAnalyzer;
use code::model_file::FileModel;
use code::model_params::RequestFileParam;
use code::model_params::ResultData;
use code::model_params::ResultParam;
use code::model_setting::Setting;
use code::service_search;
use code::service_setting;

use code::const_param::{STATIC_DATA, STATIC_SETTING};
use code::const_param::STATIC_ACTRESS;
use code::const_param::STATIC_DIR_SIZE;
use code::const_param::STATIC_LIST;
use code::const_param::STATIC_TAG_SIZE;
use code::const_param::STATIC_TYPE_SIZE;

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
    let res = service_search::search_disk(base_dir.to_vec());
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
    let mut request: RequestFileParam = match serde_json::from_str(params) {
        Ok(v) => v,
        Err(err) => {
            println!("serde_json::from_str {:?}", err);
            RequestFileParam::new()
        }
    };
    let res = STATIC_SETTING.lock().unwrap().clone();
    request.FileType = res.VideoTypes;
    // println!("search_index request{:?}", request);
    let res: ResultData = service_search::search_index(request.clone());
    return service_search::wrapper_request(&request, &res);
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
    service_setting::refresh_setting(&request);
    ResultParam::success()
}

#[tauri::command]
fn read_settings() -> Setting {
    let setting =service_setting::loading_file();
    // println!("setting {:?}", setting);
    return setting;
    // let res = STATIC_SETTING.lock().unwrap().clone();
    // println!("STATIC_SETTING {:?}", res);
    // return res;
}

#[tauri::command]
fn actress_map() -> Vec<ActressModel> {
    let res = STATIC_ACTRESS.lock().unwrap().clone();
    println!("actress_map {:?}", res);
    return res.into_values().map(|v|v.clone()).collect::<Vec<ActressModel>>();
}

#[tauri::command]
fn type_size_map() -> Vec<TypeAnalyzer> {
    let res = STATIC_TYPE_SIZE.lock().unwrap().clone();
    println!("type_size_map {:?}", res);
    return res.into_values().collect::<Vec<TypeAnalyzer>>();
}

#[tauri::command]
fn tag_size_map() -> Vec<TypeAnalyzer> {
    let res = STATIC_TAG_SIZE.lock().unwrap().clone();
    println!("tag_size_map {:?}", res);
    return res.into_values().collect::<Vec<TypeAnalyzer>>();
}

#[tauri::command]
fn dir_size_map() -> Vec<TypeAnalyzer> {
    let res = STATIC_DIR_SIZE.lock().unwrap().clone();
    println!("dir_size_map {:?}", res);
    return res.into_values().collect::<Vec<TypeAnalyzer>>();
}

fn main() {
    service_setting::loading_file();
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            greet,
            actress_map,
            type_size_map,
            tag_size_map,
            dir_size_map,
            submit_settings,
            read_settings,
            refresh_disk,
            search_index,
            find_file_info
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
