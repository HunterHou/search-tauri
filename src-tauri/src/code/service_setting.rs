use std::fs::File;
use std::io::prelude::*;
use std::ops::Deref;
use std::path::Path;
use std::sync::Mutex;

use super::const_param::{STATIC_SETTING, STATIC_SETTING_PATH};
use super::model_setting::Setting;

pub fn refresh_setting(setting: &Setting) {
    // 将请求的对象复制到静态设置的对象中
    let new_mutx = Mutex::new(setting.clone());
    STATIC_SETTING.deref().clone_from(&&new_mutx);
    let setting_json = serde_json::to_string(setting).unwrap();
    // 以只写模式打开文件，返回 `io::Result<File>`
    let binding = STATIC_SETTING_PATH;
    let path = Path::new(&binding);
    let mut file = match File::create(path) {
        Err(why) => panic!("couldn't create {:?}: {:?}", &STATIC_SETTING_PATH, why),
        Ok(file) => file,
    };
    // 将 `LOREM_IPSUM` 字符串写进 `file`，返回 `io::Result<()>`
    match file.write_all(setting_json.as_bytes()) {
        Err(why) => {
            panic!("couldn't write to {:?}: {:?}", &STATIC_SETTING_PATH, why)
        }
        Ok(_) => println!("successfully wrote to {}", setting_json),
    }
}

/**
 * 加载配置文件
 */
pub fn loading_file() {
    let binding = STATIC_SETTING_PATH;
    let path = Path::new(&binding);
    // 以只读方式打开路径，返回 `io::Result<File>`
    let mut setting_file: File = match File::open(&path) {
        // 如果打开失败，则打印错误信息并终止程序
        Err(why) => panic!("无法打开文件 {}: {:?}", &path.display(), why),
        Ok(file) => file,
    };
    let mut setting_json = String::new();
    // 读取文件内容到字符串，如果读取失败则打印错误信息并终止程序
    match setting_file.read_to_string(&mut setting_json) {
        Err(why) => panic!("无法读取文件 {}: {:?}", &path.display(), why),
        Ok(_) => print!("{} 文件内容如下:\n{}", &path.display(), setting_json),
    }
    // 将字符串转换为 Setting 对象，如果转换失败则打印错误信息并使用默认的空对象
    let setting: Setting = match serde_json::from_str(&setting_json) {
        Ok(v) => v,
        Err(err) => {
            println!("serde_json::from_str 错误信息: {:?}", err);
            Setting::new()
        }
    };
    // 将请求的对象复制到静态设置的对象中
    let new_mutx = Mutex::new(setting.clone());
    STATIC_SETTING.deref().clone_from(&&new_mutx);
}
