use warp::Filter;
use crate::db_api::reading_api;

const READINGS_ROOT: &str = "reading";

pub fn all() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path(READINGS_ROOT)
        .and(warp::path("all"))
	.and(warp::get())
        .map(|| format!("Tried to get all readings!"))
}

pub fn by_id() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path(READINGS_ROOT)
        .and(warp::path("id"))
        .and(warp::path::param())
	.and(warp::get())
        .map(|id: u32| {
	   reading_api::reading_by_id_response(id) 
	})
}

pub fn by_title() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path(READINGS_ROOT)
        .and(warp::path("title"))
        .and(warp::path::param())
	.and(warp::get())
        .map(|title: String| format!("Tried to get readings for the book with title: {}", title))
}

pub fn by_author() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path(READINGS_ROOT)
        .and(warp::path("author"))
        .and(warp::path::param())
	.and(warp::get())
        .map(|author: String| format!("Tried to get readings for the author: {}", author))
}
