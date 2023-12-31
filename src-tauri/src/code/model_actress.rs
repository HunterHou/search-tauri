use crate::code::utils_do_file_name::int_to_size_str;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
#[allow(non_snake_case)]
pub struct ActressModel {
    #[serde(default)]
    pub Name: String,
    #[serde(default)]
    pub Size: i64,
    #[serde(default)]
    pub Cnt: i64,
    #[serde(default)]
    pub SizeStr: String,
    #[serde(default)]
    pub Images: Vec<String>,
    #[serde(default)]
    pub Videos: Vec<String>,
    #[serde(default)]
    pub Url: String,
}

impl ActressModel {
    pub fn new(name: &str) -> ActressModel {
        ActressModel {
            Name: String::from(name),
            Size: 0,
            Cnt: 0,
            SizeStr: "".to_string(),
            Url: "".to_string(),
            Images: Vec::new(),
            Videos: Vec::new(),
        }
    }
    pub fn add_video(&mut self, size: i64, video: String, image: String) {
        if self.Url.len() == 0 && exists_file(&image) {
            self.Url = image.clone();
        }
        self.Cnt += 1;
        self.Size += size;
        self.SizeStr = int_to_size_str(self.Size);
        self.Videos.push(video);
        self.Images.push(image);
    }
}

fn exists_file(path: &str) -> bool {
    return std::path::Path::new(path).exists();
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[allow(non_snake_case)]
pub struct TypeAnalyzer {
    pub Name: String,
    pub IsDir: bool,
    pub Cnt: i64,
    pub Size: i64,
    pub SizeStr: String,
}

impl TypeAnalyzer {
    pub fn new(name: &str, is_dir: bool) -> TypeAnalyzer {
        TypeAnalyzer {
            Name: String::from(name),
            IsDir: is_dir,
            Cnt: 0,
            Size: 0,
            SizeStr: "".to_string(),
        }
    }

    pub fn size_plus(&mut self, size: i64) {
        self.Size += size;
        self.Cnt += 1;
        self.SizeStr = int_to_size_str(self.Size);
    }
}
