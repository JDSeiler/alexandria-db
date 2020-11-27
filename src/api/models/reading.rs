use serde::{Deserialize, Serialize};

use super::common;

#[derive(Serialize, Deserialize, Debug)]
pub struct Reading {
    id: Option<u32>,
    book: u32,
    start_date: String,
    end_date: Option<String>,
    notes: Option<String>,
}

pub fn delete_reading_by_id(id: u32) -> Result<usize, rusqlite::Error> {
    let conn = common::get_database_connection()?;
    let mut stmt = conn.prepare("DELETE FROM reading WHERE id = :id;")?;
    stmt.execute_named(&[(":id", &id)])
}

pub fn query_reading_by_id(id: u32) -> Result<Reading, rusqlite::Error> {
    let conn = common::get_database_connection()?;
    let mut stmt = conn.prepare("SELECT * FROM reading WHERE id = :id;")?;
    let row = stmt.query_row_named(&[(":id", &id)], |row| {
        Ok(Reading {
            id: row.get(0)?,
            book: row.get(1)?,
            start_date: row.get(2)?,
            end_date: row.get(3)?,
            notes: row.get(4)?,
        })
    })?;
    Ok(row)
}

pub fn query_readings_by_filter(
    filter_col: String, filter_query: String) -> Result<Vec<Reading>, rusqlite::Error> {

    if !common::column_name_is_valid(filter_col.as_ref()) {
        return Err(rusqlite::Error::InvalidColumnName(filter_col));
    }

    let conn = common::get_database_connection()?;
    let partial_stmt = format!("SELECT * FROM reading where {} = :filter_query;", filter_col);
    let mut stmt = conn.prepare(partial_stmt.as_ref())?;
    let params: &[(&str, &dyn rusqlite::ToSql)] = &[(":filter_query", &filter_query)];

    let mut rows = stmt.query_named(params)?;
    let mut readings: Vec<Reading> = Vec::new();
    while let Some(row) = rows.next()? {
        let reading = Reading {
            id: row.get(0)?,
            book: row.get(1)?,
            start_date: row.get(2)?,
            end_date: row.get(3)?,
            notes: row.get(4)?,
        };
        readings.push(reading);
    };
    Ok(readings)
}

pub fn write_reading_to_db(reading: Reading) -> Result<usize, rusqlite::Error> {
    let conn = common::get_database_connection()?;
    let mut stmt = conn.prepare(
        "INSERT INTO reading 
(book, start_date, end_date, notes) VALUES 
(:book, :start_date, :end_date, :notes);"
    )?;

    let params: &[(&str, &dyn rusqlite::ToSql)] = &[
        (":book", &reading.book),
        (":start_date", &reading.start_date),
        (":end_date", &reading.end_date),
        (":notes", &reading.notes),
    ];
    stmt.execute_named(params)
}

pub fn update_reading_in_db(reading: Reading) -> Result<usize, rusqlite::Error> {
    let conn = common::get_database_connection()?;
    let mut stmt = conn.prepare(
        "UPDATE reading SET 
book = :book,
start_date = :start_date,
end_date = :end_date,
notes = :notes
WHERE id = :id;"
    )?;

    println!("{:#?}", reading);
    let params: &[(&str, &dyn rusqlite::ToSql)] = &[
        (":id", &reading.id),
        (":book", &reading.book),
        (":start_date", &reading.start_date),
        (":end_date", &reading.end_date),
        (":notes", &reading.notes),
    ];
    stmt.execute_named(params)
}
