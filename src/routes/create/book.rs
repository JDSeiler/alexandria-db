use warp::Filter;
use serde_json;
use std::collections::HashMap;

use crate::api::controllers::book;

const CREATE_ROOT: &str = "create";
const BOOK_ROOT: &str = "book";

pub fn new_book() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    /* Notes:
    content_length_limit imposes a restriction on the content-length
    HTTP header in bytes (1024 * 4 is about 4kB)

    warp::body::json() causes the filter to pass a HashMap into its
    closure.
    */
    
    warp::path(CREATE_ROOT)
	.and(warp::path(BOOK_ROOT))
	.and(warp::post())
	.and(warp::body::content_length_limit(1024 * 4))
	.and(warp::body::json())
	.map(|body: HashMap<String, serde_json::Value>| {
            // Have to turn the payload back into a string
            // because otherwise I would have to manually
            // parse the HashMap and I don't want to do that.
            // I wasn't aware Warp used serde_json internally
            // to do this when I first wrote this endpoint
            let body = serde_json::to_string(&body).unwrap();
            book::create_book_response(body)
	})
} 
