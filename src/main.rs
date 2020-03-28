//use warp::{Filter};
use routes::{create, read, update, delete};
    
mod routes;

#[tokio::main]
async fn main() {
    let value = read::book::test();
    read::book::other_function();
}
