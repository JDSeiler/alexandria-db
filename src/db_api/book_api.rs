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

/**

This function generates a response for any get requests to the
/book/id/:id route. This response will take 1 of 3 forms:

1. If the id matches a book in the database, and the program does
   not produce any errors, the response body is the book record
   in JSON form, with status code 200.

2. If the does not match any book in the database, the response body
   is a simple error message describing that no book was found, with
   status code 404.

3. If the program encounters any other problems, the response body
   is the exception as a string, with status code 500.

**/
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

/** 

Given, an integer id, this function attempts to fetch a book record
from the database with that id. Sqlite should enforce that there are
no duplicate ids in the table. However, if that was to somehow occur,
this function would only return the first row that was found.

**/
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

/** 

This function generates a response for any delete requests to the
/book/id/:id route. This response will be either:

1. A response with HTTP status 204, indicating that either the deletion
   was a success or there was no record by that id to begin with.

2. A response with HTTP status 500 and a body with a string version
   of the exception that caused the problem.

**/

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
		.header("RowsChanged", message)
		.body(String::from(""))
		.unwrap()
	},
        Err(error) => res_builder
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(error.to_string())
            .unwrap(),
    }
}

/** 

Given, an integer id, this function attempts to delete the book record
from the database with the given id. The function then returns a
`Result` that is either the number of rows changed (on succes), or a
rusqlite:Error (if there is a problem).

**/
fn delete_book_by_id(id: u32) -> Result<usize, rusqlite::Error> {
    let conn = common::get_database_connection()?;
    let mut stmt = conn.prepare("DELETE FROM book WHERE id = :id;")?;
    // execute_named returns either Ok(usize) or Err(rusqlite::Error)
    // which is exactly what I want, so it can be returned as is.
    stmt.execute_named(&[(":id", &id)])
}
