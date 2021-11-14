use std::time::Duration;
use tokio::time::sleep;

async fn my_sleeper(duration: Duration) {
    // more here
    tokio::time::sleep(duration).await;
    println!("{:?}", duration);
}

#[tokio::main]
async fn main() {
    // single threaded solution
    let s1 = my_sleeper(Duration::from_secs(1));
    let s2 = my_sleeper(Duration::from_secs(3));
    let s3 = my_sleeper(Duration::from_secs(2));
    let start_time = std::time::Instant::now();
    tokio::join!(s1, s2, s3);
    println!("{:?}", start_time.elapsed());

    // multi threaded solution with tokio::spawn
    // this one spawns the async execution potentially on another thread
    let s1 = tokio::spawn(async { my_sleeper(Duration::from_secs(3)).await });
    let s2 = tokio::spawn(async { my_sleeper(Duration::from_secs(1)).await });
    let s3 = tokio::spawn(async { my_sleeper(Duration::from_secs(2)).await });
    let start_time = std::time::Instant::now();
    tokio::join!(s1, s2, s3);
    println!("{:?}", start_time.elapsed());
}
