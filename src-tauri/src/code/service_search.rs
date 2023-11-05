use core::fmt::Error;
use std::{thread, time::SystemTime};

use crate::code::{
    const_param::STATIC_LIST,
    service_disk::{cache_analyzer, visit_dirs},
};

use super::{
    const_param::STATIC_DATA,
    model_file::FileModel,
    model_params::{RequestFileParam, ResultData},
    utils_do_file_name::int_to_size_str,
};

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
    request.TotalPage = if request.PageSize > 0 {
        res.Count / request.PageSize
    } else {
        1
    };
    // 返回处理后的请求参数
    return request;
}
pub fn search_disk(dir_paths: Vec<&str>) -> Result<i32, Error> {
    let start = SystemTime::now();
    STATIC_LIST.try_lock().unwrap().clear();
    STATIC_DATA.try_lock().unwrap().clear();
    let mut handles = vec![];
    for dir_path in dir_paths {
        let dir = String::from(dir_path);
        let handle = thread::spawn(move || match visit_dirs(&dir) {
            Ok(value) => {
                // thread::spawn(move || {
                //     for ele in value {
                //         cache_static_analyzer(&ele);
                //     }
                // });
            }
            Err(err) => println!("{}", err),
        });
        handles.push(handle);
    }
    let end = SystemTime::now().duration_since(start);
    for handle in handles {
        handle.join().unwrap();
    }
    thread::spawn(move || {
        cache_analyzer()
    });
    println!("search_disk over:{:?}", end.ok());
    Ok(0)
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
    let mut rd: ResultData = ResultData::new();
    let mut total_size: i64 = 0;
    let mut total_count: i64 = 0;
    let mut result_list: Vec<FileModel> = Vec::new();
    // println!("STATIC_LIST:{:?}", request);
    // 通过锁定STATIC_LIST获取对静态列表的写入权限
    STATIC_LIST.try_lock().unwrap().iter().for_each(|item| {
        if !request.FileType.contains(&item.FileType) {
            return;
        }
        if request.MovieType.len() > 0 && !request.MovieType.contains(&item.MovieType) {
            return;
        }
        // 如果请求中的关键词长度大于0
        if request.Keyword.len() > 0 {
            // 如果列表项的名字包含关键词
            if item.Name.contains(&request.Keyword) {
                // 将列表项复制到结果列表中
                total_size = total_size + item.Size;
                total_count = total_count + 1;
                result_list.push(item.clone());
            }
        } else {
            // 将列表项复制到结果列表中
            total_size = total_size + item.Size;
            total_count = total_count + 1;
            result_list.push(item.clone());
        }
    });
    if request.SortField == "MTime" {
        if request.SortType == "asc" {
            result_list.sort_by(|c1, c2| c1.MTime.cmp(&c2.MTime));
        } else {
            result_list.sort_by(|c1, c2| c2.MTime.cmp(&c1.MTime));
        }
    }
    if request.SortField == "Size" {
        if request.SortType == "asc" {
            result_list.sort_by(|c1, c2| c1.Size.cmp(&c2.Size));
        } else {
            result_list.sort_by(|c1, c2| c2.Size.cmp(&c1.Size));
        }
    }
    if request.SortField == "Code" {
        if request.SortType == "asc" {
            result_list.sort_by(|c1, c2| c1.Code.cmp(&c2.MTime));
        } else {
            result_list.sort_by(|c1, c2| c2.Code.cmp(&c1.MTime));
        }
    }

    // 将结果列表赋值给返回值中的data字段
    let map = STATIC_DATA.try_lock().unwrap();
    let start_index = (request.Page - 1) * request.PageSize;
    let end_index = (start_index + request.PageSize) as usize;
    for idx in (start_index as usize)..end_index {
        let mut item: FileModel = match result_list.get(idx) {
            Some(val) => val.clone(),
            None => FileModel::new(),
        };
        if item.is_empty() {
            continue;
        }
        if !map.contains_key(&item.Jpg) {
            item.Jpg = String::new();
        }
        if !map.contains_key(&item.Gif) {
            item.Gif = String::new();
        }
        if !map.contains_key(&item.Png) {
            item.Png = String::new();
        }
        rd.Data.push(item);
    }
    rd.Count = total_count;
    rd.SizeStr = int_to_size_str(total_size);

    // 打印结果数据以供调试
    // println!("ResultData:{:?}", rd);

    // 返回结果数据
    return rd;
}
