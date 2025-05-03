use tokio::time::{sleep, Duration};

async fn fetch_data(id: u32) -> String {
    println!("Fetching data for ID: {}", id);
    sleep(Duration::from_secs(2)).await; // Simulate a network delay
    format!("Data fetched for ID: {}", id)
}

async fn process_data(data: String) {
    println!("Processing: {}", data);
    sleep(Duration::from_secs(1)).await; // Simulate processing time
    println!("Data processing complete: {}", data);
}

#[tokio::main]
pub async fn main() {
    println!("Starting async operations...");

    // launch two independent async tasks, fetching and processing data
    let task1 = tokio::spawn(async {
        let result1 = fetch_data(1).await;
        process_data(result1).await;
    });

    let task2 = tokio::spawn(async {
        let result2 = fetch_data(2).await;
        process_data(result2).await;
    });

    // Wait for both tasks to complete
    // error handling with try_join
    let _ = tokio::try_join!(task1, task2).map_err(|e| {
        eprintln!("Error occurred: {:?}", e);
    });
    println!("Error handling complete.");
    println!("All tasks completed.");
}
