use warp::Filter;

const BOOK_ROOT: &str = "book";

pub fn by_id() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path(BOOK_ROOT)
        .and(warp::path("id"))
        .and(warp::path::param())
        .and(warp::put())
        .map(|id: u32| format!("Tried to update book with id: {}", id))
}
