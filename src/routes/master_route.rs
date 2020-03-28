use warp::Filter;

use crate::routes::read as read;
use crate::routes::create as create;

pub fn generate_master_route() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    /* CREATE routes */
    let new_book = create::book::new_book();

    let create_routes = new_book;

    
    /* READ (get) routes */
    // For book objects
    let book_by_id = read::book::by_id();
    let book_by_title = read::book::by_title();
    let book_by_author = read::book::by_author();

    let book_routes = book_by_id
	.or(book_by_title)
	.or(book_by_author);

    // For reading objects
    let reading_by_id = read::reading::by_id();

    let read_routes = book_routes.or(reading_by_id);


    /* Final route */
    read_routes.or(create_routes)
}

