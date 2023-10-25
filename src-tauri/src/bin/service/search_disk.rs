use super::super::model::params::ResultData;

use super::super::model::params::RequestFileParam;

use super::super::database::db;
use super::super::model::file_model::FileModel;
use super::super::static_param::STATIC_DATA;
use super::super::utils::do_file_name::int_to_size_str;
use rusqlite::NO_PARAMS;
use std::io::Result;
use std::path::Path;
use std::time::SystemTime;
use walkdir::DirEntry;
use walkdir::WalkDir;

fn visit_dirs(dir: &str) -> Result<Vec<FileModel>> {
    let mut filelist: Vec<FileModel> = Vec::new();
    if !Path::new(dir).exists() {
        println!("dir not exists:{}", dir);
        return Ok(filelist);
    }
    let walker = WalkDir::new(dir).into_iter();
    for entry_item in walker {
        let entry: DirEntry = match entry_item {
            Ok(v) => v,
            Err(error) => panic!("{}", error),
        };
        if entry.path().is_file() {
            let mut size: i64 = 0;
            let mut created = SystemTime::now();
            match entry.metadata() {
                Ok(meta) => {
                    size = meta.len() as i64;
                    match meta.created() {
                        Ok(value) => created = value,
                        _ => {}
                    };
                }
                _ => {}
            }
            let filepath = Path::new(entry.path());
            // println!("{:?}", filepath.parent());
            // println!("{:?}", filepath.file_stem());
            // println!("{:?}", filepath.extension());
            // println!("{:?}", filepath.file_name());
            let mut dirpath = "".to_string();
            match filepath.parent() {
                Some(value) => dirpath = format!("{}", value.display()),
                _ => {}
            }

            let mut path = "".to_string();
            match filepath.file_name() {
                Some(value) => match value.to_str() {
                    Some(val) => path = format!("{}", String::from(val)),
                    _ => {}
                },
                _ => {}
            }
            let mut filename = "".to_string();
            match filepath.file_stem() {
                Some(value) => match value.to_str() {
                    Some(val) => filename = format!("{}", String::from(val)),
                    _ => {}
                },
                _ => {}
            }

            let mut extname = "".to_string();
            match filepath.extension() {
                Some(value) => match value.to_str() {
                    Some(val) => extname = format!("{}", String::from(val)),
                    _ => {}
                },
                _ => {}
            }

            let file = FileModel::from_path(dirpath, path, filename, extname, size, created);
            if file.is_empty() {
                continue;
            }
            let val = file.clone();
            STATIC_DATA
                .lock()
                .unwrap()
                .insert(String::from(&val.Id), val);
            // println!("{:?}", file);
            filelist.push(file);
        }
    }
    Ok(filelist)
}

pub fn search_disk(dir_paths: Vec<&str>) -> Result<i32> {
    let mut file_count: i32 = 0;
    for dir_path in dir_paths {
        match visit_dirs(dir_path) {
            Ok(value) => {
                // for val in value {
                //     filelist.push(val)
                // }
                add_to_db(&value);
                let count = &value.len();
                file_count = file_count + (*count as i32);
            }
            Err(err) => println!("{}", err),
        }
    }
    Ok(file_count)
}

pub fn add_to_db(files: &Vec<FileModel>) {
    if files.len() == 0 {
        return;
    }
    let conn = db::db_connection();
    let mut sql = String::from("BEGIN; ");
    for file in files {
        let items = format!(" insert into t_file(Id,Name,Code,MovieType,FileType,Png,Jpg,Actress,Path,DirPath,Title,MTime,Tags,Size,SizeStr) 
             values ('{}','{}','{}','{}','{}','{}','{}','{}','{}','{}','{}','{}','{}',{},'{}');",
            file.Id,file.Name,file.Code,file.MovieType,file.FileType,file.Png,file.Jpg,file.Actress,file.Path,file.DirPath,file.Title,file.MTime,file.Tags.join(","),file.Size,file.SizeStr);
        sql.push_str(&items);
    }
    sql.push_str(" COMMIT;");
    let res = conn.execute_batch(&sql);
    // println!("executing sql:{}", sql);
    if res.is_err() {
        println!("executing sql err:{}", res.err().unwrap());
    }
    let _ = conn.close();
}

pub fn search_index(request: RequestFileParam) -> ResultData {
    let conn = db::db_connection();
    let mut rd = ResultData::new();
    let mut condition = String::new();
    if request.FileType.len() > 0 {
        condition.push_str(&format!(
            " and FileType in ('{:?}') ",
            &request.FileType.join(",")
        ));
        println!("FileType in ('{:?}')", &request.FileType.join(","));
    }
    if request.KeyWord.len() > 0 {
        condition.push_str(&format!(
            " and (Code like '%{}%' or Path like '%{}%') ",
            &request.KeyWord, &request.KeyWord
        ));
    }
    if request.params.MovieType.len() > 0 {
        condition.push_str(&format!(
            " and MovieType = '{}' ",
            &request.params.MovieType
        ));
    }
    let mut sql_query:String= String::from("SELECT Id,Name,Code,MovieType,FileType,Png,Jpg,Actress,Path,DirPath,Title,SizeStr,Size,MTime,Tags from t_file where 1=1");
    let mut sql_count: String = String::from("SELECT ifnull(count(Id),0),ifnull(sum(Size)) 0 from t_file where 1=1");
    sql_query.push_str(&String::from(&condition).replace("\"", ""));
    sql_count.push_str(&String::from(&condition).replace("\"", ""));

    if request.SortField.len() > 0 && request.SortType.len() > 0 {
        sql_query.push_str(&format!(
            " order by {} {} ",
            &request.SortField, &request.SortType
        ));
    } else {
        sql_query.push_str(" order by MTime desc ");
    }

    sql_query.push_str(&format!(
        " limit {},{}",
        &(request.PageSize * (request.Page - 1)),
        &request.PageSize
    ));
    println!("sql:{}", &sql_query);
    let count_res = match conn.query_row(&sql_count, NO_PARAMS, |row| {
        let count: i64 = row.get(0).unwrap();
        let size: i64 = row.get(1).unwrap();
        Ok([count, size])
    }) {
        Ok(val) => val,
        Err(_) => [0, 0],
    };
    rd.Count = count_res[0];
    rd.SizeStr = int_to_size_str(count_res[1]);
    if rd.Count == 0 {
        return rd;
    }
    let mut stmt = conn.prepare(&sql_query).unwrap();
    let res = stmt
        .query_map(NO_PARAMS, |row| {
            let c14: String = row.get(14).unwrap();
            let c1414: Vec<&str> = c14.split(",").collect();
            let mut tags: Vec<String> = Vec::new();
            for tagi in c1414 {
                tags.push(String::from(tagi))
            }
            let sizes: i64 = row.get(12).unwrap();
            let v = FileModel {
                Id: row.get(0).unwrap(),
                Name: row.get(1).unwrap(),
                Code: row.get(2).unwrap(),
                MovieType: row.get(3).unwrap(),
                FileType: row.get(4).unwrap(),
                Png: row.get(5).unwrap(),
                Jpg: row.get(6).unwrap(),
                Actress: row.get(7).unwrap(),
                Path: row.get(8).unwrap(),
                DirPath: row.get(9).unwrap(),
                Title: row.get(10).unwrap(),
                SizeStr: row.get(11).unwrap(),
                Size: sizes,
                MTime: row.get(13).unwrap(),
                Tags: tags,
            };
            Ok(v)
        })
        .unwrap();
    let mut result_list: Vec<FileModel> = Vec::new();
    for x in res {
        if x.is_ok() {
            result_list.push(x.ok().unwrap())
        }
    }
    rd.Data = result_list;
    return rd;
}
