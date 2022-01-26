use tokio::sync::mpsc;
use anchor_client;

mod executions;

mod state;
use state::DnState;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    // Channel...
    let (tx, mut rx) = mpsc::channel(200);


    // Create state
    let state: &'static DnState = DnState::new();

    // Create dedicated thread is dumb, comprimise and use tokio...
    tokio::spawn(executions::scrape::mango::sol_perp(state));
}