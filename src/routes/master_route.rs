use warp::Filter;

use crate::routes::create;
use crate::routes::get;

/*
Notes:
This is the function that will progressively combine all of the routes
defined in sub-modules and turn it into one gigantic mega-route for use
in the main method.
*/

pub fn generate_master_route(
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    /* CREATE routes */
    let new_book = create::book::new_book();
    let new_reading = create::reading::new_reading();

    let create_routes = new_book.or(new_reading);

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

    let read_routes = book_routes.or(reading_routes);

    /* Final route */
    read_routes.or(create_routes)
}
