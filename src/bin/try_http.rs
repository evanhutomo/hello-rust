use reqwest::Result;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Serialize, Deserialize, Debug)]
struct User {
    id: u32,
    name: String,
    email: String,
}

async fn fetch_user(user_id: u32) -> Result<User, Box<dyn Error>> {
    let url = format!("https://jsonplaceholder.typicode.com/users/{}", user_id);
    let response = reqwest::get(&url).await?;

    if response.status().is_success() {
        let user: User = response.json().await?;
        Ok(user)
    } else {
        Err(format!("Failed to fetch user: {}", response.status()).into())
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    println!("Starting async HTTP request...");
    match fetch_user(1).await {
        Ok(user) => println!("User fetched: {:?}", user),
        Err(e) => eprintln!("Error fetching user: {}", e),
    }
    Ok(())
}
