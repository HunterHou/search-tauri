// mod database;
mod service;
// use crate::database::db::init_db;
use service::setting_service;


pub fn main() {
    init();
    
}

pub fn init() {
    setting_service::loading_file();
}