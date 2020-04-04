use warp::{Filter};

const DELETE_ROOT: &str = "delete";
const BOOK_ROOT: &str  = "book";

pub fn by_id() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone{
    warp::path(DELETE_ROOT)
	.and(warp::path(BOOK_ROOT))
	.and(warp::path::param())
	.and(warp::delete())
	.map(|id: u32| {
	    format!("Tried to delete book with id: {}", id)
	})
}
