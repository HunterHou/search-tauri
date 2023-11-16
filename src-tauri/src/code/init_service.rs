use std::thread;

use super::service_setting;
use super::service_search;
// use super::db;
pub fn init_sys() {
    // db::init_db();
    let setting= service_setting::loading_file();
    thread::spawn(move||service_search::search_disk(setting.Dirs.to_vec()));
}