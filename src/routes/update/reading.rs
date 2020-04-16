use warp::Filter;

const READING_ROOT: &str = "reading";

pub fn by_id() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path(READING_ROOT)
        .and(warp::path("id"))
        .and(warp::path::param())
        .and(warp::put())
        .map(|id: u32| format!("Tried to update reading with id: {}", id))
}
