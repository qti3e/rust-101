use std::time::{Duration, Instant};
use tokio::signal::ctrl_c;
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    let started = Instant::now();

    tokio::spawn(async move {
        foo(started).await;
    });

    tokio::spawn(async move {
        bar(started).await;
    });

    ctrl_c().await.unwrap();
}

async fn foo(started: Instant) {
    sleep(Duration::from_secs(3)).await;
    println!("Foooo {:?}", started.elapsed());
}

async fn bar(started: Instant) {
    sleep(Duration::from_secs(2)).await;
    println!("Barrrrrrr {:?}", started.elapsed());
}
