use warp::Filter;

const READ_ROOT: &str = "read";
const BOOK_ROOT: &str = "book";

pub fn by_id() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path(READ_ROOT)
	.and(warp::path(BOOK_ROOT))
	.and(warp::path("id"))
	.and(warp::path::param())
	.map(|id: u32| {
	    format!("Tried to get book with id: {}", id)
	})
}

pub fn by_title() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path(READ_ROOT)
	.and(warp::path(BOOK_ROOT))
	.and(warp::path("title"))
	.and(warp::path::param())
	.map(|title: String| {
	    format!("Tried to get book with title: {}", title)
	})
}

pub fn by_author() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path(READ_ROOT)
	.and(warp::path(BOOK_ROOT))
	.and(warp::path("author"))
	.and(warp::path::param())
	.map(|author: String| {
	    format!("Tried to get book with author: {}", author)
	})
}

