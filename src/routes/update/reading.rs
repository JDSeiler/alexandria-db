use warp::Filter;
use std::collections::HashMap;

use crate::api::controllers::reading;

const UPDATE_ROOT: &str = "update";
const READING_ROOT: &str = "reading";

pub fn by_id() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path(UPDATE_ROOT)
        .and(warp::path(READING_ROOT))
        .and(warp::put())
	.and(warp::body::content_length_limit(1024 * 4))
	.and(warp::body::json())
	.map(|body: HashMap<String, serde_json::Value>| {
            let body = serde_json::to_string(&body).unwrap();
            reading::update_reading_handler(body)
        })
}
