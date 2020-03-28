//use warp::{Filter};
use routes::*;

mod routes;

#[tokio::main]
async fn main() {
    let master_route = routes::master_route::generate_master_route();
}

