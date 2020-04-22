use warp::http::{Response, StatusCode};
use serde::{Deserialize, Serialize};
use serde_json::ser;

use super::common;

#[derive(Serialize, Deserialize)]
#[derive(Debug)]
struct Reading {
    id: u32,
    book: u32,
    start_date: String,
    end_date: Option<String>,
    notes: Option<String>
}

pub fn reading_by_id_response(id: u32) -> Response<String> {
    let res_builder = Response::builder();
    let maybe_reading = query_reading_by_id(id);

    if maybe_reading.is_err() {
        let error = maybe_reading.unwrap_err();
        match error {
	    rusqlite::Error::QueryReturnedNoRows => res_builder
		.status(StatusCode::NOT_FOUND)
		.body(String::from("No reading was found with that id"))
		.unwrap(),
	    _ => res_builder
		.status(StatusCode::INTERNAL_SERVER_ERROR)
		.body(error.to_string())
		.unwrap()
        }
    } else {
	let reading = maybe_reading.unwrap();
	let json_str = ser::to_string(&reading).unwrap();	
	let response = res_builder.status(StatusCode::OK).body(json_str).unwrap();
	return response;
    }
}

fn query_reading_by_id(id: u32) -> Result<Reading, rusqlite::Error> {
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

pub fn delete_reading_response(id: u32) -> Response<String> {
    let res_builder = Response::builder();
    let delete_result = delete_reading_by_id(id);

    match delete_result {
        Ok(changed_rows) => {
	    let message: String;
	    if changed_rows == 0 {
		message = String::from("No reading found with that id, no rows changed");
	    } else if changed_rows == 1 {
		message = String::from("A single reading has been deleted");
	    } else {
		message = format!("{} rows were changed!! Something may have gone wrong!", changed_rows);
	    }
	    res_builder
		.status(StatusCode::NO_CONTENT)
		.body(message)
		.unwrap()
	},
        Err(error) => res_builder
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(error.to_string())
            .unwrap(),
    }
}

fn delete_reading_by_id(id: u32) -> Result<usize, rusqlite::Error> {
    let conn = common::get_database_connection()?;
    let mut stmt = conn.prepare("DELETE FROM reading WHERE id = :id;")?;
    // execute_named returns either Ok(usize) or Err(rusqlite::Error)
    // which is exactly what I want, so it can be returned as is.
    stmt.execute_named(&[(":id", &id)])
}
