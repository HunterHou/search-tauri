pub mod  datamodel;
pub mod  database;
pub mod  service;
pub mod  utils;

pub mod static_param {
    use lazy_static::lazy_static;
    use std::{collections::HashMap, sync::Mutex};
    use crate::datamodel::file_model::FileModel;

    lazy_static! {
        pub static ref STATIC_DATA: Mutex<HashMap<String, FileModel>> = {
            let map = HashMap::new();
            Mutex::new(map)
        };
        pub static ref STATIC_LIST:Mutex<Vec<FileModel>> ={
            let list:Vec<FileModel> =Vec::<FileModel>::new();
            Mutex::new(list)
        };
    }
}

fn main() {
    println!("Hello, world!");
}



