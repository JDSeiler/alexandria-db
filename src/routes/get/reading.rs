use warp::Filter;
use crate::api::controllers::reading;

const READINGS_ROOT: &str = "reading";

/** 

reading#all maps to the path /reading/all

Currently this route always returns 200 with no body for any get
request. But in the future this route is intended to return all rows
from the reading table (all reading objects) in the sqlite
database. This functionality may be revised in the future.

**/
pub fn all() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path(READINGS_ROOT)
        .and(warp::path("all"))
	.and(warp::get())
        .map(|| format!("Tried to get all readings!"))
}

/** 

reading#by_id maps to the path /reading/id/:id where :id is a positive
integer corresponding to a row id in sqlite.

See the documentation for reading_api::reading_by_id_response() for details
on what this route returns.

**/
pub fn by_id() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path(READINGS_ROOT)
        .and(warp::path("id"))
        .and(warp::path::param())
	.and(warp::get())
        .map(|id: u32| {
	   reading::reading_by_id_response(id) 
	})
}

/** 

reading#by_title maps to the path /reading/title/:title where :title
is a string corresponding to the title column of the reading table in
sqlite.

Currently this route always returns 200 with no body for any get
request. But in the future this route is intended to return any rows
from the reading table (reading objects) where the title matches the
supplied parameter.

**/
pub fn by_title() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path(READINGS_ROOT)
        .and(warp::path("title"))
        .and(warp::path::param())
	.and(warp::get())
        .map(|title: String| format!("Tried to get readings for the book with title: {}", title))
}

/** 

reading#by_author maps to the path /reading/title/:author where
:author is a string corresponding to the author column of the reading
table in sqlite.

Currently this route always returns 200 with no body for any get
request. But in the future this route is intended to return any rows
from the reading table (reading objects) where the author matches the
supplied parameter.

**/
pub fn by_author() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path(READINGS_ROOT)
        .and(warp::path("author"))
        .and(warp::path::param())
	.and(warp::get())
        .map(|author: String| format!("Tried to get readings for the author: {}", author))
}
