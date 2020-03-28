use warp::Filter;
use std::collections::HashMap;

const CREATE_ROOT: &str = "create";
const BOOK_ROOT: &str = "book";

pub fn new_book() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path(CREATE_ROOT)
	.and(warp::path(BOOK_ROOT))
	.and(warp::post())
	.and(warp::body::content_length_limit(1024 * 4))
	.and(warp::body::json())
	.map(|body: HashMap<String, String>| {
	    format!("Got POST body of {:#?}", body)
	})
} 
