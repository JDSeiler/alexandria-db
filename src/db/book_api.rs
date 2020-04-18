use warp::http::{Response, StatusCode};
use serde::{Deserialize, Serialize};
use serde_json::ser;

use super::common;

#[derive(Serialize, Deserialize)]
#[derive(Debug)]
struct Book {
    title: String,
    author: String,
    pages: u32,
    genre: String,
    medium: String,
    rating: u32,
    notes: String
}

pub fn book_by_id_response(id: u32) -> Response<String> {
    let res_builder = Response::builder();
    let maybe_book = query_book_by_id(id);

    if maybe_book.is_err() {
	let error_message = maybe_book.unwrap_err().to_string();
	let response = res_builder.status(StatusCode::NOT_FOUND)
	    .body(String::from(error_message)).unwrap();
	return response;
    } else {
	let book = maybe_book.unwrap();
	let json_str = ser::to_string(&book).unwrap();	
	let response = res_builder.status(StatusCode::OK).body(json_str).unwrap();
	return response;
    }
}

fn query_book_by_id(id: u32) -> Result<Book, rusqlite::Error> {
    let maybe_conn = common::get_database_connection();
    if maybe_conn.is_err() {
	let error = maybe_conn.unwrap_err();
	return Err(error);
    } else {
	let conn = maybe_conn.unwrap();
	let mut stmt = conn.prepare("SELECT * FROM book WHERE id = :id;").unwrap();

	let row = stmt.query_row_named(&[(":id", &id)], |row| {
	    Ok(Book{
		title: row.get(1)?,
		author: row.get(2)?,
		pages: row.get(3)?,
		genre: row.get(4)?,
		medium: row.get(5)?,
		rating: row.get(6)?,
		notes: row.get(7)?
	    })
	})?;

	Ok(row)
    }
}
