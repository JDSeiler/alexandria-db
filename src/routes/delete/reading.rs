use warp::{Filter};

const DELETE_ROOT: &str = "delete";
const READING_ROOT: &str  = "reading";

pub fn by_id() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path(DELETE_ROOT)
	.and(warp::path(READING_ROOT))
	.and(warp::path::param())
	.and(warp::delete())
	.map(|id: u32| {
	    format!("Tried to delete reading with id: {}", id)
	})
}
