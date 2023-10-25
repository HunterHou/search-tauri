use serde::{Deserialize, Serialize};

use super::file_model::FileModel;
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResultData {
    pub Data: Vec<FileModel>,
    pub Count: i64,
    pub SizeStr: String,
}
impl ResultData {
    pub fn new() -> ResultData {
        ResultData {
            Data: Vec::new(),
            Count: 0,
            SizeStr: "".to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RequestFileParam {
    #[serde(default)]
    pub Page: i64,
    #[serde(default)]
    pub PageSize: i64,
    #[serde(default)]
    pub OnlyRepeat: bool,
    #[serde(default)]
    pub TotalCnt: i64,
    #[serde(default)]
    pub TotalSize: String,
    #[serde(default)]
    pub TotalPage: i64,
    #[serde(default)]
    pub ResultCnt: i64,
    #[serde(default)]
    pub ResultSize: String,
    #[serde(default)]
    pub Keyword: String,
    #[serde(default)]
    pub SortField: String,
    #[serde(default)]
    pub SortType: String,
    #[serde(default)]
    pub FileType: Vec<String>,
    #[serde(flatten)]
    pub params: FileModel,
    #[serde(default)]
    pub Data: Vec<FileModel>,
}

impl RequestFileParam {
    pub fn new() -> RequestFileParam {
        RequestFileParam {
            Page: 1,
            PageSize: 10,
            OnlyRepeat: false,
            params: FileModel::new(),
            Data: Vec::new(),
            TotalCnt: 0,
            TotalSize: "".to_string(),
            ResultCnt: 0,
            ResultSize: "".to_string(),
            Keyword: "".to_string(),
            SortField: "MTime".to_string(),
            SortType: "desc".to_string(),
            FileType: vec![String::from("mp4")],
            TotalPage:0,
        }
    }
}

impl Default for RequestFileParam {
    fn default() -> Self {
        Self {
            Page: 1,
            PageSize: 10,
            OnlyRepeat: Default::default(),
            params: FileModel::new(),
            Data: Default::default(),
            TotalCnt: Default::default(),
            TotalSize: Default::default(),
            ResultCnt: Default::default(),
            ResultSize: Default::default(),
            Keyword: Default::default(),
            SortField: Default::default(),
            SortType: Default::default(),
            FileType: Default::default(),
            TotalPage:0,
        }
    }
}
