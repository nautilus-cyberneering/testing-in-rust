use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use tokio::sync::mpsc;

use testing_in_rust::example02::api::start_server;

#[tokio::test]
async fn it_should_greeting_you() {
    let bind_address = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 3030);

    start_server_and_wait_until_is_ready_to_accept_requests(bind_address).await;

    let url = format!("http://{}/hello/{}", &bind_address, "warp"); // DevSkim: ignore DS137138

    let content = reqwest::get(url).await.unwrap().text().await.unwrap();

    assert_eq!(content, "Hello, warp!");
}

async fn start_server_and_wait_until_is_ready_to_accept_requests(addr: SocketAddr) {
    let (tx, mut rx) = mpsc::channel(100);

    tokio::spawn(async move {
        let started = true;
        tx.send(started).await.unwrap();
        start_server(addr).await;
    });

    while let Some(res) = rx.recv().await {
        if res {
            break;
        }
    }
}
