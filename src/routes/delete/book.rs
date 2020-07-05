use warp::Filter;
use crate::api::controllers::book;

const BOOK_ROOT: &str = "book";

pub fn by_id() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path(BOOK_ROOT)
        .and(warp::path("id"))
        .and(warp::path::param())
        .and(warp::delete())
        .map(|id: u32| {
	    book::delete_book_response(id)
	})
}
