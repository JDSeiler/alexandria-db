use warp::Filter;
use crate::api::controllers::book;

const BOOK_ROOT: &str = "book";

/** 

book#all maps to the path /book/all

Currently this route always returns 200 with no body for any get
request. But in the future this route is intended to return all rows
from the book table (all book objects) in the sqlite database. This
functionality may be revised in the future.

**/
pub fn all() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path(BOOK_ROOT)
        .and(warp::path("all"))
	.and(warp::get())
        .map(|| format!("Tried to get all books!"))
}

/** 

book#by_id maps to the path /book/id/:id where :id is
a positive integer corresponding to a row id in sqlite.

See the documentation for book_api::book_by_id_handler() for details
on what this route returns.

**/
pub fn by_id() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path(BOOK_ROOT)
        .and(warp::path("id"))
        .and(warp::path::param())
	.and(warp::get())
        .map(|id: u32| {
	    book::book_by_id_handler(id)
	})
}

/** 

book#by_title maps to the path /book/title/:title where :title is a
string corresponding to the title column of the book table in sqlite.

Currently this route always returns 200 with no body for any get
request. But in the future this route is intended to return any rows
from the book table (book objects) where the title matches the
supplied parameter.

**/
pub fn by_title() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path(BOOK_ROOT)
        .and(warp::path("title"))
        .and(warp::path::param())
	.and(warp::get())
        .map(|title: String| format!("Tried to get book with title: {}", title))
}
/** 

book#by_author() maps to the path /book/author/:author where :author
is a string corresponding to the author column of the book toble in
sqlite.

Currently this route always returns 200 with no body for any get
request. But in the future this route is intended to return any rows
from the book table (book objects) where the author matches the
supplied parameter.

**/
pub fn by_author() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path(BOOK_ROOT)
        .and(warp::path("author"))
        .and(warp::path::param())
	.and(warp::get())
        .map(|author: String| format!("Tried to get book with author: {}", author))
}
