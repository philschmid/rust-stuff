use std::{thread::sleep, time::Duration};

// Create a multi producer, single consumer channel, which simulates running predictions.
// Check notify https://docs.rs/tokio/1.15.0/tokio/sync/struct.Notify.html or google how this is done
// idea:
// create channel with 100 messages
// create multiple producers
// sendingto 1 consumer (prediction api) -> spawn blocking to run inference
use tokio::{
    sync::{mpsc, oneshot},
    task, time,
};
#[derive(Debug)]
struct Data {
    body: String,
}

#[derive(Debug)]
struct Command {
    message: Data,
    response: oneshot::Sender<u32>,
}

fn compute_function(data: Data) -> u8 {
    // do some compute-heavy work or call synchronous code
    sleep(Duration::from_secs(1));
    let parsed_int = data.body.split("Message ").collect::<Vec<&str>>()[1]
        .parse::<u8>()
        .unwrap();
    parsed_int.pow(2)
}

fn sent_message(message: Data, tx: mpsc::Sender<Command>) -> tokio::task::JoinHandle<()> {
    tokio::spawn(async move {
        let (resp_tx, resp_rx) = oneshot::channel();
        // Send the GET request
        tx.send(Command {
            message,
            response: resp_tx,
        })
        .await
        .unwrap();

        // Await the response
        let res = resp_rx.await;
    })
}

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel(32);

    tokio::spawn(async move {
        for i in 0..10 {
            if let Err(_) = tx
                .send(Data {
                    body: format!("Message {}", i),
                })
                .await
            {
                println!("receiver dropped");
                return;
            }
        }
    });

    let manager = tokio::spawn(async move {
        while let Some(i) = rx.recv().await {
            println!("recieved");
            // https://docs.rs/tokio/0.2.4/tokio/task/index.html#blocking-and-yielding
            let res = task::spawn_blocking(move || {
                // do some compute-heavy work or call synchronous code
                compute_function(i)
            });
            println!("{:?}", res.await.unwrap());
        }
    });
    manager.await.unwrap();
}
