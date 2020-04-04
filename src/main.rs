use warp::{Filter};
use routes::*;
use reqwest;

mod routes;

#[tokio::main]
async fn main() {
    /* 
    TODO:
    1. Write create/reading
    2. Add more endpoints to read/reading
    3. Create delete/ endpoints
    4. Create update/ endpoints
    */
    
    let master_route = routes::master_route::generate_master_route();

    warp::serve(master_route).run(([127,0,0,1], 8080)).await;
}

#[cfg(test)]
mod tests {

    #[test]
    fn tests_active() {
	assert_eq!(2+2, 4);
    }

    #[test]
    #[tokio::main]
    fn test_driver() {
	let master_route = routes::master_route::generate_master_route();
	
	warp::serve(master_route).run(([127,0,0,1], 8080)).await;
    }
}

