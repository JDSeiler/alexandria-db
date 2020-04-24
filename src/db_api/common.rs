use rusqlite::{Connection};

/** 

Returns a `Result` that is either a usable connection to the database
or a `rusqlite::Error`. The path to the database is currently
hardcoded into the function as a relative path. Starting alexandria-db
from a directory besides the project root results in this relative path
being incorrect and this function returning an error.

**/
pub fn get_database_connection() -> Result<Connection, rusqlite::Error> {
    let test_db_path = "./src/db_storage/dummy.db";
    let maybe_conn = Connection::open(&test_db_path);
    maybe_conn
}
