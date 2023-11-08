use super::service_setting;
// use super::db;
pub fn init_sys() {
    // db::init_db();
    service_setting::loading_file();
}