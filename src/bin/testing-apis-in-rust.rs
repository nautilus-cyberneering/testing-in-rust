//! cargo run --bin testing-apis-in-rust
//!
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use testing_in_rust::example02::api::start_server;

#[tokio::main]
async fn main() {
    let bind_address = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 3030);

    start_server(bind_address).await
}
