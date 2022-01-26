
/// Price Scraper Functions
pub mod scrape {
	/// Prices from Mango Markets
	pub mod mango {
		use crate::state::DnState;

		pub async fn sol_perp(state: &'static DnState) -> Result<f64, dyn std::error::Error> {
			unimplemented!();
		}

		pub async fn sol_spot(state: &'static DnState) -> Result<f64, dyn std::error::Error> {
			unimplemented!();
		}
	}	
}