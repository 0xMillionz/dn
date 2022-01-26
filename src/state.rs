use anchor_client::{
	Client,
	solana_sdk::signature::{Keypair, Signer},
};
use arc_swap::ArcSwap;
use std::fs;

pub struct DnState{
	pub mango_client: ArcSwap<Client>,
	pub payer: dyn Signer,
	// idk add other stuff in here too XD
}

// meh this will probably get bigger
impl DnState{
	pub fn new() -> &'static DnState {
		// Config Settings

		let dn_state = DnState{
			mango_client: ArcSwap::new(Client::new_with_options(cluster: Cluster, payer: Rc<dyn Signer>, options: CommitmentConfig)),
			payer,
		};


		Box::leak(Box::new(dn_state))
	}
}