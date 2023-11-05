use crate::code::utils_do_file_name::int_to_size_str;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ActressModel {
    Name: String,
    Size: i64,
    Cnt: i64,
    SizeStr: String,
    Images: Vec<String>,
    Videos: Vec<String>,
}

impl ActressModel {
    pub fn new(name: &str) -> ActressModel {
        ActressModel {
            Name: String::from(name),
            Size: 0,
            Cnt: 0,
            SizeStr: "".to_string(),
            Images: Vec::new(),
            Videos: Vec::new(),
        }
    }
    pub fn add_video(&mut self, size: i64, video: String) {
        self.Cnt += 1;
        self.Size += size;
        self.SizeStr = int_to_size_str(self.Size);
        self.Videos.push(video);
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
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
        self.SizeStr = format!("{:.2}", self.Size as f64 / 1024.0 / 1024.0);
    }
}
