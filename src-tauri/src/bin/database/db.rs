use rusqlite::{Connection};

pub fn db_connection() -> rusqlite::Connection {
    println!("db_connection...");
    Connection::open("./search.db").unwrap()
}

pub fn init_db() {
    let conn = db_connection();
    println!("Initializing table t_file");
    match conn.execute("drop table if exists t_file", []) {
        Ok(val) => println!("drop success :{}", val),
        Err(err) => println!("drop Fail:{}", err),
    };
    match conn.execute("create  table t_file(
        id integer primary key autoincrement,
        code text,
        name text,
        path text,
        size integer,
        sizeStr text
    )", []) {
        Ok(val) => println!("create Fail:{}", val),
        Err(err) => println!("create Fail:{}", err),
    };
}