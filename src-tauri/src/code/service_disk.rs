use std::collections::HashMap;
use std::fs;
use std::io::Result;
use std::path::Path;
use std::path::MAIN_SEPARATOR_STR;

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
use super::service_http;
use crate::code::utils_do_file_name::tagstr_from_name;
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
        .lock()
        .unwrap()
        .insert(String::from(&id), file.clone());
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
pub fn file_exists(path: &str) -> bool {
    let file = Path::new(path);
    return file.exists();
}

//  这个函数接受一个文件路径，判断该文件是否存在，
// 若存在则删除该文件，并返回删除是否成功，若不存在则返回失败。
pub fn delete_file(path: &str) -> bool {
    let file = Path::new(path);
    if file.exists() {
        let res = fs::remove_file(file);
        if res.is_err() {
            return false;
        }
        println!("delete file:{}", path);
        return true;
    }
    return false;
}
// 一个路径参数，判断该路径是否存在。
// 如果存在，则递归删除该路径下的所有文件和子目录，并返回true；否则返回false。
pub fn delete_dir(path: &str) -> bool {
    let file = Path::new(path);
    if file.exists() {
        fs::remove_dir_all(path).unwrap();
        return true;
    }
    return false;
}
// 这个函数接收两个参数，分别是源文件路径和目标文件路径。
// 函数首先根据源文件路径创建一个Path对象，然后检查该文件是否存在，如果存在则返回false。
// 接着使用fs::rename方法将源文件移动到目标文件路径，如果移动失败则返回false。
// 最后，如果移动成功，则返回true。
pub fn rename_file(src: &str, desc: &str) -> bool {
    let file = Path::new(src);
    if !file.exists() {
        return false;
    }
    let res = fs::rename(src, desc);
    if res.is_err() {
        println!("rename_file err:{}", res.err().unwrap());
        return false;
    }
    return true;
}
// 重命名文件模型
pub fn rename_file_model(file: &FileModel, is_move: &bool) -> ResultParam {
    // println!("{},rename_file_model:{:?}", is_move, file);
    if file.is_empty() {
        return ResultParam::error("参数为空");
    } else {
        let static_map = STATIC_DATA.lock().unwrap();
        let original_file = match static_map.get(file.Id.as_str()) {
            Some(val) => val,
            None => todo!(),
        };
        if original_file.is_empty() {
            return ResultParam::error("原始文件为空");
        }
       
        if file.Jpg.len() > 0 && file.Jpg.starts_with("http") {
            println!("fileJpg:{:?}", file.Jpg);
            let _ = service_http::download(
                &file.Jpg,
                &original_file.DirPath,
                &original_file.Name,
                "jpg",true,
            );
        }

        let metadata = Path::new(file.Name.as_str());
        let new_name = metadata.file_stem().unwrap().to_str().unwrap();
        let mut original_name = original_file.DirPath.clone();
        let original_tags = "《".to_string() + &tagstr_from_name(&original_file.Name) + "》";
        let original_type = "{{".to_string() + &original_file.MovieType + "}}";
        if *is_move {
            if &file.Actress.len() > &0 {
                original_name.push_str(MAIN_SEPARATOR_STR);
                original_name.push_str(&file.Actress);
                if !file_exists(&original_name) {
                    let _ = fs::create_dir_all(&original_name);
                }
            }
            if &file.Title.len() > &0 {
                original_name.push_str(MAIN_SEPARATOR_STR);
                let mut dir_name = String::new();
                if &file.Actress.len() > &0 {
                    dir_name.push_str(&file.Actress);
                }
                if &file.Code.len() > &0 {
                    dir_name.push_str(&file.Code);
                }
                dir_name.push_str(&file.Title);
                if dir_name.contains(&original_tags) {
                    dir_name = dir_name.replace(&original_tags, "");
                }
                if dir_name.contains(&original_type) {
                    dir_name = dir_name.replace(&original_type, "");
                }
                original_name.push_str(&dir_name);
                if !file_exists(&original_name) {
                    let _ = fs::create_dir_all(&original_name);
                }
            }
        }
        original_name.push_str(MAIN_SEPARATOR_STR);
        original_name.push_str(new_name);
        if file.Tags.len() > 0 {
            let new_tags = "《".to_string() + &file.Tags.iter().as_slice().join(",") + "》";
            println!("new_tags:{}", &new_tags);
            if original_file.Tags.len() > 0 {
                println!("original_tags:{}", &original_tags);
                original_name = original_name.replace(&original_tags, &new_tags);
            } else {
                original_name.push_str(&new_tags);
            }
        }
        println!("original_name:{}", &original_name);
        let new_type = "{{".to_string() + &file.MovieType + "}}";

        if !original_name.contains(&original_type) {
            original_name.push_str(&new_type);
        } else {
            original_name = original_name.replace(&original_type, &new_type);
        }
        original_name = original_name.trim().to_string();
        let mut dest_name = String::from(&original_name);
        let mut dest_jpg = String::from(&original_name);
        let mut dest_png = String::from(&original_name);
        let mut dest_gif = String::from(&original_name);

        dest_name.push_str(&(".".to_string() + &original_file.FileType));
        dest_jpg.push_str(".jpg");
        dest_png.push_str(".png");
        dest_gif.push_str(".gif");
        // println!("dest_name:{}", dest_name);
        if rename_file(original_file.Path.as_str(), &dest_name) {
            rename_file(original_file.Png.as_str(), &dest_png);
            rename_file(original_file.Jpg.as_str(), &dest_jpg);
            rename_file(original_file.Gif.as_str(), &dest_gif);
        } else {
            return ResultParam::error("删除失败");
        }
    }
    return ResultParam::ok();
}

// 删除文件模型
pub fn delete_file_model(file_id: &str) -> ResultParam {
    let static_map = STATIC_DATA.lock().unwrap().clone();
    let empty = FileModel::new();
    let file = match static_map.get(file_id) {
        Some(val) => val,
        None => &empty,
    };
    if file.is_empty() {
        return ResultParam::error("文件模型为空");
    } else {
        if delete_file(file.Path.as_str()) {
            delete_file(file.Png.as_str());
            delete_file(file.Jpg.as_str());
            delete_file(file.Gif.as_str());
        } else {
            return ResultParam::error("删除失败");
        }
    }
    return ResultParam::ok();
}

fn find_file_by_id(id: &str) -> FileModel {
    let static_map = STATIC_DATA.lock().unwrap().clone();
    let binding = FileModel::new();
    let original_file = match static_map.get(id) {
        Some(val) => val,
        None => &binding,
    };
    return original_file.clone();
}

#[allow(dead_code)]
pub fn add_tag(id: &str, tag: &str) -> ResultParam {
    let new_tag = String::from(tag);
    let mut original_file = find_file_by_id(id);
    if original_file.is_empty() {
        return ResultParam::error("找不到源文件");
    }
    if tag.len() == 0 {
        return ResultParam::error("请选择标签");
    }
    let mut tags = original_file.Tags.clone();
    if !tags.contains(&new_tag) {
        tags.insert(0, new_tag);
    }
    println!("add_tag-new_tags:{:?}", &tags);
    original_file.Tags = tags;

    rename_file_model(&original_file, &false);
    return ResultParam::ok();
}
#[allow(dead_code)]
pub fn remove_tag(id: &str, tag: &str) -> ResultParam {
    let new_tag = String::from(tag);
    let mut original_file = find_file_by_id(id);
    if original_file.is_empty() {
        return ResultParam::error("找不到源文件");
    }
    if tag.len() == 0 {
        return ResultParam::error("请选择标签");
    }
    let mut tags = original_file.Tags.clone();
    if tags.contains(&new_tag) {
        tags.retain(|e| e != &new_tag)
    }
    println!("remove_tag-new_tags:{:?}", &tags);
    original_file.Tags = tags;
    rename_file_model(&original_file, &false);
    return ResultParam::ok();
}
#[allow(dead_code)]
pub fn set_movie_type(id: &str, move_type: &str) -> ResultParam {
    println!("set_movie_type:{}", move_type);
    let mut original_file = find_file_by_id(id);
    if original_file.is_empty() {
        return ResultParam::error("找不到源文件");
    }
    if move_type.len() == 0 {
        return ResultParam::error("请选择类型");
    }
    original_file.MovieType = String::from(move_type);
    rename_file_model(&original_file, &false);
    return ResultParam::ok();
}
