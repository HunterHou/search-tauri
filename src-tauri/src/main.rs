// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// =======================================================================================
mod utils;
mod service;
mod datamodel;

use service::search_disk as searchDisk;
// fn main() {
//     let base_dir =["d://emby";1];
//     match searchDisk::search_disk(base_dir.to_vec()) {
//         Ok(values)=> for value in values {
//             println!("main {:?}", value);
//         },
//         _ => {}
//     }
// }


// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn refresh_disk(name: &str)  {
    let base_dir = ["d://emby"; 1];
    match searchDisk::search_disk(base_dir.to_vec()) {
        Ok(values) => for value in values {
            println!("main {:?}", value);
        },
        _ => {}
    }
}


fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet,refresh_disk])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
