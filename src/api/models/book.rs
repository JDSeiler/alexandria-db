use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

use super::common;
// Macros get exported to the root of the crate
// Odd but this works
use crate::create_struct_with_impl;

create_struct_with_impl! {
    pub struct Book {
        id: Option<u32>,
        title: String,
        author: String,
        pages: Option<u32>,
        genre: Option<String>,
        medium: String,
        rating: Option<u32>,
        notes: Option<String>,
    }
}

/** 
WIP Notes:

This logic can validate that all the keys in a payload are good,
and return a list of bad ones otherwise.

This is essentially step 1 completed, next I need to do type checking on
the values? Maybe I will add another impl to the macro to return a map
of the field names to a serde_json type, so at least the hardcoded map is
contained within the struct definition. 

After type checking I need to dynamically construct a SQL query, then execute it
and handle all the response logic.
**/
pub fn verify_update_payload(id: u32, payload: HashMap<String, Value>) -> String {
    let book_fields: Vec<&'static str> = Book::field_names();
    let mut bad_keys_list = String::new();
    
    for key in payload.keys() {
        let mut key_is_valid = false;
        
        for valid_field in book_fields.iter() {
            if key != valid_field {
                continue
            }
            else {
                key_is_valid = true;
                break;
            }
        }
        
        if key_is_valid {
            continue
        } else {
            let bad_key = format!("{} ", key);
            bad_keys_list.push_str(&bad_key);
        }
    }

    if bad_keys_list.is_empty() {
        String::from("All keys valid")
    } else {
        format!("Bad keys present: {}", bad_keys_list)
    }
}


/**

Given, an integer id, this function attempts to fetch a book record
from the database with that id. Sqlite should enforce that there are
no duplicate ids in the table. However, if that was to somehow occur,
this function would only return the first row that was found.

**/
pub fn query_book_by_id(id: u32) -> Result<Book, rusqlite::Error> {
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

Given, an integer id, this function attempts to delete the book record
from the database with the given id. The function then returns a
`Result` that is either the number of rows changed (on succes), or a
rusqlite:Error (if there is a problem).

**/
pub fn delete_book_by_id(id: u32) -> Result<usize, rusqlite::Error> {
    let conn = common::get_database_connection()?;
    let mut stmt = conn.prepare("DELETE FROM book WHERE id = :id;")?;
    // execute_named returns either Ok(usize) or Err(rusqlite::Error)
    // which is exactly what I want, so it can be returned as is.
    stmt.execute_named(&[(":id", &id)])
}

pub fn write_book_to_db(book: Book) -> Result<usize, rusqlite::Error> {
    let conn = common::get_database_connection()?;
    let mut stmt = conn.prepare(
        "INSERT INTO book (title, author, pages, genre, medium, rating, notes) 
VALUES (:title, :author, :pages, :genre, :medium, :rating, :notes)",
    )?;
    println!("{:#?}", book);
    let params: &[(&str, &dyn rusqlite::ToSql)] = &[
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

        let should_be_valid: Result<Book, serde_json::error::Error> =
            serde_json::from_str(valid_json);
        match should_be_valid {
            Ok(book) => println!("'normal' json: {:?}", book),
            Err(e) => panic!("There should not be any errors!"),
        }

        let missing_fields: Result<Book, serde_json::error::Error> =
            serde_json::from_str(missing_fields_json);
        match missing_fields {
            Ok(book) => panic!(
                "Book with missing fields should not deserialize: {:#?}",
                book
            ),
            Err(e) => println!("This json is missing fields: {:#?}", e),
        }

        let bad_types: Result<Book, serde_json::error::Error> =
            serde_json::from_str(bad_types_json);
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
            Err(e) => panic!("Insert failed with error: {:#?}", e),
        }
    }
}
