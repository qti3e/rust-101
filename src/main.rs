use std::{future::Future, time::Duration};

async fn foo() -> i32 {
    println!("Foooooooooooo");
    tokio::time::sleep(Duration::from_secs(3)).await;
    3
}

fn bar() -> impl Future<Output = i32> {
    println!("Barrrrrrrrrrrr");

    async {
        println!("Barr asynccccc");
        3
    }
}

#[tokio::main]
async fn main() {
    let fut1 = foo();
    let fut2 = bar();

    fut1.await;
    fut2.await;
}
