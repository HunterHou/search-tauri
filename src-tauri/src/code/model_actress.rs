use serde::{Deserialize, Serialize};

#[derive(Debug,Deserialize,Serialize)]
pub struct ActressModel {
    Name: String,
    Size: i64,
    Cnt: i64,
    SizeStr: String,
    Images: Vec<String>,
}

impl ActressModel {
    fn add_image(&mut self, image: String) {
        self.Images.push(image);
    }
}

pub struct TypeAnalyzer {
    pub Name: String,
    pub IsDir:bool,
    pub Cnt:i64,
    pub Size:i64,
    pub SizeStr: String,
}

impl TypeAnalyzer {

    fn new(name:String, is_dir:bool)->TypeAnalyzer {
        TypeAnalyzer {
            Name: name,
            IsDir: is_dir,
            Cnt: 0,
            Size: 0,
            SizeStr: "".to_string(),
        }
    }

    fn size_plus(&mut self, size:i64) {
        self.Size += size;
        self.SizeStr = format!("{:.2}", self.Size as f64 / 1024.0 / 1024.0);
    }
}