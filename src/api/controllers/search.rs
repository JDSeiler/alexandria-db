use serde_json::ser;
use std::collections::HashMap;
use warp::http::{Response, StatusCode};

use crate::api::models::book;
use crate::api::models::reading;

enum SearchParam {
    FilterBy,
    Query
}

pub fn search_books_handler(params: HashMap<String, String>) -> Response<String> {
    let res_builder = Response::builder();

    let param_check = params_are_valid(&params);
    if param_check.is_err() {
        let err = param_check.unwrap_err();
        match err {
            SearchParam::FilterBy => res_builder
                .status(StatusCode::UNPROCESSABLE_ENTITY)
                .body("Missing required parameter: filterBy".to_string())
                .unwrap(),
            SearchParam::Query => res_builder
                .status(StatusCode::UNPROCESSABLE_ENTITY)
                .body("Missing require parameter: query".to_string())
                .unwrap()
        }
    } else {
        let filter_col = params.get("filterBy").unwrap().to_owned();
        let filter_query = params.get("query").unwrap().to_owned();
        let results = book::query_books_by_filter(filter_col, filter_query);

        if results.is_err() {
            let err = results.unwrap_err();
            match err {
                rusqlite::Error::InvalidColumnName(col) => res_builder
                    .status(StatusCode::NOT_FOUND)
                    .body(format!("Invalid column name for query: {}", col))
                    .unwrap(),
                _ => res_builder
                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                    .body(err.to_string())
                    .unwrap(),
            }
        } else {
            let encoded_results = ser::to_string(&results.unwrap()).unwrap();
            res_builder
                .status(StatusCode::OK)
                .body(encoded_results)
                .unwrap()
        }
    }
}

pub fn search_readings_handler(params: HashMap<String, String>) -> Response<String> {
    let res_builder = Response::builder();

    let param_check = params_are_valid(&params);
    if param_check.is_err() {
        let err = param_check.unwrap_err();
        match err {
            SearchParam::FilterBy => res_builder
                .status(StatusCode::UNPROCESSABLE_ENTITY)
                .body("Missing required parameter: filterBy".to_string())
                .unwrap(),
            SearchParam::Query => res_builder
                .status(StatusCode::UNPROCESSABLE_ENTITY)
                .body("Missing require parameter: query".to_string())
                .unwrap()
        }
    } else {
        let filter_col = params.get("filterBy").unwrap().to_owned();
        let filter_query = params.get("query").unwrap().to_owned();
        let results = reading::query_readings_by_filter(filter_col, filter_query);

        if results.is_err() {
            let err = results.unwrap_err();
            match err {
                rusqlite::Error::InvalidColumnName(col) => res_builder
                    .status(StatusCode::NOT_FOUND)
                    .body(format!("Invalid column name for query: {}", col))
                    .unwrap(),
                _ => res_builder
                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                    .body(err.to_string())
                    .unwrap(),
            }
        } else {
            let encoded_results = ser::to_string(&results.unwrap()).unwrap();
            res_builder
                .status(StatusCode::OK)
                .body(encoded_results)
                .unwrap()
        }
    }
}

fn params_are_valid(params: &HashMap<String, String>) -> Result<(), SearchParam> {
    if !params.contains_key("filterBy") {
        Err(SearchParam::FilterBy)
    } else if !params.contains_key("query") {
        Err(SearchParam::Query)
    } else {
        Ok(())
    }
}
