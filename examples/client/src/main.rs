use cometd_client::{Basic, CometdClientBuilder};
use serde_json::Value as JsonValue;

#[tokio::main]
async fn main() {
    let client = CometdClientBuilder::new()
        .endpoint("http://[::1]:1025/notifications/")
        .build()
        .unwrap();

    client.handshake().await.unwrap();
    client.subscribe(&["/topic0", "/topic1"]).await.unwrap();

    for _ in 0..3 {
        let response = client.connect::<JsonValue>().await;
        println!("response: `{response:?}`.");
    }

    client.disconnect().await.unwrap();
}
