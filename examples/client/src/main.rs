use cometd_client::CometdClientBuilder;
use serde_json::Value as JsonValue;
use std::time::Duration;

#[tokio::main]
async fn main() {
    let endpoint = "http://[::1]:1025/notifications/".parse().unwrap();
    let client = CometdClientBuilder::new(&endpoint)
        .build::<JsonValue>()
        .unwrap();
    let mut rx = client.rx();

    client.subscribe(&["/topic0", "/topic1"]).await;

    for _ in 0..3 {
        let response = rx.recv().await.unwrap();
        println!("response: `{response:?}`.");
    }

    drop(client);

    tokio::time::sleep(Duration::from_secs(5)).await;
}
