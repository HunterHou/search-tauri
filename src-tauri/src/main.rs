// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// =======================================================================================
mod bin;

use bin::model::params::ResultData;
use bin::{database::db, model, service /*, static_param */};
use model::file_model::FileModel;
use model::params::RequestFileParam;
use service::search;
// use static_param::STATIC_LIST;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
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
    return search::wrapper_request(&request,&res);
}


fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, refresh_disk, search_index])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
