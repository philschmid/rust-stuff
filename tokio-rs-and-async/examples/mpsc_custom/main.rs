use std::{thread::sleep, time::Duration};

// Create a multi producer, single consumer channel, which simulates running predictions.
// Check notify https://docs.rs/tokio/1.15.0/tokio/sync/struct.Notify.html or google how this is done
// idea:
// create channel with 100 messages
// create multiple producers
// sendingto 1 consumer (prediction api) -> spawn blocking to run inference
use tokio::{sync::mpsc, time};

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel(32);
    let tx2 = tx.clone();

    tokio::spawn(async move {
        tx.send("sending from first handle").await;
        time::sleep(Duration::from_secs(1)).await;
        tx.send("sending from second message from fist handle")
            .await;
    });

    tokio::spawn(async move {
        tx2.send("sending first message from second handle").await;
        time::sleep(Duration::from_secs(1)).await;
        tx2.send("sending from second message from second handle")
            .await;
    });

    while let Some(message) = rx.recv().await {
        println!("GOT = {}", message);
        println!("do things with message");
        sleep(Duration::from_millis(2000));
        println!("Done with = {}", message);
    }
}
