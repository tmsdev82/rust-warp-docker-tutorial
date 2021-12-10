use std::env;
use std::fs;
use warp::Filter;

#[tokio::main]
async fn main() {
    println!("Starting server...");
    let instance_name = env::var("INSTANCE_NAME").expect("Error reading environment variable.");
    let text_msg = fs::read_to_string("data/data.txt").expect("could not read file");
    let text_msg = format!("[{}] {}", instance_name, text_msg);
    let root = warp::path::end().map(move || text_msg.clone());

    let routes = root.with(warp::cors().allow_any_origin());

    warp::serve(routes).run(([0, 0, 0, 0], 5000)).await;
}
