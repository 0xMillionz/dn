use tokio::sync::broadcast;

pub async fn run(rx_channel: broadcast::Receiver<String>) {
	tokio::task::spawn(async move {
		println!("Running execution loop!\n");
		loop {
			println!("{:?}", rx_channel.recv().await.unwrap());
		}
	});
}

pub enum Price {
	Market,
	Order(f64),
}

pub struct Order {
	pub ticker: String,
	pub pyth_id: Option<String>,
	pub quantity: f64,
	pub price: Price,
}

pub mod Mango {
	pub async fn place_order(order: super::Order) {
		let str_price = match order.price {
			super::Price::Market => "market".to_string(),
			super::Price::Order(price) => price.to_string(),
		};

		println!(
			"Placing order on MANGO MARKETS\n 
			\tTicker............{:?} 
			\tPyth-Id...........{:?} 
			\tQty...............{:?}
			\tPrice.............{:?}",
			order.ticker,
			order.pyth_id,
			order.quantity,
			str_price,
		);
	}
} 

/// Interactions with drift
pub mod Drift {

}

///
pub mod Hrxo {

}