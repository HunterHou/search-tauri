use std::collections::HashMap;
use std::io::Result;
use std::path::Path;

use std::time::SystemTime;
use walkdir::DirEntry;
use walkdir::WalkDir;

use super::const_param::STATIC_ACTRESS;
use super::const_param::STATIC_ACTRESS_LIST;
use super::const_param::STATIC_DATA;
use super::const_param::STATIC_DIR_SIZE;
use super::const_param::STATIC_LIST;
use super::const_param::STATIC_SETTING;
use super::const_param::STATIC_TAG_SIZE;
use super::const_param::STATIC_TYPE_SIZE;
use super::model_actress::ActressModel;
use super::model_actress::TypeAnalyzer;
use super::model_file::FileModel;
use super::model_params::ResultParam;
/**
 * 这个函数用于遍历指定目录下的文件，并将文件信息封装成FileModel对象的集合返回。
 * 首先判断目录是否存在，若不存在则返回空的文件列表。
 * 然后使用WalkDir遍历目录下的每个项，判断是否是文件，
 * 如果是文件则获取文件的大小和创建时间，并将文件的各种信息提取出来。
 * 最后将提取的信息封装成FileModel对象并加入到文件列表中。
 *
 */
pub fn visit_dirs(dir: &str) -> Result<Vec<FileModel>> {
    let mut filelist: Vec<FileModel> = Vec::new();
    if !Path::new(dir).exists() {
        println!("dir not exists:{}", dir);
        return Ok(filelist);
    }
    let walker = WalkDir::new(dir).into_iter();
    for entry_item in walker {
        let entry: DirEntry = match entry_item {
            Ok(v) => v,
            Err(error) => panic!("{}", error),
        };
        if entry.path().is_file() {
            let mut size: i64 = 0;
            let mut created = SystemTime::now();
            match entry.metadata() {
                Ok(meta) => {
                    size = meta.len() as i64;
                    match meta.created() {
                        Ok(value) => {
                            created = value;
                        }
                        _ => {}
                    };
                }
                _ => {}
            }
            let filepath = Path::new(entry.path());
            // println!("{:?}", filepath.parent());
            // println!("{:?}", filepath.file_stem());
            // println!("{:?}", filepath.extension());
            // println!("{:?}", filepath.file_name());
            let mut dirpath = "".to_string();
            match filepath.parent() {
                Some(value) => {
                    dirpath = format!("{}", value.display());
                }
                _ => {}
            }

            let mut path = "".to_string();
            match filepath.file_name() {
                Some(value) =>
                    match value.to_str() {
                        Some(val) => {
                            path = format!("{}", String::from(val));
                        }
                        _ => {}
                    }
                _ => {}
            }
            let mut filename = "".to_string();
            match filepath.file_stem() {
                Some(value) =>
                    match value.to_str() {
                        Some(val) => {
                            filename = format!("{}", String::from(val));
                        }
                        _ => {}
                    }
                _ => {}
            }

            let mut extname = "".to_string();
            match filepath.extension() {
                Some(value) =>
                    match value.to_str() {
                        Some(val) => {
                            extname = format!("{}", String::from(val));
                        }
                        _ => {}
                    }
                _ => {}
            }

            let file = FileModel::from_path(
                String::from(dir.replace("'", "''")),
                dirpath.replace("'", "''"),
                path.replace("'", "''"),
                filename.replace("'", "''"),
                extname,
                size,
                created
            );
            if file.is_empty() {
                continue;
            }
            cache_static_file(&file);
            filelist.push(file);
        }
    }
    Ok(filelist)
}

fn cache_static_file(file: &FileModel) {
    let id = String::from(&file.Id);
    STATIC_DATA.lock().unwrap().insert(String::from(&id), file.clone());
    STATIC_LIST.lock().unwrap().push(file.clone());
}
/**
 * 这段 Rust 代码实现了一个缓存分析函数 cache_analyzer。
 * 首先，它创建了一个空的哈希映射 value，然后尝试获取静态数据并将其复制到 value 中。
 * 接下来，它检查 value 的长度，如果大于 0，则进行进一步的分析操作。
 * 它从静态设置中获取视频类型，并使用哈希映射分别将女演员、电影类型、目录和标签与对应的缓存数据进行关联，
 * 并更新相应的统计信息。最后，它清空静态女演员列表，并将女演员映射添加到列表中。
 * 该函数结束时，它打印出分析所需的时间。
 */
pub fn cache_analyzer() {
    let mut value: HashMap<String, FileModel> = HashMap::new();
    match STATIC_DATA.lock() {
        Ok(val) => {
            value = val.clone();
        }
        Err(_) => cache_analyzer(),
    }
    if value.len() > 0 {
        let cl = value.into_values().into_iter();
        println!("cache_analyzer start");
        let start = SystemTime::now();

        let video = match STATIC_SETTING.lock() {
            Ok(val) => {
                let mut ve = Vec::new();
                for v in val.VideoTypes.iter() {
                    ve.push(String::from(v));
                }
                ve
            }
            Err(_) => Vec::new(),
        };
        let act_map = &mut STATIC_ACTRESS.lock().unwrap();
        let type_map = &mut STATIC_TYPE_SIZE.lock().unwrap();
        let dir_map = &mut STATIC_DIR_SIZE.lock().unwrap();
        let tag_map = &mut STATIC_TAG_SIZE.lock().unwrap();
        for ele in cl.into_iter() {
            if !ele.is_empty() {
                // println!("cache_analyzer {}", &ele.Id);
                let file_type = String::from(&ele.FileType);
                if video.contains(&file_type) {
                    let size = &ele.Size;
                    let image = String::from(&ele.Jpg);
                    let path = String::from(&ele.Path);
                    let base_dir = String::from(&ele.BaseDir);
                    let movie_type = String::from(&ele.MovieType);
                    let actress = String::from(&ele.Actress);

                    if act_map.contains_key(&actress) {
                        let actre = match act_map.get_mut(&actress) {
                            Some(val) => val,
                            None => todo!(),
                        };
                        actre.add_video(*size, path, image);
                    } else {
                        let mut actre = ActressModel::new(&actress);
                        actre.add_video(*size, path, image);
                        act_map.insert(String::from(&actress), actre);
                    }

                    if type_map.contains_key(&movie_type) {
                        let valt = match type_map.get_mut(&movie_type) {
                            Some(val) => val,
                            None => todo!(),
                        };
                        valt.size_plus(*size);
                    } else {
                        let mut actre = TypeAnalyzer::new(&movie_type, false);
                        actre.size_plus(*size);
                        type_map.insert(String::from(&movie_type), actre);
                    }

                    if dir_map.contains_key(&base_dir) {
                        let valt = match dir_map.get_mut(&base_dir) {
                            Some(val) => val,
                            None => todo!(),
                        };
                        valt.size_plus(*size);
                    } else {
                        let mut actre = TypeAnalyzer::new(&base_dir, true);
                        actre.size_plus(*size);
                        dir_map.insert(String::from(&base_dir), actre);
                    }
                    if &ele.Tags.len() > &0 {
                        for ele in &ele.Tags {
                            if tag_map.contains_key(ele) {
                                let valt = match tag_map.get_mut(ele) {
                                    Some(val) => val,
                                    None => todo!(),
                                };
                                valt.size_plus(*size);
                            } else {
                                let mut actre = TypeAnalyzer::new(ele, true);
                                actre.size_plus(*size);
                                tag_map.insert(String::from(ele), actre);
                            }
                        }
                    }
                }
            }
        }
        STATIC_ACTRESS_LIST.lock().unwrap().clear();
        let actress = act_map.clone().into_values().collect::<Vec<ActressModel>>();
        STATIC_ACTRESS_LIST.lock().unwrap().extend(actress);
        let end = SystemTime::now().duration_since(start);
        println!("cache_analyzer over:{:?}", end.ok());
    }
}

pub fn rename_file(id: &str, new_name: &str) -> ResultParam {
    
    let map = STATIC_DATA.lock().unwrap();
    let value: FileModel =match map.get(id) {
        Some(val) => val.clone(),
        None => FileModel::new(),
    };
    if value.is_empty() {
        return ResultParam::fail("文件不存在");
    }
    return ResultParam::success();
}
