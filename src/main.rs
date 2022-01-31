use tokio::sync::broadcast;
use anchor_client;

mod executions;
mod process;

pub struct Event {
	pub event_name: String,
	pub event_data: Option<T>
}

#[tokio::main]
async fn main() {
	let (event_tx, mut event_rx) = broadcast::channel::<String>(16);
	executions::run(&mut event_rx).await;
}