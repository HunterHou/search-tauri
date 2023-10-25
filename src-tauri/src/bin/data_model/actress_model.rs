use serde::{Deserialize, Serialize};

#[derive(Debug,Deserialize,Serialize)]
pub struct ActressModel {
    Name: String,
    Size: i64,
    Cnt: i64,
    SizeStr: String,
    Images: Vec<String>,
}