use warp::Filter;
use crate::api::controllers::book;
use std::collections::HashMap;

const BOOK_ROOT: &str = "book";

pub fn by_id() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path(BOOK_ROOT)
        .and(warp::path("id"))
        .and(warp::put())
	.and(warp::body::content_length_limit(1024 * 4))
	.and(warp::body::json())
	.map(|body: HashMap<String, serde_json::Value>| {
            let body = serde_json::to_string(&body).unwrap();
            // return book::update_book_handler(id, body);
            format!("got {}", body)
        })
}
