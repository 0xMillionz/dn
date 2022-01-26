use tokio;
use anchor_client;

mod executions;
mod state;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    // Create dedicated thread for 
    std::thread::spawn(move || {
        loop {
            executions::scrape::mango::sol_perp().await;
            
        }
    });
}