// use rusqlite::{Connection,NO_PARAMS};
// // "./search.db"

// pub fn query_connection() -> Connection {
//     println!("query_connection...");
//     Connection::open("./search.db").unwrap()
// }

// pub fn update_connection() -> Connection {
//     println!("update_connection...");
//     Connection::open("./search.db").unwrap()
// }

// pub fn config_connection() {
//     println!("update_connection...");
//     let conn=query_connection();
//     let _ = conn.execute("PRAGMA journal_mode = OFF", NO_PARAMS);
//     let _ = conn.execute("PRAGMA Synchronous = 0", NO_PARAMS);
//     let _ = conn.execute("PRAGMA cache_size = 10000000000", NO_PARAMS);
//     let _ = conn.execute("PRAGMA timeout = 6000", NO_PARAMS);
//     let _ = conn.execute("PRAGMA temp_store = MEMORY", NO_PARAMS);
// }

// pub fn init_db() {
//     config_connection();
//     let conn = update_connection();
//     println!("Initializing table t_file");
//     match conn.execute_batch(" begin ;
//     drop table if exists t_file;
//     create  table t_file(
//         Id text primary key,
//         Name text,
//         Code text,
//         MovieType text,
//         FileType text,
//         Png text,
//         Jpg text,
//         Gif text,
//         Actress text,
//         Path text,
//         DirPath text,
//         Title text,
//         MTime text,
//         Tags text,
//         size integer,
//         sizeStr text,
//         BaseDir text
//     ); commit;") {
//         Ok(val) => println!("create success:{:?}", val),
//         Err(err) => println!("create Fail:{}", err),
//     };
// }