use warp::Filter;

const READ_ROOT: &str = "read";
const READINGS_ROOT: &str = "reading";

pub fn all() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path(READ_ROOT)
	.and(warp::path(READINGS_ROOT))
	.and(warp::path("all"))
	.map(|| {
	    format!("Tried to get all readings!")
	})
}

pub fn by_id() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path(READ_ROOT)
	.and(warp::path(READINGS_ROOT))
	.and(warp::path("id"))
	.and(warp::path::param())
	.map(|id: u32| {
	    format!("Tried to get reading with id: {}", id)
	})
}

pub fn by_title() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path(READ_ROOT)
	.and(warp::path(READINGS_ROOT))
	.and(warp::path("title"))
	.and(warp::path::param())
	.map(|title: String| {
	    format!("Tried to get readings for the book with title: {}", title)
	})
}

pub fn by_author() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path(READ_ROOT)
	.and(warp::path(READINGS_ROOT))
	.and(warp::path("author"))
	.and(warp::path::param())
	.map(|author: String| {
	    format!("Tried to get readings for the author: {}", author)
	})
}

