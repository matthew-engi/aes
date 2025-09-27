use std::sync::Arc;
use avent::Event;

fn times(d: Arc<u32>) -> Option<u32> {
    Some(*d * 2)
}

#[tokio::main]
async fn main() {
    let mut event1: Event<u32, Option<u32>> = Event::new();
    event1.connect(times);

    println!("{:?}", event1.fire(2).await);
}