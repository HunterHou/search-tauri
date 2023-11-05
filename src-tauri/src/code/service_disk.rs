use std::io::Result;
use std::path::Path;

use std::time::SystemTime;
use walkdir::DirEntry;
use walkdir::WalkDir;

use super::const_param::STATIC_ACTRESS;
use super::const_param::STATIC_DATA;
use super::const_param::STATIC_DIR_SIZE;
use super::const_param::STATIC_LIST;
use super::const_param::STATIC_SETTING;
use super::const_param::STATIC_TAG_SIZE;
use super::const_param::STATIC_TYPE_SIZE;
use super::model_actress::ActressModel;
use super::model_actress::TypeAnalyzer;
use super::model_file::FileModel;

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
                Some(value) => match value.to_str() {
                    Some(val) => {
                        path = format!("{}", String::from(val));
                    }
                    _ => {}
                },
                _ => {}
            }
            let mut filename = "".to_string();
            match filepath.file_stem() {
                Some(value) => match value.to_str() {
                    Some(val) => {
                        filename = format!("{}", String::from(val));
                    }
                    _ => {}
                },
                _ => {}
            }

            let mut extname = "".to_string();
            match filepath.extension() {
                Some(value) => match value.to_str() {
                    Some(val) => {
                        extname = format!("{}", String::from(val));
                    }
                    _ => {}
                },
                _ => {}
            }

            let file = FileModel::from_path(
                String::from(dir.replace("'", "''")),
                dirpath.replace("'", "''"),
                path.replace("'", "''"),
                filename.replace("'", "''"),
                extname,
                size,
                created,
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
    STATIC_DATA
        .try_lock()
        .unwrap()
        .insert(String::from(&id), file.clone());
    STATIC_LIST.try_lock().unwrap().push(file.clone());
}

pub fn cache_analyzer() {
    println!("cache_analyzer start");
    let start = SystemTime::now();
    for ele in STATIC_LIST.try_lock().unwrap().clone().into_iter() {
        if !ele.is_empty() {
            cache_static_analyzer(&ele);
        }
    }
    let end = SystemTime::now().duration_since(start);
    println!("cache_analyzer:{:?}", end.ok());
}

pub fn cache_static_analyzer(file: &FileModel) {
    let file_type = String::from(&file.FileType);
    let video = match STATIC_SETTING.try_lock() {
        Ok(val) => {
            let mut ve = Vec::new();
            for v in val.VideoTypes.iter() {
                ve.push(String::from(v))
            }
            ve
        }
        Err(_) => Vec::new(),
    };
    if video.contains(&file_type) {
        let size = &file.Size;
        let path = String::from(&file.Path);
        let base_dir = String::from(&file.BaseDir);
        let movie_type = String::from(&file.MovieType);
        let actress = String::from(&file.Actress);

        let act_map = &mut STATIC_ACTRESS.try_lock().unwrap();
        if act_map.contains_key(&actress) {
            let actre = match act_map.get_mut(&actress) {
                Some(val) => val,
                None => todo!(),
            };
            actre.add_video(*size, path);
        } else {
            let mut actre = ActressModel::new(&actress);
            actre.add_video(*size, path);
            STATIC_ACTRESS
                .try_lock()
                .unwrap()
                .insert(String::from(&actress), actre);
        }
        let type_map = &mut STATIC_TYPE_SIZE.try_lock().unwrap();
        if type_map.contains_key(&movie_type) {
            let valt = match type_map.get_mut(&movie_type) {
                Some(val) => val,
                None => todo!(),
            };
            valt.size_plus(*size);
        } else {
            let mut actre = TypeAnalyzer::new(&movie_type, false);
            actre.size_plus(*size);
            STATIC_TYPE_SIZE
                .try_lock()
                .unwrap()
                .insert(String::from(&movie_type), actre);
        }

        let dir_map = &mut STATIC_DIR_SIZE.try_lock().unwrap();
        if dir_map.contains_key(&base_dir) {
            let valt = match dir_map.get_mut(&base_dir) {
                Some(val) => val,
                None => todo!(),
            };
            valt.size_plus(*size);
        } else {
            let mut actre = TypeAnalyzer::new(&base_dir, true);
            actre.size_plus(*size);
            STATIC_DIR_SIZE
                .try_lock()
                .unwrap()
                .insert(String::from(&base_dir), actre);
        }
        if &file.Tags.len() > &0 {
            let tag_map = &mut STATIC_TAG_SIZE.try_lock().unwrap();
            for ele in &file.Tags {
                if tag_map.contains_key(ele) {
                    let valt = match tag_map.get_mut(ele) {
                        Some(val) => val,
                        None => todo!(),
                    };
                    valt.size_plus(*size);
                } else {
                    let mut actre = TypeAnalyzer::new(ele, true);
                    actre.size_plus(*size);
                    STATIC_TAG_SIZE
                        .try_lock()
                        .unwrap()
                        .insert(String::from(ele), actre);
                }
            }
        }
    }
}
