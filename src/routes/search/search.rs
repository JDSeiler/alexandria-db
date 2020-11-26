use warp::Filter;
use std::collections::HashMap;

const SEARCH_ROOT: &str = "search";
const BOOK_ROOT: &str = "books";
const READING_ROOT: &str = "readings";

/* 
Some notes:
The .unify makes the type signatures later in the chain nicer

query() is odd, it parses query params into this enigmatic type "T".
T is actually inferred *somehow* from the closure inside map. The type
wizardry happening here is too much for my brain. However, query() is
basically just a wrapper on top of serde_urlencoded::from_str, so any type
that would work there is acceptable for the param in map.
*/
pub fn search_content() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let books_search = warp::path(BOOK_ROOT);
    let readings_search = warp::path(READING_ROOT);
    warp::path(SEARCH_ROOT)
        .and(books_search.or(readings_search).unify())
        .and(warp::get())
        .and(warp::query::query())
        .map(|params: HashMap<String, String>| {
            println!("Params are {:#?}", params);
            format!("You made it to the search endpoint!")
        })
}
