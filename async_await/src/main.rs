use std::time::Duration;
use tokio::time;

async fn say_hello() {
    time::sleep(Duration::from_millis(1000)).await;
    println!("Hello, world!");
}

#[tokio::main]
async fn main() {
    let handle = tokio::spawn(say_hello());
    handle.await.unwrap();
}