use warp::Filter;

const READ_ROOT: &str = "read";
const READINGS_ROOT: &str = "reading";

pub fn by_id() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path(READ_ROOT)
	.and(warp::path(READINGS_ROOT))
	.and(warp::path("id"))
	.and(warp::path::param())
	.map(|id: u32| {
	    format!("Tried to get reading with id: {}", id)
	})
}
