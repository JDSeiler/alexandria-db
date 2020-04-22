use warp::Filter;
use crate::db_api::reading_api;

const READING_ROOT: &str = "reading";

pub fn by_id() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path(READING_ROOT)
        .and(warp::path("id"))
        .and(warp::path::param())
        .and(warp::delete())
        .map(|id: u32| {
	    reading_api::delete_reading_response(id)
	})
}
