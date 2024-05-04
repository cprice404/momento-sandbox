use std::error::Error;
use std::time::{Duration, Instant};
use tokio::task::JoinSet;
use tokio::time::sleep;

async fn print_task(id: usize) {
    let start_time = Instant::now();
    while Instant::now() - start_time < Duration::from_secs(10) {
        println!("Thread {}: Hello from Tokio!", id);
        sleep(Duration::from_secs(1)).await;
    }
}

#[tokio::main]
async fn main() {
    let mut join_set = JoinSet::new();
    (0..5).for_each(|i| {
        join_set.spawn(print_task(i));
        ()
    });
    // for h in handles {
    //     h.await?
    // }
    while let Some(res) = join_set.join_next().await {
        match res {
            Ok(_) => println!("Task completed successfully: {:?}", res),
            Err(e) => eprintln!("Task failed: {}", e),
        }
    }
}
