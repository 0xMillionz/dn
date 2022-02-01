use pyth_client::{
	load_mapping,
	load_product,
	load_price,
	Price
};
use arc_swap::ArcSwap;

pub struct DnState {
	pub tickers: ArcSwap<Vec<Price>>
}

impl DnState {
	pub fn new() -> &'static DnState {
		let state = DnState {
			tickers: ArcSwap::from_pointee(Vec::new()),
		};

		Box::leak(Box::new(state))
	}
}