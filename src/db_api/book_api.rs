use serde::{Deserialize, Serialize};
use serde_json::ser;
use warp::http::{Response, StatusCode};

use super::common;

#[derive(Serialize, Deserialize, Debug)]
struct Book {
    id: Option<u32>,
    title: String,
    author: String,
    pages: Option<u32>,
    genre: Option<String>,
    medium: String,
    rating: Option<u32>,
    notes: Option<String>,
}

/*,
TODO: Rename functions that return HTTP responses to "handlers"?
*/

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
                message = format!(
                    "{} rows were changed!! Something may have gone wrong!",
                    changed_rows
                );
            }
            res_builder
                .status(StatusCode::NO_CONTENT)
                .header("RowsChanged", message)
                .body(String::from(""))
                .unwrap()
        }
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

// Keeping this for reference, but I will get rid of this function later once I have
// the response handler
//fn create_book_from_json<'a>(json_string: &'a str) -> Result<Book, serde_json::error::Error> {
//    serde_json::from_str(json_string)?
//}

fn write_book_to_db(book: Book) -> Result<usize, rusqlite::Error> {
    let conn = common::get_database_connection()?;
    let mut stmt = conn.prepare(
        "INSERT INTO book (title, author, pages, genre, medium, rating, notes) 
VALUES (:title, :author, :pages, :genre, :medium, :rating, :notes)",
    )?;
    let params: &[(&str, &dyn rusqlite::ToSql)] = 
        &[
            (":title", &book.title),
            (":author", &book.author),
            (":pages", &book.pages),
            (":genre", &book.genre),
            (":medium", &book.medium),
            (":rating", &book.rating),
            (":notes", &book.notes),
        ];
    stmt.execute_named(params)
}

#[cfg(test)]
mod tests {
    use super::*;

    /*
    struct Book {
        id: Option<u32>,
        title: String,
        author: String,
        pages: Option<u32>,
        genre: Option<String>,
        medium: String,
        rating: Option<u32>,
        notes: Option<String>,
    }
    */
    #[test]
    fn convert_json_to_book() {
        let valid_json: &str = r#"{"id": 1, 
"title": "Test", 
"author": "TestName", 
"pages": 45, 
"genre": "Thriller", 
"medium": "print", 
"rating": null, 
"notes": null}"#;

        let missing_fields_json: &str = r#"{"id": 1,
"title": "BadTest",
"author": "SomeTitle",
"notes": "I left out a bunch of stuff"}"#;

        let bad_types_json: &str = r#"{"id": 1, 
"title": "Test", 
"author": "TestName", 
"pages": 45, 
"genre": "Thriller", 
"medium": "print", 
"rating": "5", 
"notes": ""}"#;

        let should_be_valid: Result<Book, serde_json::error::Error> = serde_json::from_str(valid_json);
        match should_be_valid {
            Ok(book) => println!("'normal' json: {:?}", book),
            Err(e) => panic!("There should not be any errors!"),
        }

        let missing_fields: Result<Book, serde_json::error::Error> = serde_json::from_str(missing_fields_json);
        match missing_fields {
            Ok(book) => panic!(
                "Book with missing fields should not deserialize: {:#?}",
                book
            ),
            Err(e) => println!("This json is missing fields: {:#?}", e),
        }

        let bad_types: Result<Book, serde_json::error::Error> = serde_json::from_str(bad_types_json);
        match bad_types {
            Ok(book) => panic!("Book with bad types should not deserialize: {:#?}", book),
            Err(e) => println!("This json has bad types: {:#?}", e),
        }
    }

    #[test]
    fn query_books_from_db() {
        for id in 1..45 {
            let result = query_book_by_id(id);
            match result {
                Ok(book) => {}
                Err(e) => panic!("A query on the database for id {} failed: {:?}", id, e),
            }
        }
    }


    #[test]
    fn writing_a_book_to_db() {
        let new_book: Book = Book {
            id: None,
            title: "A New Book".to_string(),
            author: "JDSeiler-Test".to_string(),
            pages: Some(45),
            genre: Some("Thriller".to_string()),
            medium: "paper".to_string(),
            rating: Some(5),
            notes: None, 
        };
        let changes = write_book_to_db(new_book);
        match changes {
            Ok(rows) => assert!(rows == 1),
            Err(e) => panic!("Insert failed with error: {:#?}", e)
        }
    }
}
