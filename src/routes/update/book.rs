use warp::Filter;
use crate::api::controllers::book;
use std::collections::HashMap;

const UPDATE_ROOT: &str = "update";
const BOOK_ROOT: &str = "book";

pub fn by_id() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path(UPDATE_ROOT)
        .and(warp::path(BOOK_ROOT))
        .and(warp::put())
	.and(warp::body::content_length_limit(1024 * 4))
	.and(warp::body::json())
	.map(|body: HashMap<String, serde_json::Value>| {
            let body = serde_json::to_string(&body).unwrap();
            book::update_book_handler(body)
        })
}
