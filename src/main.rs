use warp::{Filter};
use routes::*;

mod routes;

#[tokio::main]
async fn main() {
    let master_route = routes::master_route::generate_master_route();

    warp::serve(master_route).run(([127,0,0,1], 8080)).await;
}
