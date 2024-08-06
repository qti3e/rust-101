use std::time::{Duration, Instant};
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    let started = Instant::now();

    tokio::select! {
        a = foo(started) => {
            println!("Foo returned {}", a);
        },
        b = bar(started) => {
            println!("Bar returned {}", b);
        }
    }
}

async fn foo(started: Instant) -> i32 {
    sleep(Duration::from_secs(3)).await;
    println!("Foooo {:?}", started.elapsed());
    3
}

async fn bar(started: Instant) -> i32 {
    sleep(Duration::from_secs(2)).await;
    println!("Barrrrrrr {:?}", started.elapsed());
    5
}
