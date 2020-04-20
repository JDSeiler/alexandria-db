use warp::Filter;
use crate::db_api::book_api;

const BOOK_ROOT: &str = "book";

/*
Notes:
impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
seems to be the catch-all return type for Filters. It's all over the warp
docs and the source code.
*/

pub fn all() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path(BOOK_ROOT)
        .and(warp::path("all"))
	.and(warp::get())
        .map(|| format!("Tried to get all books!"))
}

pub fn by_id() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path(BOOK_ROOT)
        .and(warp::path("id"))
        .and(warp::path::param())
	.and(warp::get())
        .map(|id: u32| {
	    book_api::book_by_id_response(id)
	})
}

pub fn by_title() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path(BOOK_ROOT)
        .and(warp::path("title"))
        .and(warp::path::param())
	.and(warp::get())
        .map(|title: String| format!("Tried to get book with title: {}", title))
}

pub fn by_author() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path(BOOK_ROOT)
        .and(warp::path("author"))
        .and(warp::path::param())
	.and(warp::get())
        .map(|author: String| format!("Tried to get book with author: {}", author))
}
