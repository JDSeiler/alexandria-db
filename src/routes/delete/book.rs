use warp::Filter;
use crate::db_api::book_api;


const BOOK_ROOT: &str = "book";

pub fn by_id() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path(BOOK_ROOT)
        .and(warp::path("id"))
        .and(warp::path::param())
        .and(warp::delete())
        .map(|id: u32| {
	    book_api::delete_book_response(id)
	})
}
