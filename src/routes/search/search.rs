use warp::Filter;
use std::collections::HashMap;

const SEARCH_ROOT: &str = "search";
const BOOK_ROOT: &str = "books";
const READING_ROOT: &str = "readings";

/* 
query() is odd, it parses query params into this enigmatic type "T".
T is actually inferred *somehow* from the closure inside map. The type
wizardry happening here is too much for my brain. However, query() is
basically just a wrapper on top of serde_urlencoded::from_str, so any type
that would work there is acceptable for the param in map.
*/
pub fn search_content() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let books_search = warp::path(BOOK_ROOT);
    let readings_search = warp::path(READING_ROOT);

    let book_search_route = warp::path(SEARCH_ROOT)
        .and(warp::get())
        .and(books_search)
        .and(warp::query::query())
        .map(|params: HashMap<String, String>| {
            println!("Params are {:#?}", params);
            format!("You made it to the book search endpoint!")
        });
    let readings_search_route = warp::path(SEARCH_ROOT)
        .and(warp::get())
        .and(readings_search)
        .and(warp::query::query())
        .map(|params: HashMap<String, String>| {
            println!("Params are {:#?}", params);
            format!("You made it to the reading search endpoint!")
        });

    book_search_route.or(readings_search_route)
}
