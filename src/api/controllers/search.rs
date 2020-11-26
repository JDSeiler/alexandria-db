use std::collections::HashMap;
use crate::api::models::common;

pub fn search_books_handler(params: HashMap<String, String>) {
    let conn = common::get_database_connection()?;

}

pub fn search_readings_handler(params: HashMap<String, String>) {
    let conn = common::get_database_connection()()?;
    
}

fn params_are_valid(params: &HashMap<String, String>) -> bool {
    return params.contains_key("filterBy") && params.contains_key("query");
}
