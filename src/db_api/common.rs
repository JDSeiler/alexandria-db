use rusqlite::{Connection};

pub fn get_database_connection() -> Result<Connection, rusqlite::Error> {
    let test_db_path = "./src/db_storage/dummy.db";
    let maybe_conn = Connection::open(&test_db_path);
    maybe_conn
}
