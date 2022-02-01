use tokio::sync::broadcast;
use anchor_client;

mod executions;
mod process;
mod state;

#[tokio::main]
async fn main() {
	let (event_tx, mut event_rx) = broadcast::channel::<state::DnState>(16);
	executions::run(event_rx).await;
}