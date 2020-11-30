use serde_json::ser;
use warp::http::{Response, StatusCode};

use crate::api::models::reading::*;

pub fn reading_by_id_handler(id: u32) -> Response<String> {
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
                .unwrap(),
        }
    } else {
        let reading = maybe_reading.unwrap();
        let json_str = ser::to_string(&reading).unwrap();
        let response = res_builder.status(StatusCode::OK).body(json_str).unwrap();
        return response;
    }
}

pub fn delete_reading_handler(id: u32) -> Response<String> {
    let res_builder = Response::builder();
    let delete_result = delete_reading_by_id(id);
    println!("{:#?}", delete_result);
    match delete_result {
        Ok(rows_changed) => res_builder
            .status(StatusCode::NO_CONTENT)
            .header("Rows-Changed", rows_changed)
            .body(String::from(""))
            .unwrap(),
        Err(error) => res_builder
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(error.to_string())
            .unwrap(),
    }
}

pub fn create_reading_handler(payload: String) -> Response<String> {
    let res_builder = Response::builder();
    let maybe_reading = serde_json::from_str(payload.as_str());
    match maybe_reading {
        Ok(reading) => match write_reading_to_db(reading) {
            Ok(rows_changed) => res_builder
                .status(StatusCode::NO_CONTENT)
                .header("RowsChanged", rows_changed)
                .body(String::from(""))
                .unwrap(),
            Err(db_err) => {
                println!("{:#?}", db_err);
                res_builder
                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                    .body(db_err.to_string())
                    .unwrap()
            }
        },
        Err(payload_err) => {
            println!("{:#?}", payload_err);
            res_builder
                .status(StatusCode::BAD_REQUEST)
                .body(payload_err.to_string())
                .unwrap()
        }
    }
}

pub fn update_reading_handler(payload: String) -> Response<String> {
    let res_builder = Response::builder();
    let maybe_reading = serde_json::from_str(payload.as_str());
    match maybe_reading {
        Ok(reading) => match update_reading_in_db(reading) {
            Ok(rows_changed) => res_builder
                .status(StatusCode::NO_CONTENT)
                .header("Rows-Changed", rows_changed)
                .body(String::from(""))
                .unwrap(),
            Err(db_err) => {
                println!("{:#?}", db_err);
                res_builder
                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                    .body(db_err.to_string())
                    .unwrap()
            }
        },
        Err(payload_err) => res_builder
            .status(StatusCode::BAD_REQUEST)
            .body(payload_err.to_string())
            .unwrap(),
    }
}
