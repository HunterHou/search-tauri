pub mod database;
pub mod model;
pub mod service;
pub mod utils;

pub mod static_param {
    use super::model::file_model::FileModel;
    use super::model::setting::Setting;
    use lazy_static::lazy_static;
    use std::{collections::HashMap, sync::Mutex};

    // 全局常量
    pub const STATIC_SETTING_PATH: &str = "./setting.json";
    // 全局变量
    lazy_static! {
        pub static ref STATIC_DATA: Mutex<HashMap<String, FileModel>> = {
            let map = HashMap::new();
            Mutex::new(map)
        };
        pub static ref STATIC_LIST: Mutex<Vec<FileModel>> = {
            let list: Vec<FileModel> = Vec::<FileModel>::new();
            Mutex::new(list)
        };
        pub static ref STATIC_SETTING: Mutex<Setting> = {
            let setting: Setting = Setting::new();
            Mutex::new(setting)
        };
    }
}

fn main() {
    println!("Hello, world!");
}
