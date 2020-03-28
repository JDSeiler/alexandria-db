use crate::routes::read as read;

pub fn generate_master_route() -> bool {
    read::book::other_function();
    println!("generate_master_route");
    true
}
