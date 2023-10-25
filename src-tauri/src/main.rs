// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// =======================================================================================
mod bin;
use bin::{data_model, database::db, service, static_param};
use data_model::file_model::FileModel;
use service::search_disk as searchDisk;
use static_param::STATIC_LIST;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn refresh_disk(name: &str) -> String {
    println!("refresh_disk {:?}", name);
    let base_dir = ["d://emby"; 1];
    let _filelist: Vec<FileModel> = Vec::new();
    db::init_db();
    let res = searchDisk::search_disk(base_dir.to_vec());
    if res.is_err() {
        let msg =&res.err().unwrap().to_string();
        println!("refresh_disk error:{}", &msg);
        return serde_json::to_string(msg).unwrap();
    } else {
        let count = res.ok().unwrap();
        return serde_json::to_string(&count).unwrap();
    }
}
#[tauri::command]
fn search_index(name: &str) -> String {
    println!("search_index {:?}", name);
    let mut filelist: Vec<&FileModel> = Vec::new();
    let map = STATIC_LIST.lock().unwrap();
    for value in map.as_slice() {
        filelist.push(value);
    }
    serde_json::to_string(&filelist).unwrap()
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, refresh_disk, search_index])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
