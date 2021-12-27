use tokio::sync::mpsc;

use async_stream;

use futures_core::stream::Stream;
use futures_util::pin_mut;
use futures_util::stream::StreamExt;

fn create_stream_from_mpsc(mut rx: mpsc::Receiver<i32>) -> impl Stream<Item = i32> {
    async_stream::stream! {
        while let Some(item) = rx.recv().await {
            yield item;
        }
    }
}

// documentation: https://docs.rs/futures/0.3.16/futures/stream/trait.StreamExt.html
async fn read_stream(stream: impl Stream<Item = i32>) {
    pin_mut!(stream);

    while let Some(item) = stream.next().await {
        println!("{:?}", item);
    }

    // let messages = stream.take(3).collect::<Vec<i32>>().await;
    // println!("{:?}", messages);

    // let messages = stream.take(3).collect::<Vec<i32>>().await;

    // loop {
    //     let slice = stream.by_ref().take(3).collect::<Vec<i32>>().await;
    //     println!("{:?}", slice);
    //     if slice.is_empty() {
    //         break;
    //     }
    // }

    // while let items = stream.by_ref().take(3).collect::<Vec<i32>>().await {
    //     println!("{:?}", items);
    // }
}

#[tokio::main]
async fn main() {
    let (tx, rx) = mpsc::channel::<i32>(16);
    let my_stream = create_stream_from_mpsc(rx);
    let rs_ = read_stream(my_stream);

    tokio::spawn(async move {
        for i in 0..10 {
            tx.send(i).await.unwrap();
        }
    });

    rs_.await
}
