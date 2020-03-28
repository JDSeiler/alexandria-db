use warp::Filter;
use std::collections::HashMap;

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
	.map(|body: HashMap<String, String>| {
	    format!("Got POST body of {:#?}", body)
	})
} 
