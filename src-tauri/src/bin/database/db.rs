use rusqlite::{Connection,NO_PARAMS};

pub fn db_connection() -> Connection {
    println!("db_connection...");
    Connection::open("./search.db").unwrap()
}

pub fn init_db() {
    let conn = db_connection();
    println!("Initializing table t_file");
    match conn.execute("drop table if exists t_file", NO_PARAMS) {
        Ok(val) => println!("drop success :{}", val),
        Err(err) => println!("drop Fail:{}", err),
    };
    match conn.execute("create  table t_file(
        Id integer primary key autoincrement,
        Name text,
        Code text,
        MovieType text,
        FileType text,
        Png text,
        Jpg text,
        Actress text,
        Path text,
        DirPath text,
        Title text,
        MTime text,
        Tags text,
        size integer,
        sizeStr text
    )", NO_PARAMS) {
        Ok(val) => println!("create Fail:{}", val),
        Err(err) => println!("create Fail:{}", err),
    };
}