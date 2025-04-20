use std::sync::{Arc, Mutex};
use tokio::spawn;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    // create a shared mutable counter
    let counter = Arc::new(Mutex::new(0));

    // create multiple tasks that will increment the counter
    let mut handles = Vec::new();
    for i in 0..5 {
        let counter_clone = Arc::clone(&counter);
        let handle = spawn(async move {
            println!("Task {} is starting", i);
            sleep(Duration::from_millis(rand::random::<u64>() % 1000)).await; // simulate some work
            let mut num = counter_clone.lock().unwrap();
            *num += 1;
            println!("Task {} incremented counter to {}", i, *num);
        });
        handles.push(handle);
    }

    // Wait for all tasks to complete
    for handle in handles {
        handle.await.unwrap();
    }

    // Access the final value of the counter
    let final_count = *counter.lock().unwrap();
    println!("Final counter value: {}", final_count);
    // print a message indicating completion
    println!("All tasks completed.");
}
