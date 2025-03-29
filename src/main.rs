use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::io;

#[derive(Serialize, Deserialize)]
struct UnlockRequest {
    account_id: String,
    items: Vec<String>,
}

#[derive(Serialize, Deserialize)]
struct UnlockResponse {
    success: bool,
    message: String,
}

struct Atomic {
    client: Client,
}

impl Atomic {
    fn new() -> Self {
        let client = Client::new();
        Atomic { client }
    }

    async fn unlock_items(&self, account_id: &str, items: Vec<String>) -> Result<UnlockResponse, Box<dyn Error>> {
        let request = UnlockRequest {
            account_id: account_id.to_string(),
            items,
        };

        let response = self.client.post("https://api.fortnite.com/unlock")
            .json(&request)
            .send()
            .await?;

        let unlock_response: UnlockResponse = response.json().await?;
        Ok(unlock_response)
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let atomic = Atomic::new();
    let mut account_id = String::new();
    let mut items_input = String::new();

    println!("Enter your Fortnite account ID:");
    io::stdin().read_line(&mut account_id)?;
    println!("Enter the items to unlock (comma separated):");
    io::stdin().read_line(&mut items_input)?;

    let items: Vec<String> = items_input.trim().split(',').map(|s| s.trim().to_string()).collect();
    let response = atomic.unlock_items(account_id.trim(), items).await?;

    if response.success {
        println!("Items unlocked successfully: {}", response.message);
    } else {
        println!("Failed to unlock items: {}", response.message);
    }

    Ok(())
}