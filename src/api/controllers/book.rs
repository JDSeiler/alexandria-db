use serde_json::ser;
use warp::http::{Response, StatusCode};

use crate::api::models::book::*;

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
pub fn book_by_id_handler(id: u32) -> Response<String> {
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

This function generates a response for any delete requests to the
/book/id/:id route. This response will be either:

1. A response with HTTP status 204, indicating that either the deletion
   was a success or there was no record by that id to begin with.

2. A response with HTTP status 500 and a body with a string version
   of the exception that caused the problem.

**/

pub fn delete_book_handler(id: u32) -> Response<String> {
    let res_builder = Response::builder();
    let delete_result = delete_book_by_id(id);

    match delete_result {
        Ok(changed_rows) => {
            res_builder
                .status(StatusCode::NO_CONTENT)
                .header("Rows-Changed", changed_rows)
                .body(String::from(""))
                .unwrap()
        }
        Err(error) => res_builder
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(error.to_string())
            .unwrap(),
    }
}

pub fn create_book_handler(payload: String) -> Response<String> {
    let res_builder = Response::builder();
    let maybe_book = serde_json::from_str(payload.as_str());
    match maybe_book {
        Ok(book) => {
            match write_book_to_db(book) {
                Ok(rows_changed) => {
                    res_builder
                        .status(StatusCode::NO_CONTENT)
                        .header("RowsChanged", rows_changed)
                        .body(String::from(""))
                        .unwrap()
                }
                Err(db_err) => {
                    println!("{:#?}", db_err);
                    res_builder
                        .status(StatusCode::INTERNAL_SERVER_ERROR)
                        .body(db_err.to_string())
                        .unwrap()
                }
            }
        }
        Err(payload_err) => {
            res_builder
                .status(StatusCode::BAD_REQUEST)
                .body(payload_err.to_string())
                .unwrap()
        }
    }

}

pub fn update_book_handler(payload: String) -> Response<String> {
    let res_builder = Response::builder();
    let maybe_book = serde_json::from_str(payload.as_str());
    match maybe_book {
        Ok(book) => {
            match update_book_in_db(book) {
                Ok(rows_changed) => {
                    return res_builder
                        .status(StatusCode::NO_CONTENT)
                        .header("Rows-Changed", rows_changed)
                        .body(String::from(""))
                        .unwrap()
                }
                Err(db_err) => {
                    println!("{:#?}", db_err);
                    return res_builder
                        .status(StatusCode::INTERNAL_SERVER_ERROR)
                        .body(db_err.to_string())
                        .unwrap()
                }
            }
        }
        Err(payload_err) => {
            return res_builder
                .status(StatusCode::BAD_REQUEST)
                .body(payload_err.to_string())
                .unwrap()
        }
    }
}
