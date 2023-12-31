use serde::{ Deserialize, Serialize };

use super::{ model_file::FileModel, model_actress::ActressModel };

#[derive(Serialize, Deserialize, Debug, Clone)]
#[allow(non_snake_case)]
pub struct ResultParam {
    pub Success: bool,
    // 结果码，表示请求执行的结果状态
    pub Code: i64,
    // 消息，描述结果码的含义
    pub Message: String,
    // 数据，包含具体的结果数据
    pub Data: ResultData,
}

// ResultParam 类的实现
impl ResultParam {
    // 创建一个成功的 ResultParam 对象
    pub fn ok() -> ResultParam {
        ResultParam {
            Code: 200,
            Success: true,
            Message: "执行成功".to_string(),
            Data: ResultData::new(),
        }
    }
    #[allow(dead_code)]
    pub fn is_ok(&self) -> bool {
        self.Success
    }

    // 创建一个失败的 ResultParam 对象
    pub fn error(msg: &str) -> ResultParam {
        ResultParam {
            Code: 400,
            Success: false,
            Message: String::from(msg),
            Data: ResultData::new(),
        }
    }
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResultData {
    // 数据字段，保存文件模型的集合
    pub Data: Vec<FileModel>,
    // 总数
    pub Count: i64,
    // 大小字符串
    pub SizeStr: String,
}
impl ResultData {
    // 构造函数
    pub fn new() -> ResultData {
        // 创建 ResultData 实例，并初始化字段
        ResultData {
            // 文件模型的集合，初始为空
            Data: Vec::new(),
            // 总数，初始为0
            Count: 0,
            // 大小字符串，初始为空字符串
            SizeStr: "".to_string(),
        }
    }
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RequestFileParam {
    #[serde(default)]
    // 页码
    pub Page: i64,
    #[serde(default)]
    // 每页数量
    pub PageSize: i64,
    #[serde(default)]
    // 是否只包含重复文件
    pub OnlyRepeat: bool,
    #[serde(default)]
    // 总数
    pub TotalCnt: i64,
    #[serde(default)]
    // 总大小
    pub TotalSize: String,
    #[serde(default)]
    // 总页数
    pub TotalPage: i64,
    #[serde(default)]
    // 结果数量
    pub ResultCnt: i64,
    #[serde(default)]
    // 结果大小
    pub ResultSize: String,
    #[serde(default)]
    // 关键字
    pub Keyword: String,
    #[serde(default)]
    // 排序字段
    pub SortField: String,
    #[serde(default)]
    // 排序方式
    pub SortType: String,
    #[serde(default)]
    // 电影类型
    pub MovieType: String,
    #[serde(default)]
    // 文件类型
    pub FileType: Vec<String>,
    #[serde(flatten)]
    // 文件模型参数
    pub params: FileModel,
    #[serde(default)]
    // 文件模型数据
    pub Data: Vec<FileModel>,
}

impl RequestFileParam {
    // 创建一个新的 RequestFileParam 实例
    pub fn new() -> RequestFileParam {
        RequestFileParam {
            Page: 1,
            PageSize: 10,
            OnlyRepeat: false,
            params: FileModel::new(),
            Data: Vec::new(),
            TotalCnt: 0,
            TotalSize: "".to_string(),
            MovieType: "".to_string(),
            ResultCnt: 0,
            ResultSize: "".to_string(),
            Keyword: "".to_string(),
            SortField: "MTime".to_string(),
            SortType: "desc".to_string(),
            FileType: vec![String::from("mp4")],
            TotalPage: 0,
        }
    }
}

impl Default for RequestFileParam {
    // 创建默认的 RequestFileParam 实例
    fn default() -> Self {
        Self {
            Page: 1,
            PageSize: 10,
            OnlyRepeat: Default::default(),
            params: FileModel::new(),
            Data: Default::default(),
            MovieType: Default::default(),
            TotalCnt: Default::default(),
            TotalSize: Default::default(),
            ResultCnt: Default::default(),
            ResultSize: Default::default(),
            Keyword: Default::default(),
            SortField: Default::default(),
            SortType: Default::default(),
            FileType: Default::default(),
            TotalPage: 0,
        }
    }
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RequestActressParam {
    // 页码
    #[serde(default)]
    pub Page: i64,
    #[serde(default)]
    // 每页数量
    pub PageSize: i64,
    #[serde(default)]
    // 总数
    pub TotalCnt: i64,
    #[serde(default)]
    // 总大小
    pub TotalSize: String,
    #[serde(default)]
    // 总页数
    pub TotalPage: i64,
    #[serde(default)]
    // 结果数量
    pub ResultCnt: i64,
    #[serde(default)]
    // 结果大小
    pub ResultSize: String,
    #[serde(default)]
    // 关键字
    pub Keyword: String,
    #[serde(default)]
    // 排序字段
    pub SortField: String,
    #[serde(default)]
    // 排序方式
    pub SortType: String,

    #[serde(flatten)]
    // 文件模型参数
    pub params: ActressModel,
    #[serde(default)]
    // 文件模型数据
    pub Data: Vec<ActressModel>,
}

impl RequestActressParam {
    // 创建一个新的 RequestActressParam 实例
    pub fn new() -> RequestActressParam {
        RequestActressParam {
            Page: 1,
            PageSize: 10,
            params: ActressModel::new(""),
            Data: Vec::new(),
            TotalCnt: 0,
            TotalSize: "".to_string(),
            ResultCnt: 0,
            ResultSize: "".to_string(),
            Keyword: "".to_string(),
            SortField: "MTime".to_string(),
            SortType: "desc".to_string(),
            TotalPage: 0,
        }
    }

    pub fn start_index(&self) -> usize {
        let start_index = (self.Page - 1) * self.PageSize;
        return start_index as usize;
    }

    pub fn end_index(&self) -> usize {
        let end_index = ((self.Page - 1) * self.PageSize + self.PageSize) as usize;
        return end_index;
    }
}
