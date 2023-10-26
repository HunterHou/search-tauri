use rusqlite::{Connection,NO_PARAMS};

pub fn query_connection() -> Connection {
    println!("query_connection...");
    Connection::open("./search.db").unwrap()
}

pub fn update_connection() -> Connection {
    println!("update_connection...");
    Connection::open("./search.db").unwrap()
}

pub fn init_db() {
    let conn = update_connection();
    println!("Initializing table t_file");
    match conn.execute_batch(" begin ;
    drop table if exists t_file;
    create  table t_file(
        Id text primary key,
        Name text,
        Code text,
        MovieType text,
        FileType text,
        Png text,
        Jpg text,
        Gif text,
        Actress text,
        Path text,
        DirPath text,
        Title text,
        MTime text,
        Tags text,
        size integer,
        sizeStr text,
        BaseDir text
    ); commit;") {
        Ok(val) => println!("create success:{:?}", val),
        Err(err) => println!("create Fail:{}", err),
    };
}