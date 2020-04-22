use serde::{Deserialize, Serialize};
use serde_json::ser;
use warp::http::{Response, StatusCode};

use super::common;

#[derive(Serialize, Deserialize, Debug)]
struct Book {
    id: u32,
    title: String,
    author: String,
    pages: u32,
    genre: String,
    medium: String,
    rating: u32,
    notes: String,
}

pub fn book_by_id_response(id: u32) -> Response<String> {
    let res_builder = Response::builder();
    let maybe_book = query_book_by_id(id);

    if maybe_book.is_err() {
        let error = maybe_book.unwrap_err();
        match error {
            rusqlite::Error::QueryReturnedNoRows => res_builder
                .status(StatusCode::NOT_FOUND)
                .body(String::from("No book was found with that id"))
                .unwrap(),
            _ => res_builder
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(error.to_string())
                .unwrap(),
        }
    } else {
        let book = maybe_book.unwrap();
        let json_str = ser::to_string(&book).unwrap();
        let response = res_builder.status(StatusCode::OK).body(json_str).unwrap();
        return response;
    }
}

fn query_book_by_id(id: u32) -> Result<Book, rusqlite::Error> {
    let conn = common::get_database_connection()?;
    let mut stmt = conn.prepare("SELECT * FROM book WHERE id = :id;")?;
    let row = stmt.query_row_named(&[(":id", &id)], |row| {
	Ok(Book {
	    id: row.get(0)?,
	    title: row.get(1)?,
	    author: row.get(2)?,
	    pages: row.get(3)?,
	    genre: row.get(4)?,
	    medium: row.get(5)?,
	    rating: row.get(6)?,
	    notes: row.get(7)?,
	})
    })?;
    Ok(row)
}

pub fn delete_book_response(id: u32) -> Response<String> {
    let res_builder = Response::builder();
    let delete_result = delete_book_by_id(id);

    match delete_result {
        Ok(changed_rows) => {
	    let message: String;
	    if changed_rows == 0 {
		message = String::from("No book found with that id, no rows changed");
	    } else if changed_rows == 1 {
		message = String::from("A single book has been deleted");
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

fn delete_book_by_id(id: u32) -> Result<usize, rusqlite::Error> {
    let conn = common::get_database_connection()?;
    let mut stmt = conn.prepare("DELETE FROM book WHERE id = :id;")?;
    // execute_named returns either Ok(usize) or Err(rusqlite::Error)
    // which is exactly what I want, so it can be returned as is.
    stmt.execute_named(&[(":id", &id)])
}
