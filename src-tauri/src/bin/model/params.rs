use serde::{Deserialize, Serialize};

use super::file_model::FileModel;

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
    pub Page: i64,
    pub PageSize: i64,
    pub OnlyRepeat: bool,
    pub params: FileModel,
    pub Data: Vec<FileModel>,
    pub TotalCnt: i64,
    pub TotalSize: String,
    pub ResultCnt: i64,
    pub ResultSize: String,
    pub KeyWord: String,
    pub OrderField: String,
    pub OrderType: String,
    pub FileType: Vec<String>,
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
            KeyWord: "".to_string(),
            OrderField: "MTime".to_string(),
            OrderType: "desc".to_string(),
            FileType: vec![String::from("mp4")],
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
            KeyWord: Default::default(),
            OrderField: "MTime".to_string(),
            OrderType: "desc".to_string(),
            FileType: vec![String::from("mp4")],
        }
    }
}
