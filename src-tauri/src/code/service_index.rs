use super::utils_do_file_name::int_to_size_str;
use rusqlite::{Connection, Error, NO_PARAMS};
use std::{thread, time::SystemTime};

use super::super::model::params::{RequestFileParam, ResultData};

use super::db;
use super::file_service::visit_dirs;

pub fn search_disk(dir_paths: Vec<&str>) -> Result<i32, Error> {
    let mut file_count: i32 = 0;
    let start = SystemTime::now();
    // let mut file_list:Vec<FileModel>=Vec::new();
    // let mut handles = vec![];
    for dir_path in dir_paths {
        let dir = String::from(dir_path);
        let handle = thread::spawn(move || match visit_dirs(&dir) {
            Ok(value) => {
                let count = &value.len();
                file_count = file_count + (*count as i32);
                add_to_db(&value, &dir, None);
            }
            Err(err) => println!("{}", err),
        });
    }
    let end = SystemTime::now().duration_since(start);
    println!("search_disk over:{:?}", end.ok());
    Ok(file_count)
}

pub fn add_to_db(files: &Vec<FileModel>, dir_path: &str, mut connect: Option<Connection>) {
    if files.len() == 0 {
        return;
    }
    let start = SystemTime::now();
    if connect.is_none() {
        connect = Some(db::update_connection());
    }
    let conn: Connection = match connect {
        Some(value) => value,
        None => db::update_connection(),
    };

    let del_sql = format!("delete from t_file where BaseDir='{}' ", dir_path);
    let mut sql = String::from("BEGIN; ");
    let _ = conn.execute(&del_sql, NO_PARAMS);

    for file in files {
        let items = format!("insert into t_file(Id,Name,Code,MovieType,FileType,Png,Jpg,Gif,Actress,Path,DirPath,Title,MTime,Tags,Size,SizeStr,BaseDir)  values ('{}','{}','{}','{}','{}','{}','{}','{}','{}','{}','{}','{}','{}','{}',{},'{}','{}');",
                            file.Id,file.Name,file.Code,file.MovieType,file.FileType,file.Png,file.Jpg,file.Gif,file.Actress,file.Path,file.DirPath,file.Title,file.MTime,file.Tags.join(","),file.Size,file.SizeStr,file.BaseDir
        );
        sql.push_str(&items);
    }
    sql.push_str(" COMMIT;");
    let res = conn.execute_batch(&sql);
    // println!("executing sql:{}", sql);
    if res.is_err() {
        // println!("executing sql :{} \n\n\n err:{}",&sql, res.err().unwrap());
        println!("executing sql   err:{} \n\n\n", res.err().unwrap());
    }
    let _ = conn.close();
    let start = SystemTime::now().duration_since(start);
    println!(
        "executing over :{}, dir:{} ,time:{:?} \n\n",
        &files.len(),
        &dir_path,
        start.ok()
    );
}

pub fn search_index(request: RequestFileParam) -> ResultData {
    let conn = db::query_connection();
    let mut rd = ResultData::new();
    let mut condition = String::new();
    if request.FileType.len() > 0 {
        condition.push_str(&format!(
            " and t.FileType in ('{:?}') ",
            &request.FileType.join("','")
        ));
        // println!("FileType in ('{:?}')", &request.FileType.join("','"));
    }
    if request.Keyword.len() > 0 {
        condition.push_str(&format!(
            " and (t.Code like '%{}%' or t.Path like '%{}%') ",
            &request.Keyword, &request.Keyword
        ));
    }
    if request.params.MovieType.len() > 0 {
        condition.push_str(&format!(
            " and t.MovieType = '{}' ",
            &request.params.MovieType
        ));
    }
    let mut sql_query: String = String::from(
        "SELECT t.Id,
                t.Name,
                t.Code,
                t.MovieType,
                t.FileType,
                ifnull(t1.Path,'')  Png,
                ifnull(t2.Path,'') Jpg,
                t.Actress,
                t.Path,
                t.DirPath,
                t.Title,
                t.MTime,
                t.Tags,
                t.size,
                t.sizeStr,
                ifnull(t3.Path,'') Gif,
                t.BaseDir
        from t_file t
                    left join t_file t1 on t.png = t1.Path
                    left join t_file t2 on t.jpg = t2.Path
                    left join t_file t3 on t.gif = t3.Path
        where 1 = 1",
    );
    let mut sql_count: String =
        String::from("SELECT ifnull(count(Id),0),ifnull(sum(Size),0)  from t_file t where 1=1");
    sql_query.push_str(&String::from(&condition).replace("\"", ""));
    sql_count.push_str(&String::from(&condition).replace("\"", ""));

    if request.SortField.len() > 0 && request.SortType.len() > 0 {
        sql_query.push_str(&format!(
            " order by t.{} {} ",
            &request.SortField, &request.SortType
        ));
    } else {
        sql_query.push_str(" order by t.MTime desc ");
    }

    sql_query.push_str(&format!(
        " limit {},{}",
        &(request.PageSize * (request.Page - 1)),
        &request.PageSize
    ));
    // println!("sql_count:{}", &sql_count);
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
        // println!("ResultData:{:?}", rd);
        return rd;
    }
    println!("sql_query:{}", &sql_query);
    let mut stmt = conn.prepare(&sql_query).unwrap();
    let res = stmt
        .query_map(NO_PARAMS, |row| {
            let c14: String = row.get(12).unwrap();
            let c1414: Vec<&str> = c14.split(",").collect();
            let mut tags: Vec<String> = Vec::new();
            for tagi in c1414 {
                tags.push(String::from(tagi))
            }
            let sizes: i64 = row.get(13).unwrap();
            let id: String = row.get(0).unwrap();
            let name: String = row.get(1).unwrap();
            let path: String = row.get(8).unwrap();
            let dir_path: String = row.get(9).unwrap();
            let png: String = row.get(5).unwrap();
            let jpg: String = row.get(6).unwrap();
            let gif: String = row.get(15).unwrap();
            let v = FileModel {
                Id: id.replace("''", "'"),
                Name: name.replace("''", "'"),
                Code: row.get(2).unwrap(),
                MovieType: row.get(3).unwrap(),
                FileType: row.get(4).unwrap(),
                Png: png.replace("''", "'"),
                Jpg: jpg.replace("''", "'"),
                Actress: row.get(7).unwrap(),
                Path: path.replace("''", "'"),
                DirPath: dir_path.replace("''", "'"),
                Title: row.get(10).unwrap(),
                MTime: row.get(11).unwrap(),
                Tags: tags,
                Size: sizes,
                SizeStr: row.get(14).unwrap(),
                Gif: gif.replace("''", "'"),
                BaseDir: row.get(16).unwrap(),
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
    // println!("ResultData:{:?}", rd);
    return rd;
}
