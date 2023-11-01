use super::super::utils::do_file_name::int_to_size_str;
use rusqlite::{ Connection, Error, NO_PARAMS };
use std::{ thread, time::SystemTime };

use super::super::model::params::{ RequestFileParam, ResultData };

use super::super::{ database::db, model::file_model::FileModel };
use super::disk_service::visit_dirs;
use static_param::STATIC_LIST;

/*
 * 封装请求参数
 */
pub fn wrapper_request(req: &RequestFileParam, res: &ResultData) -> RequestFileParam {
    // 克隆请求参数
    let mut request = req.clone();
    // 合并响应数据到请求的数据字段
    request.Data.extend(res.Data.to_vec());
    // 设置总记录数
    request.TotalCnt = res.Count;
    // 设置总大小
    request.TotalSize = String::from(&res.SizeStr);
    // 设置请求的记录数
    request.ResultCnt = res.Count;
    // 设置请求的大小
    request.ResultSize = String::from(&res.SizeStr);
    // 计算总页数（若每页大小非零，则用总记录数除以每页大小；否则默认为1）
    request.TotalPage = if request.PageSize > 0 { res.Count / request.PageSize } else { 1 };
    // 返回处理后的请求参数
    return request;
}
pub fn search_disk(dir_paths: Vec<&str>) -> Result<i32, Error> {
    let mut file_count: i32 = 0;
    let start = SystemTime::now();
    // let mut file_list:Vec<FileModel>=Vec::new();
    let mut handles = vec![];
    for dir_path in dir_paths {
        let dir = String::from(dir_path);
        let handle = thread::spawn(move || {
            match visit_dirs(&dir) {
                Ok(value) => {
                    let count = &value.len();
                    file_count = file_count + (*count as i32);
                }
                Err(err) => println!("{}", err),
            }
        });
        handles.push(handle);
    }
    let end = SystemTime::now().duration_since(start);
    for handle in handles {
        handle.join().unwrap();
    }
    println!("search_disk over:{:?}", end.ok());
    Ok(file_count)
}
/**
 * 索引搜索
 * @param request 请求参数
 * @return
 * 
 * 
 */
pub fn search_index(request: RequestFileParam) -> ResultData {
    // 创建一个空的结果列表
    let mut result_list: Vec<FileModel> = Vec::new();
    // 通过锁定STATIC_LIST获取对静态列表的写入权限
    STATIC_LIST.lock()
        .unwrap()
        .iter()
        .for_each(|item| {
            // 如果请求中的关键词长度大于0
            if request.Keyword.len() > 0 {
                // 如果列表项的名字包含关键词
                if item.Name.contains(&request.Keyword) {
                    // 将列表项复制到结果列表中
                    result_list.push(item.clone());
                }
            } else {
                // 将列表项复制到结果列表中
                result_list.push(item.clone());
            }
        });

    // 将结果列表赋值给返回值中的data字段
    rd.Data = result_list;

    // 打印结果数据以供调试
    // println!("ResultData:{:?}", rd);

    // 返回结果数据
    return rd;
}
