// Prevents additional console window on Windows in release, DO NOT REMOVE!!
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod code;

use code::const_param::QUERY_DB;
use code::init_service;
use code::model_actress::ActressModel;
use code::model_actress::TypeAnalyzer;
use code::model_file::FileModel;
use code::model_params::RequestFileParam;
use code::model_params::ResultData;
use code::model_params::ResultParam;
use code::model_setting::Setting;
use code::service_disk;
use code::service_search;
use code::service_setting;

use code::const_param::STATIC_ACTRESS_LIST;
use code::const_param::STATIC_DIR_SIZE;
use code::const_param::STATIC_TAG_SIZE;
use code::const_param::STATIC_TYPE_SIZE;
use code::const_param::{STATIC_DATA, STATIC_SETTING};

use crate::code::model_params::RequestActressParam;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn refresh_disk(name: &str) -> ResultParam {
    println!("refresh_disk {:?}", name);
    let base_dir = STATIC_SETTING.lock().unwrap().Dirs.to_vec();
    let _filelist: Vec<FileModel> = Vec::new();
    let res = service_search::search_disk(base_dir);
    if res.is_err() {
        let msg = &res.err().unwrap().to_string();
        println!("refresh_disk error:{}", &msg);
        return ResultParam::error(msg);
    } else {
        let count = res.ok().unwrap();
        println!("refresh_disk ok:{}", &count);
        return ResultParam::ok();
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
    let res = service_setting::loading_file();
    if res.VideoTypes.len() > 0 {
        request.FileType.extend(res.VideoTypes.clone());
    }
    if res.ImageTypes.len() > 0 {
        request.FileType.extend(res.ImageTypes.clone());
    }
    if res.DocsTypes.len() > 0 {
        request.FileType.extend(res.DocsTypes.clone());
    }
    println!("search_index request{:?}", res.ImageTypes);
    println!("search_index request{:?}", res.DocsTypes);
    if *QUERY_DB {
        return request;
    } else {
        let res: ResultData = service_search::search_index(request.clone());
        return service_search::wrapper_request(&request, &res);
    }
}

#[tauri::command]
fn find_file_info(id: &str) -> FileModel {
    let map = STATIC_DATA.lock().unwrap();
    // println!("find_file_info id:{} ", id);
    if map.contains_key(id) {
        let res: FileModel = map.get(id).unwrap().clone();
        // println!("find_file_info id:{} {:?}", id, res);
        return res.clone();
    }
    return FileModel::new();
}

#[tauri::command]
fn actress_map(params: &str) -> RequestActressParam {
    let mut request: RequestActressParam = match serde_json::from_str(params) {
        Ok(v) => v,
        Err(err) => {
            println!("serde_json::from_str {:?}", err);
            RequestActressParam::new()
        }
    };
    let mut actress_lib: Vec<ActressModel> = match STATIC_ACTRESS_LIST.try_lock() {
        Ok(val) => val.to_vec(),
        Err(_) => Vec::new(),
    };
    actress_lib.sort_by(|v1, v2| v2.Cnt.cmp(&v1.Cnt));
    let total = actress_lib.len() as i64;
    request.TotalCnt = total;
    request.TotalPage = total / request.PageSize + 1;
    let mut start = request.start_index();
    if start as i64 > request.TotalCnt {
        start = request.TotalCnt as usize;
    }
    let mut end = request.end_index();
    if end as i64 > request.TotalCnt {
        end = request.TotalCnt as usize;
    }
    // println!("actress_map {:?}-{:?}", start, end);
    let page_data = actress_lib[start..end].to_vec();
    request.Data = page_data.clone();
    // println!("actress_map {:?}", request);
    return request;
}

#[tauri::command]
fn type_size_map() -> Vec<TypeAnalyzer> {
    let type_map = STATIC_TYPE_SIZE.try_lock().unwrap().clone();
    // println!("type_size_map {:?}", res);
    let mut res = type_map.into_values().collect::<Vec<TypeAnalyzer>>();
    res.sort_by(|v1, v2| v2.Cnt.cmp(&v1.Cnt));
    return res;
}

#[tauri::command]
fn tag_size_map() -> Vec<TypeAnalyzer> {
    let map = STATIC_TAG_SIZE.try_lock().unwrap().clone();
    let mut res = map.into_values().collect::<Vec<TypeAnalyzer>>();
    res.sort_by(|v1, v2| v2.Cnt.cmp(&v1.Cnt));
    return res;
}

#[tauri::command]
fn dir_size_map() -> Vec<TypeAnalyzer> {
    let map = STATIC_DIR_SIZE.try_lock().unwrap().clone();
    let mut res = map.into_values().collect::<Vec<TypeAnalyzer>>();
    res.sort_by(|v1, v2| v2.Cnt.cmp(&v1.Cnt));
    return res;
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
    ResultParam::ok()
}

#[tauri::command]
fn read_settings() -> Setting {
    let setting = service_setting::loading_file();
    return setting;
}

#[tauri::command]
fn files_by_dir(path: &str) -> Vec<FileModel> {
    let mut files: Vec<FileModel> = match service_disk::visit_dirs(path) {
        Ok(val) => val,
        Err(_) => Vec::new(),
    };
    let image = vec![
        String::from("jpg"),
        String::from("png"),
        String::from("gif"),
    ];
    files = files
        .into_iter()
        .filter(|e| image.contains(&&e.FileType))
        .collect::<Vec<FileModel>>();
    return files;
}
#[tauri::command]
fn delete_dir(path: &str) {
    service_disk::delete_dir(path);
}

#[tauri::command]
fn delete_model(path: &str) -> ResultParam {
    return service_disk::delete_file_model(path);
}

#[tauri::command]
fn delete_file(path: &str) -> ResultParam {
    return service_disk::delete_file_model(path);
}

#[tauri::command]
fn rename_model(params: &str, is_move: bool) -> ResultParam {
    // println!("rename_model:{:?}", params);
    let file: FileModel = match serde_json::from_str(params) {
        Ok(v) => v,
        Err(err) => {
            println!("serde_json::from_str {:?}", err);
            FileModel::new()
        }
    };
    return service_disk::rename_file_model(&file, &is_move);
}

#[tauri::command]
fn add_tag(id: &str, tag: &str) -> ResultParam {
    return service_disk::add_tag(id, tag);
}
#[tauri::command]
fn remove_tag(id: &str, tag: &str) -> ResultParam {
    return service_disk::remove_tag(id, tag);
}

#[tauri::command]
fn set_movie_type(id: &str, tag: &str) -> ResultParam {
    return service_disk::remove_tag(id, tag);
}

fn main() {
    init_service::init_sys();
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            greet,
            set_movie_type,
            add_tag,
            remove_tag,
            delete_file,
            delete_dir,
            delete_model,
            rename_model,
            files_by_dir,
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
