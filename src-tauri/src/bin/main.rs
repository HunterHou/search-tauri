// mod database;
mod service;
// use crate::database::db::init_db;
use service::setting_service;


fn main() {
    // init_db();
    setting_service::loading_file();
}