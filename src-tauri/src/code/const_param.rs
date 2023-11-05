use lazy_static::lazy_static;
use std::{collections::HashMap, sync::Mutex};

use crate::code::{
    model_actress::{ActressModel, TypeAnalyzer},
    model_file::FileModel,
    model_setting::Setting,
};

// 全局常量
pub const STATIC_SETTING_PATH: &str = "./setting.json";
// 全局变量
lazy_static! {
    pub static ref STATIC_DATA: Mutex<HashMap<String, FileModel>> = {
        let map = HashMap::new();
        Mutex::new(map)
    };
}

lazy_static! {
    pub static ref STATIC_LIST: Mutex<Vec<FileModel>> = {
        let list: Vec<FileModel> = Vec::<FileModel>::new();
        Mutex::new(list)
    };
}

lazy_static! {
    pub static ref STATIC_ACTRESS: Mutex<HashMap<String, ActressModel>> = {
        let list = HashMap::new();
        Mutex::new(list)
    };
}

lazy_static! {
    pub static ref STATIC_TYPE_SIZE: Mutex<HashMap<String, TypeAnalyzer>> = {
        let list = HashMap::new();
        Mutex::new(list)
    };
}

lazy_static! {
    pub static ref STATIC_TAG_SIZE: Mutex<HashMap<String, TypeAnalyzer>> = {
        let list = HashMap::new();
        Mutex::new(list)
    };
}
lazy_static! {
    pub static ref STATIC_DIR_SIZE: Mutex<HashMap<String, TypeAnalyzer>> = {
        let list = HashMap::new();
        Mutex::new(list)
    };
    pub static ref STATIC_SETTING: Mutex<Setting> = {
        let setting: Setting = Setting::new();
        Mutex::new(setting)
    };
}
