use std::net::SocketAddr;

use colored::*;
use warp::Filter;

pub async fn start_server(addr: SocketAddr) {
    // GET /hello/warp => 200 OK with body "Hello, warp!"
    let hello = warp::path!("hello" / String).map(|name| format!("Hello, {}!", name));

    let api_base_url = "http://127.0.0.1:3030/";

    println!(
        "Server running in: {}{}",
        api_base_url.yellow(),
        "hello/warp".yellow()
    );

    warp::serve(hello).run(addr).await
}
