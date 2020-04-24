/*! 

# master_route

This file contains the utility functions used to compose all
of the individual routes specified in their own sub-modules.

There is a function per major HTTP verb: POST, GET, PUT, and DELETE.
or create, read, update and delete. Each of these functions combines
some number of smaller routes into a larger one. The
`generate_master_route` function then combines these larger routes
into a final route that is ready to be served in the `main` method
of the server.

These functions contain no real logic to speak of, but more serve as
containers for organizing the different routes and combining them in
a neat fashion.

!*/

use warp::Filter;

use crate::routes::create;
use crate::routes::delete;
use crate::routes::get;
use crate::routes::update;

fn generate_create_routes(
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    /* CREATE routes */
    let new_book = create::book::new_book();
    let new_reading = create::reading::new_reading();

    new_book.or(new_reading)
}

fn generate_get_routes(
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
{
    /* READ (get) routes */
    // For book objects
    let all_books = get::book::all();
    let book_by_id = get::book::by_id();
    let book_by_title = get::book::by_title();
    let book_by_author = get::book::by_author();

    let book_routes = all_books
        .or(book_by_id)
        .or(book_by_title)
        .or(book_by_author);

    // For reading objects
    let all_readings = get::reading::all();
    let reading_by_id = get::reading::by_id();
    let readings_by_title = get::reading::by_title();
    let readings_by_author = get::reading::by_author();

    let reading_routes = all_readings
        .or(reading_by_id)
        .or(readings_by_title)
        .or(readings_by_author);

    book_routes.or(reading_routes)
}

fn generate_update_routes(
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    /* UPDATE routes */
    // For book objects
    let book_by_id = update::book::by_id();
    let book_routes = book_by_id;

    // For reading objects
    let reading_by_id = update::reading::by_id();
    let reading_routes = reading_by_id;

    // The variables book_routes and reading_routes will become useful
    // when there are other endpoints to include. They are redundant for now.

    // All update routes
    book_routes.or(reading_routes)
}

fn generate_delete_routes(
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    /* DELETE routes */
    // For book objects
    let book_by_id = delete::book::by_id();
    let book_routes = book_by_id;

    // For reading objects
    let reading_by_id = delete::reading::by_id();
    let reading_routes = reading_by_id;

    // The variables book_routes and reading_routes will become useful
    // when there are other endpoints to include. They are redundant for now.

    // All delete routes
    book_routes.or(reading_routes)
}

pub fn generate_master_route(
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let create_routes = generate_create_routes();
    let read_routes = generate_get_routes();
    let update_routes = generate_update_routes();
    let delete_routes = generate_delete_routes();

    /* Final route */
    create_routes
        .or(read_routes)
        .or(update_routes)
        .or(delete_routes)
}
