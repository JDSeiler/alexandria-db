use serde::{Deserialize, Serialize};

use super::common;

#[derive(Serialize, Deserialize)]
#[derive(Debug)]
pub struct Reading {
    id: u32,
    book: u32,
    start_date: String,
    end_date: Option<String>,
    notes: Option<String>
}

pub fn delete_reading_by_id(id: u32) -> Result<usize, rusqlite::Error> {
    let conn = common::get_database_connection()?;
    let mut stmt = conn.prepare("DELETE FROM reading WHERE id = :id;")?;
    // execute_named returns either Ok(usize) or Err(rusqlite::Error)
    // which is exactly what I want, so it can be returned as is.
    stmt.execute_named(&[(":id", &id)])
}

pub fn query_reading_by_id(id: u32) -> Result<Reading, rusqlite::Error> {
    let conn = common::get_database_connection()?;
    let mut stmt = conn.prepare("SELECT * FROM reading WHERE id = :id;")?;
    let row = stmt.query_row_named(&[(":id", &id)], |row| {
	Ok(Reading{
	    id: row.get(0)?,
	    book: row.get(1)?,
	    start_date: row.get(2)?,
	    end_date: row.get(3)?,
	    notes: row.get(4)?
	})
    })?;
    Ok(row)
}

