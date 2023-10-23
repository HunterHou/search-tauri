// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// =======================================================================================
mod datamodel;
mod service;
mod utils;

use crate::datamodel::file_model::FileModel;
use service::search_disk as searchDisk;

use lazy_static::lazy_static;
use std::{collections::HashMap, sync::Mutex};

lazy_static! {
    static ref STATIC_DATA: Mutex<HashMap<String, FileModel>> = {
        let map = HashMap::new();
        Mutex::new(map)
    };
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn refresh_disk(name: &str) -> String {
    println!("refresh_disk {:?}", name);
    let base_dir = ["d://emby"; 1];
    let mut filelist: Vec<FileModel> = Vec::new();
    match searchDisk::search_disk(base_dir.to_vec()) {
        Ok(values) => {
            for value in values {
                println!("main {:?}", &value);
                let val = value.clone();
                STATIC_DATA
                .lock()
                .unwrap()
                .insert(String::from(&value.Id), val);
                filelist.push(value);
               
            }
        }
        // Err(err) => Err(err)
        _ => {}
    }
    serde_json::to_string(&filelist).unwrap()
}
#[tauri::command]
fn search_index(name: &str) -> String {
    println!("search_index {:?}", name);
    let mut filelist: Vec<&FileModel> = Vec::new();
    let map = STATIC_DATA.lock().unwrap();
    for value in map.values() {
        filelist.push(value);
    }
    serde_json::to_string(&filelist).unwrap()
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, refresh_disk,search_index])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
