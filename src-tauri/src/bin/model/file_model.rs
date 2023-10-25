// 数据模型 文件
use super::super::utils::do_file_name::{
    actress_from_name, code_from_name, int_to_size_str, movie_type_from_name,
    system_time_to_string, tags_from_name, title_from_name, vm_jpg_from_name, vm_png_from_name,
};
use serde::{Deserialize, Serialize};
use std::time::SystemTime;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct FileModel {
    pub Id: String,
    pub Name: String,
    pub Code: String,
    pub MovieType: String,
    pub FileType: String,
    pub Png: String,
    pub Jpg: String,
    pub Actress: String,
    pub Path: String,
    pub DirPath: String,
    pub Title: String,
    pub SizeStr: String,
    pub Size: i64,
    pub MTime: String,
    pub Tags: Vec<String>,
}

impl FileModel {
    pub fn is_empty(&self) -> bool {
        self.Id == ""
    }

    pub fn new() -> FileModel {
        FileModel {
            Id: "".to_string(),
            Name: "".to_string(),
            Code: "".to_string(),
            MovieType: "".to_string(),
            FileType: "".to_string(),
            Png: "".to_string(),
            Jpg: "".to_string(),
            Actress: "".to_string(),
            Path: "".to_string(),
            DirPath: "".to_string(),
            Title: "".to_string(),
            SizeStr: "".to_string(),
            Size: 0,
            MTime: "".to_string(),
            Tags: Vec::new(),
        }
    }

    pub fn from_path(
        dirpath: String,
        path: String,
        name: String,
        ext: String,
        size: i64,
        created: SystemTime,
    ) -> FileModel {
        let abs_path ="".to_string() + &dirpath + "\\" + &path;
        let abs_name ="".to_string() + &dirpath + "\\" + &name;
        let code = code_from_name(&name);
        let actress = actress_from_name(&name);
        let movie_type = movie_type_from_name(&name);
        let title = title_from_name(&name);
        let png = vm_png_from_name(&abs_name);
        let jpg = vm_jpg_from_name(&abs_name);
        let tags = tags_from_name(&name);
        let path_bac = &String::from(&abs_path);
        let mtime = system_time_to_string(&created);
        return FileModel {
            Id: String::from(path_bac),
            Name: name,
            Code: code,
            MovieType: movie_type,
            FileType: ext,
            Png: png,
            Jpg: jpg,
            Actress: actress,
            Path: String::from(path_bac),
            DirPath: dirpath,
            Title: title,
            SizeStr: int_to_size_str(size),
            Size: size,
            MTime: mtime,
            Tags: tags,
        };
    }
}
