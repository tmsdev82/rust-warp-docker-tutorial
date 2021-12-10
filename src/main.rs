use std::fs;
use warp::Filter;

#[tokio::main]
async fn main() {
    println!("Starting server...");
    let text_msg = fs::read_to_string("data/data.txt").expect("could not read file");
    let root = warp::path::end().map(move || text_msg.clone());

    let routes = root.with(warp::cors().allow_any_origin());

    warp::serve(routes).run(([0, 0, 0, 0], 5000)).await;
}
