use rusqlite::{Connection};

pub fn db_connection() -> rusqlite::Connection {
    println!("db_connection...");
    Connection::open("./search.db").unwrap()
}
pub fn init_db() {
    let conn = db_connection();
    println!("Initializing table t_file");
    let _ = conn.execute("create or replace table t_file(
        id integer primary key autoincrement,
        name text,
        path text,
        size integer,
        sizeStr text
    )", []);
}