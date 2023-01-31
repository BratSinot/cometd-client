use cometd_client::types::Event;
use cometd_client::CometdClientBuilder;
use std::time::Duration;

#[tokio::main]
async fn main() {
    let endpoint = "http://[::1]:1025/notifications/".parse().unwrap();
    let client = CometdClientBuilder::new(&endpoint).build().unwrap();
    let mut rx = client.rx();

    client.handshake().await.unwrap();
    let _elapsed = tokio::time::timeout(Duration::from_secs(3), rx.recv())
        .await
        .unwrap_err();

    client.subscribe(&["/topic0", "/topic1"]).await.unwrap();

    for _ in 0..3 {
        let event = rx.recv().await.unwrap();
        println!("response: `{event:?}`.");
    }

    client.disconnect().await.unwrap();
    let _elapsed = tokio::time::timeout(Duration::from_secs(3), async {
        while let Ok(false) = rx
            .recv()
            .await
            .map(|event| matches!(*event, Event::Disconnected))
        {}
    })
    .await
    .unwrap();
}
